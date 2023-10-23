use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json,
};

use crate::{
    db::models::{Case, CaseEgg},
    AppState,
};

async fn get_cases(State(state): State<AppState>) -> Json<Vec<Case>> {
    let cases = state.case_dao.list().await.unwrap();
    Json(cases)
}

async fn delete_case(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<String>, StatusCode> {
    let event = state.case_dao.delete(id).await;
    event
        .map(|_| Json(format!("Deleted case with id {}", id)))
        .map(Ok)
        .unwrap_or(Err(StatusCode::NOT_FOUND))
}

async fn create_case(State(state): State<AppState>, Json(case): Json<CaseEgg>) -> Json<Case> {
    let created_case = state.case_dao.create(&case).await.unwrap();
    Json(created_case)
}

pub fn get_routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/cases", get(get_cases))
        .route("/cases", post(create_case))
        .route("/cases/:id", delete(delete_case))
}
