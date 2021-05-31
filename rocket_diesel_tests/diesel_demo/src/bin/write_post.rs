use std::io::{Write, stdin, stdout};

use diesel_demo::{create_post, establish_connection};

fn main() {
    let conn = establish_connection();

    loop {
        println!("Create new post");
        print!("Enter post title: ");
        stdout().flush().unwrap();
        let mut title = String::new();
        stdin().read_line(&mut title).unwrap();
        let title = &title[..(title.len() - 1)];
        
        print!("Enter post body: ");
        stdout().flush().unwrap();
        let mut body = String::new();
        stdin().read_line(&mut body).unwrap();
        let body = &body[..(body.len() - 1)];
    
        let post = create_post(&conn, title, body);
        println!("Created new post: {:?}", post);

        println!("Create another post? (y/n)");
        let mut another = String::new();
        stdin().read_line(&mut another).unwrap();

        if !another.starts_with("y") {
            break;
        }
    }
    
}   