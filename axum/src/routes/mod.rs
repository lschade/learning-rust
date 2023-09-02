mod custom_middleware;
mod hello_world;
mod params;
mod response;
mod set_middleware_custom_header;
mod string_body;
mod teapot;
mod database;
mod sqlx_db;

use axum::{
    body::Body,
    middleware,
    routing::{get, post},
    Router,
};
use custom_middleware::custom_middleware;
use hello_world::hello_world;
use params::path_variable;
use response::return_response;
use set_middleware_custom_header::read_custom_header;
use string_body::json_body;
use string_body::string_body;
use teapot::im_a_teapot;
use database::{create_event, get_events};
use sqlx_db::get_events as get_events_sqlx;

use crate::AppState;

use self::{params::query_params, database::get_event};

pub fn get_routes(state: AppState) -> Router<(), Body> {
    Router::new()
        .route("/", get(hello_world))
        .route("/string", post(string_body))
        .route("/json", post(json_body))
        .route("/path/:id/:number", get(path_variable))
        .route("/query", get(query_params))
        .route("/middleware", get(custom_middleware))
        .route("/teapot", get(im_a_teapot))
        .route("/response", get(return_response))
        .route("/event", post(create_event))
        .route("/event", get(get_events))
        .route("/event-sqlx", get(get_events_sqlx))
        .route("/event/:id", get(get_event))
        .route_layer(middleware::from_fn(read_custom_header))
        .with_state(state)
}

// what is wrong with create_event?
