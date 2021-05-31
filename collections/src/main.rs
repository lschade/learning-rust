use core::ops::Add;
use std::collections::HashMap;

mod vector_ops;
mod string_ops;

fn main() {
    // let v: Vec<i32> = Vec::new();
    let mut v = Vec::new();
    v.push(2);

    let mut v = vec![1,2,3,4];
    v.push(7);
    println!("{:?}", v);

    let el = v[2];
    println!("{}", el);

    let el = &v[2];
    println!("{}", el);

    let el = v.get(2).expect("No value");
    println!("{}", el);

    let does_not_exist = v.get(100);
    println!("{:?}", does_not_exist);
    // let does_not_exist = &v[100];


    let mut v = vec![1, 2, 3, 4, 5];
    let mut first_copy = v[0];
    first_copy = 10;
    let second_mut = &mut v[1];
    *second_mut = 20;
    // println!("The first element is: {}", first_ref);
    println!("{:?}", v);

    for i in &mut v {
        *i += 1;
        println!("{:?}", i);
    }


    let v = vec![Element::Int(10), Element::Float(10.5), Element::String(String::from("Hello"))];
    for i in &v {
        match i {
            Element::Int(val) => println!("Int: {}", val),
            Element::Float(val) => println!("Float: {}", val),
            Element::String(val) => println!("String: {}", val),
        }
        println!("{:?}", i);
    }

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s3 = s1.add(&s2);
    // let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // println!("{}", s1);
    println!("{}", s3);

    let hello = String::from("Hello");
    let s4 = format!("{} {}", hello, String::from("World"));
    println!("{} - {}", s4, hello);

    // let hello = "Здравствуйте";
    // let answer = &hello[0];

    let value = String::from("value");
    let value2 = String::from("value2");
    let mut map = HashMap::new();
    map.insert("key1", value);
    map.insert("key2", value2);
    // map.insert("key1", 20);

    println!("{}", map.get("key1").expect("not found"));
    // println!("{}", value);

    for (key, value) in &mut map {
        let v = format!("asdasd - {}", key);
        *value = v;
    }

    let x = map.entry("key1").or_insert(String::from("inserted"));
    *x = String::from("s: &str");
    println!("{:?}", x);

    let y = map.get("key1").expect("msg: &str");
    println!("{:?}", y);


    let mut vec = vec![-10, 5, -20, 10, 10, 2, 6, 3];
    let (mean, median, mode) = vector_ops::vector_exercise(&vec);
    println!("mean: {}, median: {}, mode: {} | {:?}", mean, median, mode, vec);
    vec.sort();
    println!("{:?}", vec);


    let pl = string_ops::pig_lating(&String::from("Vectors, strings, and hash maps will provide a large amount of functionality necessary in programs when you need to store, access, and modify data. Here are some exercises you should now be equipped to solve:"));
    println!("{}", pl);
}   

#[derive(Debug)]
enum Element {
    Int(i32),
    Float(f32),
    String(String)
}
