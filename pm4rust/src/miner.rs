use itertools::Itertools;
use serde::de::IntoDeserializer;
use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use chrono::{Days, Utc};

use crate::model::{Event, Trace};

pub fn alpha_miner(traces: Vec<Trace>) {
    let activities = get_activities(&traces);
    let start_activities = get_start_activities(&traces);
    let end_activities = get_end_activities(&traces);

    let matrix = get_footprint_matrix(&traces, &activities);

    let choices: Vec<&String> = activities
        .iter()
        .filter(|a| matrix[&(a.to_owned().clone(), a.to_owned().clone())] == Relations::Choice)
        .collect();
}

fn find_sets(
    traces: &Vec<Trace>,
    matrix: &HashMap<(String, String), Relations>,
) -> HashSet<(Vec<String>, Vec<String>)> {
    let activities = get_activities(&traces);

    let mut pairs: HashSet<(Vec<String>, Vec<String>)> = HashSet::new();

    for (i, act) in activities.iter().enumerate() {
        let mut sets = HashSet::new();
        while true {
            let mut to_set = vec![];
            let mut from_set = vec![];

            for a in activities.iter().skip(i + 1) {
                let relation = matrix[&(act.clone(), a.to_owned().clone())];
                if relation == Relations::CausalityLR {
                    if testt(&to_set, a, matrix) {
                        to_set.push(a.clone());
                    }
                }
            }

            if to_set.is_empty() || sets.contains(&to_set) {
                break;
            }

            for a in activities.iter().take(i) {
                if to_set.iter().all(|to| {
                    matrix[&(a.to_owned().clone(), to.to_owned().clone())] == Relations::CausalityLR
                }) {
                    from_set.push(a.clone());
                }
            }
            
            sets.insert(to_set.clone());
            pairs.insert((from_set, to_set));
        }


        let self_relation = matrix[&(act.clone(), act.clone())];
        if self_relation != Relations::Choice {
            continue;
        }

        let mut to_set = vec![];

        
        for a in activities.iter() {
            let relation = matrix[&(act.clone(), a.to_owned().clone())];
            if relation == Relations::CausalityLR {
                if testt(&to_set, a, matrix) {
                    to_set.push(a.clone());
                }
            }
        }

        if to_set.is_empty() {
            continue;
        }

        let from_set: Vec<String> = activities
            .iter()
            .filter(|a| {
                to_set.iter().all(|to| {
                    matrix[&(a.to_owned().clone(), to.to_owned().clone())] == Relations::CausalityLR
                })
            })
            .map(|a| a.clone())
            .collect();

        pairs.insert((from_set, to_set));
    }

    return pairs;
}

fn testt(to_set: &Vec<String>, a: &String, matrix: &HashMap<(String, String), Relations>) -> bool {
    to_set.iter().all(|to| {
        matrix[&(a.to_owned().clone(), to.to_owned().clone())] == Relations::Choice
    })
}

fn get_activities(traces: &Vec<Trace>) -> Vec<String> {
    traces
        .iter()
        .flat_map(|trace| trace.events.iter().map(|event| event.activity.clone()))
        .unique()
        .collect()
}

fn get_start_activities(traces: &Vec<Trace>) -> HashSet<String> {
    traces
        .iter()
        .map(|trace| trace.events.first().unwrap().activity.clone())
        .collect()
}

fn get_end_activities(traces: &Vec<Trace>) -> HashSet<String> {
    traces
        .iter()
        .map(|trace| trace.events.last().unwrap().activity.clone())
        .collect()
}

fn get_footprint_matrix(
    traces: &Vec<Trace>,
    activities: &Vec<String>,
) -> HashMap<(String, String), Relations> {
    let mut matrix: HashMap<(String, String), Relations> = HashMap::new();

    // every combination of activities is a choice
    activities.iter().for_each(|a| {
        activities.iter().for_each(|b| {
            matrix.insert((a.clone(), b.clone()), Relations::Choice);
        })
    });

    traces.iter().for_each(|trace| {
        trace.events.windows(2).for_each(|w| {
            let (a, b) = (w[0].activity.clone(), w[1].activity.clone());
            let key = (a.clone(), b.clone());
            let key_reverse = (b.clone(), a.clone());

            let x = matrix.get(&key).unwrap_or(&Relations::Choice);
            let x = x + &Relations::CausalityLR;
            matrix.insert(key, x);

            let x = matrix.get(&key_reverse).unwrap_or(&Relations::Choice);
            let x = x + &Relations::CausalityRL;
            matrix.insert(key_reverse, x);
        })
    });
    matrix
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Relations {
    CausalityLR,
    CausalityRL,
    Parallel,
    Choice,
}

// implement Add for Relations where Choice + Choice = Choice and CausalityLR + CausalityLR = CaussalityLR and CaussalityRL + CausalityRL = CausalityRL and all other combinations are Parallel
impl Add<&Relations> for &Relations {
    type Output = Relations;

    fn add(self, other: &Relations) -> Relations {
        match (self, other) {
            (Relations::Choice, x) => x.to_owned(),
            (Relations::CausalityLR, Relations::CausalityLR) => Relations::CausalityLR,
            (Relations::CausalityRL, Relations::CausalityRL) => Relations::CausalityRL,
            _ => Relations::Parallel,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::miner::{find_sets, get_activities, get_footprint_matrix, trace, Relations};

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
        assert_eq!(
            matrix[&("A".to_string(), "A".to_string())],
            Relations::Choice
        );
        assert_eq!(
            matrix[&("B".to_string(), "B".to_string())],
            Relations::Choice
        );
        assert_eq!(
            matrix[&("C".to_string(), "C".to_string())],
            Relations::Choice
        );
        assert_eq!(
            matrix[&("D".to_string(), "D".to_string())],
            Relations::Choice
        );
        assert_eq!(
            matrix[&("E".to_string(), "E".to_string())],
            Relations::Choice
        );

        assert_eq!(
            matrix[&("A".to_string(), "B".to_string())],
            Relations::CausalityLR
        );
        assert_eq!(
            matrix[&("A".to_string(), "C".to_string())],
            Relations::CausalityLR
        );
        assert_eq!(
            matrix[&("A".to_string(), "D".to_string())],
            Relations::Choice
        );
        assert_eq!(
            matrix[&("A".to_string(), "E".to_string())],
            Relations::CausalityLR
        );

        assert_eq!(
            matrix[&("B".to_string(), "C".to_string())],
            Relations::Parallel
        );
        assert_eq!(
            matrix[&("B".to_string(), "D".to_string())],
            Relations::CausalityLR
        );
        assert_eq!(
            matrix[&("B".to_string(), "E".to_string())],
            Relations::Choice
        );

        assert_eq!(
            matrix[&("C".to_string(), "D".to_string())],
            Relations::CausalityLR
        );
        assert_eq!(
            matrix[&("C".to_string(), "E".to_string())],
            Relations::Choice
        );

        assert_eq!(
            matrix[&("D".to_string(), "E".to_string())],
            Relations::CausalityRL
        );
    }

    #[test]
    fn test_find_sets() {
        let traces = vec![
            trace(vec!["A", "B", "C", "D"]),
            trace(vec!["A", "C", "B", "D"]),
            trace(vec!["A", "E", "D"]),
        ];

        let matrix = get_footprint_matrix(&traces, &get_activities(&traces));
        let sets = find_sets(&traces, &matrix);

        println!("{:?}", sets);
    }
}

fn trace(activities: Vec<&str>) -> Trace {
    let case_id = uuid::Uuid::new_v4().to_string();
    let events = activities
        .iter()
        .enumerate()
        .map(|(i, activity)| Event {
            case_id: case_id.clone(),
            activity: activity.to_string(),
            timestamp: Utc::now().add(Days::new(i as u64)),
        })
        .collect();
    Trace {
        case_id: case_id,
        events,
    }
}
