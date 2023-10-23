use std::env;

use db::{
    case_dao::CaseDao, event_dao::EventDao, planned_event_dao::PlannedEventDao,
    project_dao::ProjectDao,
};
use dotenvy::dotenv;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::sync::Arc;

mod db;
mod routes;

#[derive(Clone)]
pub struct AppState {
    pub sqlx_pool: Pool<Sqlite>,
    pub event_dao: Arc<EventDao>,
    pub case_dao: Arc<CaseDao>,
    pub project_dao: Arc<ProjectDao>,
    pub planned_event_dao: Arc<PlannedEventDao>,
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
    let case_dao = CaseDao::new(sqlx_pool.clone());
    let project_dao = ProjectDao::new(sqlx_pool.clone());
    let planned_event_dao = PlannedEventDao::new(sqlx_pool.clone());

    let state = AppState {
        sqlx_pool,
        event_dao: Arc::new(event_dao),
        case_dao: Arc::new(case_dao),
        project_dao: Arc::new(project_dao),
        planned_event_dao: Arc::new(planned_event_dao),
    };

    let app = routes::get_routes(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
