use std::io::{self, Write};

fn main() {
    // loop {
    //     println("looooop");
    // }

    // loop_assignment();
    // while_loop();
    // for_in();
    // range();

    print!("Input: ");
    io::stdout().flush().unwrap();

    let mut n = String::new();
    io::stdin()
            .read_line(&mut n)
            .expect("Error");
    
    let n: u32 = n.trim().parse().expect("Err");
    let x = fibo2(n);
    println!("{}th Fibonacci number is {}", n, x);

}

fn loop_assignment() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("counter = {}", counter);

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("number = {}", number);
        number -= 1;
    };
}

fn for_in() {
    let a = [1, 2, 3, 4, 5, 10];

    for el in a.iter() {
        println!("{}", el)
    }
}

fn range() {
    for number in (0..5).rev() {
        println!("{}", number);
    }
}

fn fibo(n: u32) -> f64 {
    println!("{} -> {}", 1, 1);

    if n == 1 {
        return 1.0;
    }

    let mut x0 = 0.0;
    let mut x1 = 1.0;

    for i in 2..n + 1 {
        let temp = x1;
        x1 = x0 + x1;
        x0 = temp;

        println!("{} -> {}", i, x1);
    }

    return x1;
}

fn fibo2(n: u32) -> u128 {
    fibo_rec(n - 1, 0, 1)
}

fn fibo_rec(n: u32, x0: u128, x1: u128) -> u128 {
    println!("{} -> {}", n, x1);

    if n == 0 {
        x1
    } else {
        fibo_rec(n - 1, x1, x0 + x1)
    }
}