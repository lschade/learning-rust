use crate::{
    db::models::{Event, EventEgg},
    AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};

pub async fn get_events(State(state): State<AppState>) -> Json<Vec<Event>> {
    let events = state.event_dao.list().await.unwrap();
    Json(events)
}

pub async fn get_events_by_case(
    State(state): State<AppState>,
    Path(case_id): Path<u32>,
) -> Json<Vec<Event>> {
    let events = state.event_dao.list_by_case(case_id).await.unwrap();
    Json(events)
}

pub async fn get_event(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<Event>, StatusCode> {
    let event = state.event_dao.get(id).await;
    event
        .map(Json)
        .map(Ok)
        .unwrap_or(Err(StatusCode::NOT_FOUND))
}

pub async fn create_event(
    State(state): State<AppState>,
    Json(event): Json<EventEgg>,
) -> Json<Event> {
    let created_event = state.event_dao.create(&event).await.unwrap();
    Json(created_event)
}

pub async fn get_activities(State(state): State<AppState>) -> Json<Vec<String>> {
    let activities = state.event_dao.get_activities().await.unwrap();
    Json(activities)
}

pub async fn delete_event(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<String>, StatusCode> {
    let event = state.event_dao.delete(id).await;
    event
        .map(|_| Json(format!("Deleted event with id {}", id)))
        .map(Ok)
        .unwrap_or(Err(StatusCode::NOT_FOUND))
}

pub async fn get_dependencies(
    State(state): State<AppState>,
    Path(event_id): Path<u32>,
) -> Result<Json<Vec<Event>>, StatusCode> {
    let dependencies = state.event_dao.get_dependencies(event_id).await;
    dependencies
        .map(Json)
        .map(Ok)
        .unwrap_or(Err(StatusCode::NOT_FOUND))
}

pub async fn create_dependency(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Path(dependency_id): Path<i64>,
) -> Result<Json<String>, StatusCode> {
    let dependency = state.event_dao.create_dependency(id, dependency_id).await;
    dependency
        .map(|_| {
            Json(format!(
                "Created dependency on {} for event {}",
                dependency_id, id
            ))
        })
        .map(Ok)
        .unwrap_or(Err(StatusCode::NOT_FOUND))
}

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .route("/events", get(get_events))
        .route("/events/:id", get(get_event))
        .route("/events", post(create_event))
        .route("/events/:id", delete(delete_event))
        .route("/activities", get(get_activities))
        .route("/cases/:id/events", get(get_events_by_case))
        .route("/events/:id/dependencies", get(get_dependencies))
        .route(
            "/events/:id/dependencies/:dependency_id",
            post(create_dependency),
        )
}
