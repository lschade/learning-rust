use diesel::result::Error;

use document_storage::models::user::{User, UserNew, UserUpdate};
use rocket::Route;
use rocket_contrib::json::Json;

use document_storage::database::{ DbConn };
use document_storage::repositories::user_repo;

pub fn routes() -> Vec<Route> {
    routes![get, get_all, update, create]
}

#[get("/")]
fn get_all(db_conn: DbConn) -> Result<Json<Vec<User>>, Error> {
    user_repo::get_all(&db_conn).map(|x| Json(x))
}

#[get("/<id>")]
fn get(id: i32, db_conn: DbConn) -> Result<Json<User>, Error> {
    user_repo::get(id, &db_conn).map(|x| Json(x))
}

#[put("/<id>", data = "<update>")]
fn update(db_conn: DbConn, id: i32, update: Json<UserUpdate>) -> Result<Json<User>, Error> {
    let update = update.0;
    user_repo::update(id, update, &db_conn).map(|x| Json(x))
}

#[post("/", data = "<new_user>")]
fn create(db_conn: DbConn, new_user: Json<UserNew>) -> Result<Json<User>, Error> {
    let new_user = new_user.0;
    user_repo::create(new_user, &db_conn).map(|x| Json(x))
}