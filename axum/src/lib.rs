use std::env;

use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};

mod routes;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

pub async fn run() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    let migration_result = Migrator::up(&conn, None).await;
    match migration_result {
        Ok(()) => println!("Migrations successful"),
        Err(e) => println!("Migrations failed: {}", e)
    }

    let state = AppState { conn };

    let app = routes::get_routes(state);


    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
