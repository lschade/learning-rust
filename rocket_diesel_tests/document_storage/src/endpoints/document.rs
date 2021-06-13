use document_storage::models::document::{Document, DocumentNew, DocumentUpdate};
use document_storage::models::user::User;

use rocket::Route;
use rocket::serde::json::Json;

use document_storage::database::{ DbConn };
use document_storage::repositories::document_repo;

pub fn routes() -> Vec<Route> {
    routes![get, update, get_all, create]
}

#[get("/")]
async fn get_all(db_conn: DbConn, user: User) -> Result<Json<Vec<Document>>, String> {
    db_conn.run(
        move |c| { 
            document_repo::get_all(user.id, &c)
                            .map(|x| Json(x))
                            .map_err(|e| e.to_string()) 
        }).await
}

#[get("/<id>")]
async fn get(id: i32, db_conn: DbConn) -> Result<Json<Document>, String> {
    db_conn.run(
        move |c| { 
            document_repo::get(id, c)
                            .map(|x| Json(x))
                            .map_err(|e| format!("{}", e))
        }).await
}

#[put("/<id>", data = "<update>")]
async fn update(db_conn: DbConn, id: i32, update: Json<DocumentUpdate>) -> Result<Json<Document>, String> {
    let update = update.0;
    
    db_conn.run(
        move |c| { 
            document_repo::update(id, update, c)
                            .map(|x| Json(x))
                            .map_err(|e| format!("{}", e))
        }).await
}

#[post("/", data = "<new_document>")]
async fn create(new_document: Json<DocumentNew>, db_conn: DbConn) -> Result<Json<Document>, String> {
    let new_document = new_document.0;

    db_conn.run(
        move |c| { 
            document_repo::create(new_document, c)
                            .map(|x| Json(x))
                            .map_err(|e| format!("{}", e))
        }).await
}