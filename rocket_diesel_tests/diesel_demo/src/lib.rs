#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use models::Post;
use std::env;

use crate::models::NewPost;

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
        SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &SqliteConnection, title: &'a str, body: &'a str) {
    use schema::posts;

    let new_post = NewPost {
        title,
        body
    };

    diesel::insert_into(posts::table)
            .values(&new_post)
            .execute(conn).expect("error");
}

#[cfg(test)]
mod tests {
    #[test]
    fn db_works() {
        crate::establish_connection();
    }
}
