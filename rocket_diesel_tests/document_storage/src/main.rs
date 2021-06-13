#[macro_use] 
extern crate rocket;

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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(database::DbConn::fairing())
        .mount("/", routes![index_user])
        .mount("/user", user::routes())
        .mount("/documents", document::routes())
        .mount("/collections", collection::routes())
}

