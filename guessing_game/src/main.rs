use std::io::{self, Write};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101); 

    loop {
        let mut guess = String::new();
        print!("Please input your guess: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number :(");
                11
            }
        };

        println!("Your guess: {};", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Correct!");
                break;
            } 
            Ordering::Less => {
                println!("The secret is greater than {}", guess);
            }
            Ordering::Greater => {
                println!("The secret is less than {}", guess);
            }
        }
    }
}