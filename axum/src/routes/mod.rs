mod custom_middleware;
mod database;
mod hello_world;
mod params;
mod response;
mod set_middleware_custom_header;
mod sqlx_db;
mod string_body;
mod teapot;

use axum::{
    body::Body,
    http::Method,
    middleware,
    routing::{delete, get, post},
    Router,
};
use custom_middleware::custom_middleware;
use database::{create_event, get_events};
use hello_world::hello_world;
use params::path_variable;
use response::return_response;
use set_middleware_custom_header::read_custom_header;
use string_body::json_body;
use string_body::string_body;
use teapot::im_a_teapot;
use tower_http::cors::{Any, CorsLayer};

use crate::AppState;

use self::{database::get_event, params::query_params, sqlx_db::delete_event};

pub fn get_routes(state: AppState) -> Router<(), Body> {
    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(Any);

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
        .route("/event-sqlx", get(sqlx_db::get_events))
        .route("/event-sqlx/:id", get(sqlx_db::get_event))
        .route("/event-sqlx", post(sqlx_db::create_event))
        .route("/event/:id", get(get_event))
        .route("/event-sqlx/:id", delete(delete_event))
        .route("/activities", get(sqlx_db::get_activities))
        .route_layer(middleware::from_fn(read_custom_header))
        .with_state(state)
        .layer(cors)
}
