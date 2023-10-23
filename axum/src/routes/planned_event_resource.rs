use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json,
};
use serde::Deserialize;

use crate::{
    db::models::{Event, PlannedEvent, PlannedEventEgg},
    AppState,
};

#[derive(Deserialize)]
struct QueryParams {
    project_id: Option<u32>,
}

async fn get_planned_event(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<PlannedEvent>, StatusCode> {
    let event = state.planned_event_dao.get(id).await;
    event
        .map(Json)
        .map(Ok)
        .unwrap_or(Err(StatusCode::NOT_FOUND))
}

async fn list(
    State(state): State<AppState>,
    Query(params): Query<QueryParams>,
) -> Json<Vec<PlannedEvent>> {
    if let Some(project_id) = params.project_id {
        let events = state
            .planned_event_dao
            .list_by_project(project_id)
            .await
            .unwrap();
        return Json(events);
    } else {
        let events = state.planned_event_dao.list().await.unwrap();
        return Json(events);
    }
}

async fn create_planned_event(
    State(state): State<AppState>,
    Json(event): Json<PlannedEventEgg>,
) -> Result<Json<PlannedEvent>, StatusCode> {
    let event = state.planned_event_dao.create(&event).await;
    event.map(Json).map(Ok).unwrap_or_else(|err| {
        eprintln!("Error creating event: {:?}", err);
        return Err(StatusCode::BAD_REQUEST);
    })
}

async fn delete_planned_event(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<StatusCode, StatusCode> {
    let result = state.planned_event_dao.delete(id).await;
    result
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::NOT_FOUND)
}

async fn complete_event(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<Event>, StatusCode> {
    let planned_event = state.planned_event_dao.get(id).await.map_err(|err| {
        eprintln!("Error getting event: {:?}", err);
        match err {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    })?;

    if planned_event.completed {
        eprintln!("Event already completed: {:?}", planned_event);
        return Err(StatusCode::BAD_REQUEST);
    }

    state
        .planned_event_dao
        .complete(planned_event)
        .await
        .map(Json)
        .map_err(|err| {
            eprintln!("Error completing event: {:?}", err);
            match err {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })
}

pub fn get_routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/planned_events/:id", get(get_planned_event))
        .route("/planned_events", get(list))
        .route("/planned_events", post(create_planned_event))
        .route("/planned_events/:id", delete(delete_planned_event))
        .route("/planned_events/:id/complete", post(complete_event))
}
