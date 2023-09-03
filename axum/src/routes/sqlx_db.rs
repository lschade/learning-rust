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
) -> Json<String> {
    let id = state.event_dao.create(&event).await.unwrap();
    Json(format!("Created event with id {}", id))
}
