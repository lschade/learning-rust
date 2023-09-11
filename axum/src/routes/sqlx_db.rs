use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    db::models::{Event, EventEgg},
    AppState,
};

pub async fn get_events(State(state): State<AppState>) -> Json<Vec<Event>> {
    let events = state.event_dao.list().await.unwrap();
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
