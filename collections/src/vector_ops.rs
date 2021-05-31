use std::collections::HashMap;

pub fn vector_exercise(v: &Vec<i32>) -> (f32, i32, i32) {
    let mut map = HashMap::new();
    for x in v {
        let count = map.entry(*x).or_insert(0);
        *count += 1;
    }

    let sum = v.iter().fold(0, |acc, x| acc + x);
    let mean = (sum as f32) / (v.len() as f32);
    
    let mode = map.iter().max_by(|a, b| a.1.cmp(&b.1)).expect("err").0;

    let mut v_sorted = v.clone();
    v_sorted.sort();
    let middle = v_sorted.len() / 2;
    let median = v_sorted[middle];

    return (mean, *mode, median);
}