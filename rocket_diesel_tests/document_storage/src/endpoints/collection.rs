use document_storage::models::collection::{Collection, CollectionNew, CollectionUpdate};
use document_storage::models::user::User;
use rocket::Route;
use rocket::serde::json::Json;

use document_storage::database::{ DbConn };
use document_storage::repositories::collection_repo;

pub fn routes() -> Vec<Route> {
    routes![get, update, get_all, create]
}

#[get("/")]
async fn get_all(db_conn: DbConn, user: User) -> Result<Json<Vec<Collection>>, String> {
    db_conn.run(
        move |c| { 
            collection_repo::get_all(user.id, &c)
                            .map(|x| Json(x))
                            .map_err(|e| e.to_string()) 
        }).await
}

#[get("/<id>")]
async fn get(id: i32, db_conn: DbConn) -> Result<Json<Collection>, String> {
    db_conn.run(
        move |c| { 
            collection_repo::get(id, c)
                            .map(|x| Json(x))
                            .map_err(|e| e.to_string()) 
        }).await
}

#[put("/<id>", data = "<update>")]
async fn update(db_conn: DbConn, id: i32, update: Json<CollectionUpdate>) -> Result<Json<Collection>, String> {
    let update = update.0;
    
    db_conn.run(
        move |c| { 
            collection_repo::update(id, update, c)
                            .map(|x| Json(x))
                            .map_err(|e| e.to_string()) 
        }).await
}

#[post("/", data = "<new_collection>")]
async fn create(new_collection: Json<CollectionNew>, db_conn: DbConn) -> Result<Json<Collection>, String> {
    let new_collection = new_collection.0;

    db_conn.run(
        move |c| { 
            collection_repo::create(new_collection, c)
                            .map(|x| Json(x))
                            .map_err(|e| e.to_string()) 
        }).await
}