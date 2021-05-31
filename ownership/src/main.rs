fn main() {
    string_clone();

    let mut s = String::from("Hello");
    
    // // take_ownership(s); //would take ownership of s;
    // take_ownership(s.clone());
    // println!("{}", s);
    
    // let (s, x) = take_and_give_ownership(s);
    // println!("{}", s);

    // take_ownership_u32(x);
    // println!("{}", x);

    reference(&s);
    reference_mut(&mut s);

    let r1 = &mut s;
    // let r2 = &mut s;
    r1.pop().expect("asd");

    //s still available here; function does not take ownership, because only borrowed
    println!("Result: ");
    println!("{}", s);
}

fn string_borrow() {
    let s1 = String::from("hello");
    let s2 = s1;
    //s1 was moved into s2; s1 was invalidated

    // println!("{}, world!", s1);
}

fn string_clone() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1: {}; s2: {}", s1, s2);


}

fn copy_trait() {
    let tup = (1, "");
    let tup2 = tup;
    println!("{}", tup.1);

    let tup = (1, String::from(""));
    let tup2 = tup.clone();
    println!("{}", tup.1);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn take_and_give_ownership(s: String) -> (String, u32) {
    println!("{}", s);
    return (s, 10);
}

fn take_ownership_u32(x: u32) {
    println!("{}", x);
}

fn reference(s: &String) {
    // s.pop().expect("Err");
    println!("{}", s);
}

fn reference_mut(s: &mut String) {
    s.pop().expect("Err");
    println!("{}", s);
}


// fn dangling_reference() -> &String {
//     let s = String::from("s: &str");
//     return &s;
// }