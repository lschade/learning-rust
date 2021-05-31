fn main() {
    let m = Message::Move { x: 1, y: 2 };
    let m2 = Message::Quit;
    m.print();
    m2.print();


    let x = match m {
        Message::Move { x, y} => x + y,
        Message::Quit => -1,
        Message::Write(s) => -1
    };

    let some_number = Some(5);
    let no_number: Option<u32> = None;

    let number = some_number.expect("None!");
    println!("number: {}", number);
    
    let some_u8_value = Some(0u8);
    if let Some(0) = some_u8_value {
        println!("Zerrrroo");
    }

    let x = Some(10);

    let y = if let Some(val) = x { val } else { -1 };
    println!("{}", y);
}



#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String)
}

impl Message {
    fn print(&self) {
        println!("{:?}", self);
    }
}