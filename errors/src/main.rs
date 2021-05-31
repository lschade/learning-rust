use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let r = read_username_from_file();
    match r {
        Result::Ok(s) => println!("{}", s),
        Result::Err(err) => println!("Error: {}", err)
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}