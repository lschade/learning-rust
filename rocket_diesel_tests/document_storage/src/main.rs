#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod endpoints;

use endpoints::{collection, document, user};
use document_storage::{ database, models::user::User };

#[get("/")]
fn index_user(user_maybe: Option<User>) -> String {
    match user_maybe {
        None => String::from("Hello World!"),
        Some(user) => format!("Hello {} :)", user.username)
    }
}

fn main() {
    rocket::ignite()
        .manage(database::establish_connection())
        .mount("/", routes![index_user])
        .mount("/user", user::routes())
        .mount("/documents", document::routes())
        .mount("/collections", collection::routes())
        .launch();
}

