use std::env;

use db::event_dao::EventDao;
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::sync::Arc;

mod db;
mod routes;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub sqlx_pool: Pool<Sqlite>,
    pub event_dao: Arc<EventDao>,
}

pub async fn run() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let sqlx_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Database connection failed");

    let event_dao = EventDao::new(sqlx_pool.clone());

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    let migration_result = Migrator::up(&conn, None).await;
    match migration_result {
        Ok(()) => println!("Migrations successful"),
        Err(e) => println!("Migrations failed: {}", e),
    }

    let state = AppState {
        conn,
        sqlx_pool,
        event_dao: Arc::new(event_dao),
    };

    let app = routes::get_routes(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
