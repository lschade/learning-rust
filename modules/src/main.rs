mod front_of_house;
use crate::front_of_house::hosting::{ self, add_to_waitlist as waitlist, table };

fn main() {
    front_of_house::hosting::add_to_waitlist();
    self::front_of_house::hosting::add_to_waitlist();
    crate::front_of_house::hosting::add_to_waitlist();

    crate::eat_at_restaurant();

    // hosting::private_fn();

    hosting::add_to_waitlist();
    waitlist();
    table::greet();
}

pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
    println!("Eat");
}