use std::collections::{HashMap, HashSet};

pub fn get_sets<'a>(
    activities: &'a Vec<String>,
    matrix: &HashMap<(String, String), Relation>,
) -> Vec<(HashSet<&'a String>, HashSet<&'a String>)> {
    let subsets = powerset(&activities);

    let subsets: Vec<Vec<&'a String>> = subsets
        .iter()
        .filter(|s| filter_rel(s.to_vec(), matrix))
        .filter(|s| filter_self_rel(s.to_vec(), matrix))
        .map(|x| x.iter().map(|y| y.clone()).collect())
        .collect();

    let mut eligible: Vec<(HashSet<&'a String>, HashSet<&'a String>)> = vec![];

    for x in subsets.iter() {
        for y in subsets.iter() {
            if filter_set(x, y, matrix) {
                let aa = (
                    HashSet::from_iter(x.iter().map(|e| e.clone())),
                    HashSet::from_iter(y.iter().map(|e| e.clone())),
                );
                eligible.push(aa);
            }
        }
    }

    eligible.retain(|(x, y)| {
        !eligible
            .into_iter()
            .any(|(x2, y2)| x.is_subset(&x2) && y.is_subset(&y2))
    });

    return eligible;
}

fn powerset<'a, T>(s: &'a [T]) -> Vec<Vec<&'a T>>
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

fn filter_self_rel<'a>(s: Vec<&'a String>, matrix: &HashMap<(String, String), Relation>) -> bool {
    s.iter()
        .all(|x| matrix[&(x.clone().clone(), x.clone().clone())] == Relation::Choice)
}

fn filter_rel<'a>(s: Vec<&'a String>, matrix: &HashMap<(String, String), Relation>) -> bool {
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

        let activities = vec![
            "A".to_owned().to_owned(),
            "B".to_owned().to_owned(),
            "C".to_owned().to_owned(),
            "D".to_owned().to_owned(),
            "E".to_owned().to_owned(),
        ];
        let sets = get_sets(&activities, &matrix);

        println!("{:?}", sets);
    }
}
