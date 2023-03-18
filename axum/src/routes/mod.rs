mod hello_world;
mod string_body;
mod params;
mod custom_middleware;
mod set_middleware_custom_header;
mod teapot;

use axum::{
    body::Body,
    routing::{get, post},
    Router, middleware,
};
use hello_world::hello_world;
use string_body::string_body;
use string_body::json_body;
use params::path_variable;
use custom_middleware::custom_middleware;
use set_middleware_custom_header::read_custom_header;
use teapot::im_a_teapot;

use self::params::query_params;

pub fn get_routes() -> Router<(), Body> {
    Router::new()
        .route("/", get(hello_world))
        .route("/string", post(string_body))
        .route("/json", post(json_body))
        .route("/path/:id/:number", get(path_variable))
        .route("/query", get(query_params))
        .route("/middleware", get(custom_middleware))
        .route("/teapot", get(im_a_teapot))
        .route_layer(middleware::from_fn(read_custom_header))
}
