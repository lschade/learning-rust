use std::fmt;

fn main() {
    let mut rect = Rectangle {
        width: 30,
        height: 50
    };

    let rect_tuple = RectangleTuple(30, 50);

    println!("Area: {}", rect.area());
    println!("Area: {}", rect_tuple.area());

    rect.set_width(20);
    println!("rectanle: {:#?}", rect);
    println!("rectanle tuple: {}", rect_tuple);


    let rect2 = Rectangle {
        width: rect.width - 1,
        height: rect.height - 1
    };

    println!("rect {:?} can hold rect2 {:?}: {}", rect, rect2, rect.can_hold(&rect2));
    println!("square {:?}", Rectangle::square(10));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area2(rectangle: &RectangleTuple) -> u32 {
    rectangle.0 * rectangle.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        self.height >= rect2.height && self.width >= rect2.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}


struct RectangleTuple(u32, u32);

impl fmt::Display for RectangleTuple  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(0: {}, 1: {})", self.0, self.1)
    }
}

impl RectangleTuple {
    fn area(&self) -> u32 {
        println!("Computing area of {}", self);
        self.0 * self.1
    }
}