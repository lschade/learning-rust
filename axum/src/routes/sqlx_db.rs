use axum::{extract::State, Json};
use entity::event;

use crate::AppState;

pub async fn get_events(State(state): State<AppState>) -> Json<Vec<event::Model>> {
    sqlx::query!("SELECT * FROM event")
        .fetch_all(&state.sqlx_pool)
        .await
        .map(|rows| {
            rows.into_iter()
                .map(|row| event::Model {
                    id: row.id.try_into().unwrap(),
                    activity: row.activity,
                    start_date: row.start_date,
                    end_date: row.end_date,
                    location: row.location,
                })
                .collect::<Vec<_>>()
        })
        .map_err(|e| {
            println!("Error: {}", e);
            e
        })
        .unwrap()
        .into()
}
