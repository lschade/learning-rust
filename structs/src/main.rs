fn main() {
    let user1 = User {
        name: String::from("Username"),
        active: true,
        id: 1
    };

    let user2 = User {
        name: user1.name.clone(),
        id: user1.id + 1,
        ..user1
    };

    println!("User2: {}, {}, {}", user1.id, user1.name, user1.active);
    println!("User2: {}, {}, {}", user2.id, user2.name, user2.active);
}

struct User {
    name: String,
    active: bool,
    id: u32
}
