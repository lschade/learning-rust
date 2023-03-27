use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use entity::event::{self};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct EventRequest {
    pub activity: String,
    pub start_date: i64,
    pub end_date: Option<i64>,
    pub location: Option<String>,
}

pub async fn create_event(
    State(state): State<AppState>,
    Json(event): Json<EventRequest>,
) -> Json<String> {
    let event = event::ActiveModel {
        activity: Set(event.activity),
        start_date: Set(event.start_date),
        end_date: Set(event.end_date),
        location: Set(event.location),
        ..Default::default()
    };
    let e = event.save(&state.conn).await.unwrap();
    dbg!(e);
    Json("ok".to_owned())
}

pub async fn get_events(State(state): State<AppState>) -> Json<Vec<event::Model>> {
    let events = event::Entity::find().all(&state.conn).await.unwrap();
    Json(events)
}

pub async fn get_event(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<event::Model>, StatusCode> {
    let event = event::Entity::find_by_id(id)
        .one(&state.conn)
        .await
        .unwrap();
    event
        .map(Json)
        .map(Ok)
        .unwrap_or(Err(StatusCode::NOT_FOUND))
}
