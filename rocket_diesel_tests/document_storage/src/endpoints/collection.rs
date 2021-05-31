use diesel::result::Error;

use document_storage::models::collection::{Collection, CollectionNew, CollectionUpdate};
use document_storage::models::user::User;
use rocket::Route;
use rocket_contrib::json::Json;

use document_storage::database::{ DbConn };
use document_storage::repositories::collection_repo;

pub fn routes() -> Vec<Route> {
    routes![get, update, get_all, create]
}

#[get("/")]
fn get_all(db_conn: DbConn, user: User) -> Result<Json<Vec<Collection>>, Error> {
    collection_repo::get_all(user.id, &db_conn).map(|x| Json(x))
}

#[get("/<id>")]
fn get(id: i32, db_conn: DbConn) -> Result<Json<Collection>, Error> {
    collection_repo::get(id, &db_conn).map(|x| Json(x))
}

#[put("/<id>", data = "<update>")]
fn update(db_conn: DbConn, id: i32, update: Json<CollectionUpdate>) -> Result<Json<Collection>, Error> {
    let update = update.0;
    collection_repo::update(id, update, &db_conn).map(|x| Json(x))
}

#[post("/", data = "<new_collection>")]
fn create(new_collection: Json<CollectionNew>, db_conn: DbConn) -> Result<Json<Collection>, Error> {
    let new_collection = new_collection.0;
    collection_repo::create(new_collection, &db_conn).map(|x| Json(x))
}