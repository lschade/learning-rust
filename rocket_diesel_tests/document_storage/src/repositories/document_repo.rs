use diesel::pg::PgConnection;
use crate::models::document::{ Document, DocumentNew, DocumentUpdate };
use crate::schema::document;
use crate::diesel::{ QueryDsl, RunQueryDsl, ExpressionMethods };

use diesel::QueryResult;

pub fn get(id: i32, db: &PgConnection) -> QueryResult<Document> {
    document::table.find(id).get_result(db)
}

pub fn get_all(user_id: i32, db: &PgConnection) -> QueryResult<Vec<Document>> {
    document::table.filter(document::owner.eq(user_id)).get_results(db)
}

pub fn create(new_document: DocumentNew, db: &PgConnection) -> QueryResult<Document> {
    diesel::insert_into(document::table).values(new_document).get_result(db)
}

pub fn update(id: i32, document_update: DocumentUpdate, db: &PgConnection) -> QueryResult<Document> {
    diesel::update(document::table.find(id)).set(&document_update).get_result(db)
}