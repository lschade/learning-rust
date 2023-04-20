use std::collections::{HashMap, HashSet};

pub fn get_sets(activities: Vec<String>, matrix: &HashMap<(String, String), Relation>) -> Vec<(HashSet<&&String>, HashSet<&&String>)> {
    let subsets = powerset(&activities);

    let subsets: Vec<&Vec<&String>> = subsets
        .iter()
        .filter(|s| filter_rel(s.to_vec(), matrix))
        .filter(|s| filter_self_rel(s.to_vec(), matrix))
        .collect();

    let mut eligible: Vec<(HashSet<&&String>, HashSet<&&String>)> = vec![];
    for x in subsets.iter() {
        for y in subsets.iter() {
            if filter_set(x, y, matrix) {
                eligible.push((HashSet::from_iter(x.iter().clone()), HashSet::from_iter(y.iter().clone())));
            }
        }
    }

    for (i, (x,y)) in eligible.clone().iter().enumerate() {
        for (x2, y2) in eligible.iter() {
            if x.is_subset(x2) && y.is_subset(y2) {
                eligible.remove(i);
                break;
            }
        }
    }

    return eligible;
}

fn powerset<T>(s: &[T]) -> Vec<Vec<&T>>
where
    T: Clone,
{
    (0..2usize.pow(s.len() as u32))
        .map(|i| {
            s.iter()
                .enumerate()
                .filter(|&(t, _)| (i >> t) % 2 == 1)
                .map(|(_, element)| element)
                .collect()
        })
        .collect()
}

fn filter_self_rel(s: Vec<&String>, matrix: &HashMap<(String, String), Relation>) -> bool {
    s.iter()
        .all(|x| matrix[&(x.clone().clone(), x.clone().clone())] == Relation::Choice)
}

fn filter_rel(s: Vec<&String>, matrix: &HashMap<(String, String), Relation>) -> bool {
    s.iter().all(|x| {
        s.iter()
            .all(|b| matrix[&(x.clone().clone(), b.clone().clone())] == Relation::Choice)
    })
}

fn filter_set(
    x: &Vec<&String>,
    y: &Vec<&String>,
    matrix: &HashMap<(String, String), Relation>,
) -> bool {
    x.iter().all(|a| {
        y.iter()
            .all(|b| matrix[&(a.clone().clone(), b.clone().clone())] == Relation::Follows)
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
        let matrix = HashMap::from([
            (("A".to_owned(), "A".to_owned()), Relation::Choice),
            (("B".to_owned(), "B".to_owned()), Relation::Choice),
            (("C".to_owned(), "C".to_owned()), Relation::Choice),
            (("D".to_owned(), "D".to_owned()), Relation::Choice),
            (("E".to_owned(), "E".to_owned()), Relation::Choice),
            (("A".to_owned(), "B".to_owned()), Relation::Follows),
            (("A".to_owned(), "C".to_owned()), Relation::Follows),
            (("A".to_owned(), "D".to_owned()), Relation::Choice),
            (("A".to_owned(), "E".to_owned()), Relation::Follows),
            (("B".to_owned(), "A".to_owned()), Relation::Precedes),
            (("B".to_owned(), "C".to_owned()), Relation::Parallel),
            (("B".to_owned(), "D".to_owned()), Relation::Follows),
            (("B".to_owned(), "E".to_owned()), Relation::Choice),
            (("C".to_owned(), "A".to_owned()), Relation::Precedes),
            (("C".to_owned(), "B".to_owned()), Relation::Parallel),
            (("C".to_owned(), "D".to_owned()), Relation::Follows),
            (("C".to_owned(), "E".to_owned()), Relation::Choice),
            (("D".to_owned(), "A".to_owned()), Relation::Choice),
            (("D".to_owned(), "B".to_owned()), Relation::Precedes),
            (("D".to_owned(), "C".to_owned()), Relation::Precedes),
            (("D".to_owned(), "E".to_owned()), Relation::Precedes),
            (("E".to_owned(), "A".to_owned()), Relation::Precedes),
            (("E".to_owned(), "B".to_owned()), Relation::Choice),
            (("E".to_owned(), "C".to_owned()), Relation::Choice),
            (("E".to_owned(), "D".to_owned()), Relation::Follows),
        ]);

        get_sets(vec!["A".to_owned().to_owned(), "B".to_owned().to_owned(), "C".to_owned().to_owned(), "D".to_owned().to_owned(), "E".to_owned().to_owned()], &     matrix);
    }
}
