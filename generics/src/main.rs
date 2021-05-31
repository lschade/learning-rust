use std::fmt::Display;

fn main() {
    let mut x = Whatever::new(10);

    x.print_it();

    println!("x = {}", x.x);
    x.add(5);
    println!("x = {}", x.x);

    test(&mut x, 5);

    println!("Test: {}", plus(10, 20));

    let array = vec![1,2,10,20,5];
    println!("max of {:?} = {}", array, largest(&array));
    println!("{:?}", array);
}

struct Whatever<T> {
    x: T,
}

impl<T> Whatever<T> {
    fn new(x: T) -> Whatever<T> {
        Whatever { x }
    }
}

trait Addition<T> {
    fn add(&mut self, x: T) -> T;
}

trait Traito {
    fn print_it(&self);
}

impl Addition<u32> for Whatever<u32> {
    fn add(&mut self, x: u32) -> u32 {
        self.x += x;
        self.x
    }
}

fn test<T: Display>(x: &mut impl Addition<T>, a: T) {
    println!("{}", x.add(a));
}

fn plus<T>(a: T, b: T) -> T
where
    T: std::ops::Add + std::ops::Add<Output = T>,
{
    a + b
}

fn largest<T>(arr: &[T]) -> &T 
where T: PartialOrd
{
    let mut max = &arr[0];
    for x in arr {
        if x > max {
            max = x;
        }
    }
    return max;
}


impl<T: Addition<u32>> Traito for T {

    fn print_it(&self) {
        println!("Hellooo");
    }

}