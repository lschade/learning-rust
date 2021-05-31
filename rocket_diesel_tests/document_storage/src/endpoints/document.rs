use diesel::result::Error;

use document_storage::models::document::{Document, DocumentNew, DocumentUpdate};
use document_storage::models::user::User;
use rocket::Route;
use rocket_contrib::json::Json;

use document_storage::database::{ DbConn };
use document_storage::repositories::document_repo;

pub fn routes() -> Vec<Route> {
    routes![get, update, get_all, create]
}

#[get("/")]
fn get_all(db_conn: DbConn, user: User) -> Result<Json<Vec<Document>>, Error> {
    document_repo::get_all(user.id, &db_conn).map(|x| Json(x))
}

#[get("/<id>")]
fn get(id: i32, db_conn: DbConn) -> Result<Json<Document>, Error> {
    document_repo::get(id, &db_conn).map(|x| Json(x))
}

#[put("/<id>", data = "<update>")]
fn update(db_conn: DbConn, id: i32, update: Json<DocumentUpdate>) -> Result<Json<Document>, Error> {
    let update = update.0;
    document_repo::update(id, update, &db_conn).map(|x| Json(x))
}

#[post("/", data = "<new_document>")]
fn create(new_document: Json<DocumentNew>, db_conn: DbConn) -> Result<Json<Document>, Error> {
    let new_document = new_document.0;
    document_repo::create(new_document, &db_conn).map(|x| Json(x))
}