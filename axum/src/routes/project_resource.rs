use crate::{
    db::models::{Project, ProjectEgg},
    AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json,
};

pub async fn get_projects(State(state): State<AppState>) -> Json<Vec<Project>> {
    let projects = state.project_dao.get_projects().await.unwrap();
    Json(projects)
}

pub async fn get_project(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<Project>, StatusCode> {
    let project = state.project_dao.get_project(id).await;
    project
        .map(Json)
        .map(Ok)
        .unwrap_or(Err(StatusCode::NOT_FOUND))
}

pub async fn create_project(
    State(state): State<AppState>,
    Json(project): Json<ProjectEgg>,
) -> Json<Project> {
    let created_project = state.project_dao.create_project(&project).await.unwrap();
    Json(created_project)
}

pub async fn delete_project(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<String>, StatusCode> {
    let project = state.project_dao.delete_project(id).await;
    project
        .map(|_| Json(format!("Deleted project with id {}", id)))
        .map(Ok)
        .unwrap_or(Err(StatusCode::NOT_FOUND))
}

pub async fn update_project(
    State(state): State<AppState>,
    Json(project): Json<Project>,
) -> Json<Project> {
    let updated_project = state.project_dao.update_project(&project).await.unwrap();
    Json(updated_project)
}

pub fn get_routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/projects", get(get_projects))
        .route("/projects/:id", get(get_project))
        .route("/projects", post(create_project))
        .route("/projects/:id", delete(delete_project))
        .route("/projects", put(update_project))
}
