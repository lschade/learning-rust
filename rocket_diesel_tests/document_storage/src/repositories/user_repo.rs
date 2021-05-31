use diesel::pg::PgConnection;
use crate::models::user::{ User, UserNew, UserUpdate };
use crate::schema::app_user;
use crate::diesel::{ QueryDsl, RunQueryDsl };

use diesel::QueryResult;

pub fn get(id: i32, db: &PgConnection) -> QueryResult<User> {
    app_user::table.find(id).get_result(db)
}

pub fn get_all(db: &PgConnection) -> QueryResult<Vec<User>> {
    app_user::table.get_results(db)
}

pub fn create(new_user: UserNew, db: &PgConnection) -> QueryResult<User> {
    diesel::insert_into(app_user::table).values(new_user).get_result(db)
}

pub fn update(id: i32, update_user: UserUpdate, db: &PgConnection) -> QueryResult<User> {
    diesel::update(app_user::table.find(id)).set(&update_user).get_result(db)
}