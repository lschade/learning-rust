pub mod table;

pub fn add_to_waitlist() {
    super::greet();
    println!("Waitlist");
    private_fn();
    table::greet();
}

fn private_fn() {
    println!("Private fn");
}