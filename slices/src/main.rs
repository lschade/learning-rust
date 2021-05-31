fn main() {
    let mut s = String::from("H€ello World!");
    println!("{}", &s[0..4]);

    let mut s = String::from("hello world");
    let word = &s[..5];
    // s.clear(); // error!
    println!("the first word is: {}", word);
}

