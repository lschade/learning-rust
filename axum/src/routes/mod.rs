mod case_resource;
mod custom_middleware;
mod event_resource;
mod hello_world;
mod params;
mod planned_event_resource;
mod project_resource;
mod response;
mod set_middleware_custom_header;
mod string_body;
mod teapot;

use axum::{
    body::Body,
    http::Method,
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
use tower_http::cors::{Any, CorsLayer};

use crate::AppState;

use self::params::query_params;

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
        .merge(project_resource::get_routes())
        .merge(case_resource::get_routes())
        .merge(event_resource::get_routes())
        .merge(planned_event_resource::get_routes())
        .route_layer(middleware::from_fn(read_custom_header))
        .with_state(state)
        .layer(cors)
}
