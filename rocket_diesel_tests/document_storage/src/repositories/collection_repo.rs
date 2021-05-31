use diesel::pg::PgConnection;
use crate::models::collection::{ Collection, CollectionNew, CollectionUpdate };
use crate::schema::collection;
use crate::diesel::{ QueryDsl, RunQueryDsl, ExpressionMethods };

use diesel::QueryResult;

pub fn get(id: i32, db: &PgConnection) -> QueryResult<Collection> {
    collection::table.find(id).get_result(db)
}

pub fn get_all(user_id: i32, db: &PgConnection) -> QueryResult<Vec<Collection>> {
    collection::table.filter(collection::owner.eq(user_id)).get_results(db)
}

pub fn create(new_collection: CollectionNew, db: &PgConnection) -> QueryResult<Collection> {
    diesel::insert_into(collection::table).values(new_collection).get_result(db)
}

pub fn update(id: i32, collection_update: CollectionUpdate, db: &PgConnection) -> QueryResult<Collection> {
    diesel::update(collection::table.find(id)).set(&collection_update).get_result(db)
}