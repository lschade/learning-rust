use std::{collections::{HashMap, HashSet}, ops::Add};

use chrono::{Days, Utc};
use itertools::Itertools;
use netcrab::petri_net::{PetriNet, TransitionRef};

use crate::model::{Trace, Event};


pub fn apply(traces: &Vec<Trace>) -> PetriNet {
  let activities = get_activities(traces);
  let matrix = get_footprint_matrix(traces, &activities);
  let sets = get_sets(&activities, &matrix);

  let start_activities = get_start_activities(traces);
  let end_activities = get_end_activities(traces);

  let mut net = PetriNet::new();

  let transitions: HashMap<&str, TransitionRef> = activities
      .iter()
      .map(|a| (*a, net.add_transition(a)))
      .collect();

  let mut i = 0;
  start_activities.iter().for_each(|a| {
      let p_label = format!("p_s_{}", i);
      i += 1;

      let place = net.add_place(p_label.as_str());
      net.add_token(&place, 1).unwrap();

      let transition = &transitions[a];
      let result = net.add_arc_place_transition(&place, transition);
      assert!(result.is_ok());
  });

  let mut i = 0;
  end_activities.iter().for_each(|a| {
      let p_label = format!("p_e_{}", i);
      i += 1;

      let place = net.add_place(p_label.as_str());
      let transition = &transitions[a];
      let result = net.add_arc_transition_place(transition, &place);
      assert!(result.is_ok());
  });

  let mut i = 0;
  sets.iter().for_each(|(x, y)| {
      let p_label = format!("p_{}", i);
      let place: netcrab::petri_net::PlaceRef = net.add_place(p_label.as_str());
      i += 1;

      x.iter().for_each(|a| {
          let transition = &transitions[a];
          let result = net.add_arc_transition_place(transition, &place);
          assert!(result.is_ok());
      });

      y.iter().for_each(|a| {
          let transition = &transitions[a];
          let result = net.add_arc_place_transition(&place, transition);
          assert!(result.is_ok());
      });
  });

  net
}

pub fn get_sets<'a>(
  activities: &Vec<&'a str>,
  matrix: &HashMap<(&str, &str), Relation>,
) -> Vec<(HashSet<&'a str>, HashSet<&'a str>)> {
  let subsets = powerset(&activities);
  let subsets: Vec<&Vec<&str>> = subsets
      .iter()
      .filter(|s| filter_rel(s.to_vec(), &matrix))
      .filter(|s| filter_self_rel(s.to_vec(), &matrix))
      .filter(|s| !s.is_empty())
      .collect();

  let mut eligible: Vec<(HashSet<&str>, HashSet<&str>)> = vec![];
  for x in subsets.iter() {
      for y in subsets.iter() {
          if filter_set(x, y, &matrix) {
              let t1 = HashSet::from_iter(x.iter().map(|e| e.clone()));
              let t2 = HashSet::from_iter(y.iter().map(|e| e.clone()));
              eligible.push((t1, t2));
          }
      }
  }

  let eligible: Vec<(HashSet<&str>, HashSet<&str>)> = eligible
      .iter()
      .filter(|(x, y)| {
          !eligible
              .iter()
              .filter(|(a, b)| (a, b) != (x, y))
              .any(|(x2, y2)| x.is_subset(x2) && y.is_subset(y2))
      })
      .map(|e| e.clone())
      .collect();

  eligible
}

fn powerset<T>(s: &[T]) -> Vec<Vec<T>>
where
  T: Clone,
{
  (0..2usize.pow(s.len() as u32))
      .map(|i| {
          s.iter()
              .enumerate()
              .filter(|&(t, _)| (i >> t) % 2 == 1)
              .map(|(_, element)| element.clone())
              .collect()
      })
      .collect()
}

fn filter_self_rel(s: Vec<&str>, matrix: &HashMap<(&str, &str), Relation>) -> bool {
  s.iter().all(|x| matrix[&(*x, *x)] == Relation::Choice)
}

fn filter_rel(s: Vec<&str>, matrix: &HashMap<(&str, &str), Relation>) -> bool {
  s.iter()
      .all(|x| s.iter().all(|b| matrix[&(*x, *b)] == Relation::Choice))
}

fn filter_set(x: &&Vec<&str>, y: &&Vec<&str>, matrix: &HashMap<(&str, &str), Relation>) -> bool {
  x.iter()
      .all(|a| y.iter().all(|b| matrix[&(*a, *b)] == Relation::Follows))
}

fn get_footprint_matrix<'a>(
  traces: &'a Vec<Trace>,
  activities: &Vec<&'a str>,
) -> HashMap<(&'a str, &'a str), Relation> {
  let mut matrix: HashMap<(&str, &str), Relation> = HashMap::new();

  // every combination of activities is a choice
  activities.iter().for_each(|a| {
      activities.iter().for_each(|b| {
          matrix.insert((a, b), Relation::Choice);
      })
  });

  traces.iter().for_each(|trace| {
      trace.events.windows(2).for_each(|w| {
          let (a, b) = (w[0].activity.as_str(), w[1].activity.as_str());
          let key = (a, b);
          let key_reverse = (b.clone(), a.clone());

          let x = matrix.get(&key).unwrap_or(&Relation::Choice);
          let x = x + &Relation::Follows;
          matrix.insert(key, x);

          let x = matrix.get(&key_reverse).unwrap_or(&Relation::Choice);
          let x = x + &Relation::Precedes;
          matrix.insert(key_reverse, x);
      })
  });
  matrix
}

fn get_activities(traces: &Vec<Trace>) -> Vec<&str> {
  traces
      .iter()
      .flat_map(|trace| trace.events.iter().map(|event| event.activity.as_str()))
      .unique()
      .collect()
}

fn get_start_activities(traces: &Vec<Trace>) -> HashSet<&str> {
  traces
      .iter()
      .map(|trace| trace.events.first().unwrap().activity.as_str())
      .collect()
}

fn get_end_activities(traces: &Vec<Trace>) -> HashSet<&str> {
  traces
      .iter()
      .map(|trace| trace.events.last().unwrap().activity.as_str())
      .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Relation {
  Choice,
  Follows,
  Precedes,
  Parallel,
}

// implement Add for Relation where Choice + Choice = Choice and Follows + Follows = CaussalityLR and CaussalityRL + CausalityRL = CausalityRL and all other combinations are Parallel
impl Add<&Relation> for &Relation {
  type Output = Relation;

  fn add(self, other: &Relation) -> Relation {
      match (self, other) {
          (Relation::Choice, x) => x.to_owned(),
          (Relation::Follows, Relation::Follows) => Relation::Follows,
          (Relation::Precedes, Relation::Precedes) => Relation::Precedes,
          _ => Relation::Parallel,
      }
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use super::*;

  #[test]
  fn it_works() {
      let activities: Vec<&str> = vec!["A", "B", "C", "D", "E"];
      let matrix = HashMap::from([
          (("A", "A"), Relation::Choice),
          (("B", "B"), Relation::Choice),
          (("C", "C"), Relation::Choice),
          (("D", "D"), Relation::Choice),
          (("E", "E"), Relation::Choice),
          (("A", "B"), Relation::Follows),
          (("A", "C"), Relation::Follows),
          (("A", "D"), Relation::Choice),
          (("A", "E"), Relation::Follows),
          (("B", "A"), Relation::Precedes),
          (("B", "C"), Relation::Parallel),
          (("B", "D"), Relation::Follows),
          (("B", "E"), Relation::Choice),
          (("C", "A"), Relation::Precedes),
          (("C", "B"), Relation::Parallel),
          (("C", "D"), Relation::Follows),
          (("C", "E"), Relation::Choice),
          (("D", "A"), Relation::Choice),
          (("D", "B"), Relation::Precedes),
          (("D", "C"), Relation::Precedes),
          (("D", "E"), Relation::Precedes),
          (("E", "A"), Relation::Precedes),
          (("E", "B"), Relation::Choice),
          (("E", "C"), Relation::Choice),
          (("E", "D"), Relation::Follows),
      ]);

      let sets = get_sets(&activities, &matrix);

      println!("{:?}", sets)
  }

  #[test]
  fn test_footprint_matrix() {
      let traces = vec![
          trace(vec!["A", "B", "C", "D"]),
          trace(vec!["A", "C", "B", "D"]),
          trace(vec!["A", "E", "D"]),
      ];

      let matrix = get_footprint_matrix(&traces, &get_activities(&traces));

      println!("{:?}", matrix);

      assert_eq!(matrix.len(), 5 * 5);
      assert_eq!(matrix[&("A", "A")], Relation::Choice);
      assert_eq!(matrix[&("B", "B")], Relation::Choice);
      assert_eq!(matrix[&("C", "C")], Relation::Choice);
      assert_eq!(matrix[&("D", "D")], Relation::Choice);
      assert_eq!(matrix[&("E", "E")], Relation::Choice);

      assert_eq!(matrix[&("A", "B")], Relation::Follows);
      assert_eq!(matrix[&("A", "C")], Relation::Follows);
      assert_eq!(matrix[&("A", "D")], Relation::Choice);
      assert_eq!(matrix[&("A", "E")], Relation::Follows);

      assert_eq!(matrix[&("B", "C")], Relation::Parallel);
      assert_eq!(matrix[&("B", "D")], Relation::Follows);
      assert_eq!(matrix[&("B", "E")], Relation::Choice);

      assert_eq!(matrix[&("C", "D")], Relation::Follows);
      assert_eq!(matrix[&("C", "E")], Relation::Choice);

      assert_eq!(matrix[&("D", "E")], Relation::Precedes);
  }

  #[test]
  fn test_find_sets() {
      let traces = vec![
          trace(vec!["A", "B", "C", "D"]),
          trace(vec!["A", "C", "B", "D"]),
          trace(vec!["A", "E", "D"]),
      ];

      let activities = get_activities(&traces);
      let matrix = get_footprint_matrix(&traces, &activities);
      let sets = get_sets(&activities, &matrix);

      assert!(sets.len() == 4);
      assert!(sets.contains(&(
          HashSet::from_iter(vec!["A"]),
          HashSet::from_iter(vec!["B", "E"])
      )));
      assert!(sets.contains(&(
          HashSet::from_iter(vec!["A"]),
          HashSet::from_iter(vec!["C", "E"])
      )));
      assert!(sets.contains(&(
          HashSet::from_iter(vec!["E", "B"]),
          HashSet::from_iter(vec!["D"])
      )));
      assert!(sets.contains(&(
          HashSet::from_iter(vec!["E", "C"]),
          HashSet::from_iter(vec!["D"])
      )));

      println!("{:?}", sets);
  }

  #[test]
  fn test_apply() {
      let traces = vec![
          trace(vec!["A", "B", "C", "D"]),
          trace(vec!["A", "C", "B", "D"]),
          trace(vec!["A", "E", "D"]),
      ];

      let net = apply(&traces);

      let dot_string = net.to_dot_string();
      println!("{}", dot_string.unwrap());
  }
}

fn trace(activities: Vec<&str>) -> Trace {
  let case_id = uuid::Uuid::new_v4();
  let events = activities
      .iter()
      .enumerate()
      .map(|(i, activity)| Event {
          case_id: case_id.to_string(),
          activity: activity.to_string(),
          timestamp: Utc::now().add(Days::new(i as u64)),
      })
      .collect();
  Trace {
      case_id: case_id.to_string(),
      events,
  }
}
