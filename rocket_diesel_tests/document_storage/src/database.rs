use rocket_sync_db_pools::{diesel, database};

#[database("diesel_demo")]
pub struct DbConn(diesel::PgConnection);
