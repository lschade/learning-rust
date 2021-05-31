use add_one::add_one;
use add_two::add_two;

fn main() {
    let x = add_one(10);

    println!("Add one {} + 1 = {}", x, add_one(x));
    println!("Add two {} + 2 = {}", x, add_two(x));
}
