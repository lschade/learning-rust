use std::collections::{HashMap, HashSet};

pub fn get_sets(
    activities: Vec<String>,
    matrix: &HashMap<(&str, &str), Relation>,
) -> Vec<(HashSet<String>, HashSet<String>)> {
    let subsets = powerset(&activities);
    let subsets: Vec<&Vec<String>> = subsets
        .iter()
        .filter(|s| filter_rel(s.to_vec(), &matrix))
        .filter(|s| filter_self_rel(s.to_vec(), &matrix))
        .filter(|s| !s.is_empty())
        .collect();

    let mut eligible: Vec<(HashSet<String>, HashSet<String>)> = vec![];
    for x in subsets.iter() {
        for y in subsets.iter() {
            if filter_set(x, y, &matrix) {
                let t1 = HashSet::from_iter(x.iter().map(|e| e.clone()));
                let t2 = HashSet::from_iter(y.iter().map(|e| e.clone()));
                eligible.push((t1, t2));
            }
        }
    }

    let eligible: Vec<(HashSet<String>, HashSet<String>)> = eligible
        .iter()
        .filter(|(x, y)| {
            !eligible
                .iter()
                .filter(|(a,b)| (a,b) != (x,y))
                .any(|(x2, y2)| x.is_subset(x2) && y.is_subset(y2))
        })
        .map(|e| e.clone())
        .collect();

    return eligible;
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

fn filter_self_rel(s: Vec<String>, matrix: &HashMap<(&str, &str), Relation>) -> bool {
    s.iter()
        .all(|x| matrix[&(x.as_str(), x.as_str())] == Relation::Choice)
}

fn filter_rel(s: Vec<String>, matrix: &HashMap<(&str, &str), Relation>) -> bool {
    s.iter().all(|x| {
        s.iter()
            .all(|b| matrix[&(x.as_str(), b.as_str())] == Relation::Choice)
    })
}

fn filter_set(
    x: &&Vec<String>,
    y: &&Vec<String>,
    matrix: &HashMap<(&str, &str), Relation>,
) -> bool {
    x.iter().all(|a| {
        y.iter()
            .all(|b| matrix[&(a.as_str(), b.as_str())] == Relation::Follows)
    })
}

#[derive(PartialEq, Eq)]
pub enum Relation {
    Choice,
    Follows,
    Precedes,
    Parallel,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn it_works() {
        let activities = vec![
            "A".to_owned(),
            "B".to_owned(),
            "C".to_owned(),
            "D".to_owned(),
            "E".to_owned(),
        ];
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

        let sets = get_sets(activities, &matrix);

        println!("{:?}", sets)
    }
}
