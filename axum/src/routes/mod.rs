mod hello_world;
mod string_body;
mod params;

use axum::{
    body::Body,
    routing::{get, post},
    Router,
};
use hello_world::hello_world;
use string_body::string_body;
use string_body::json_body;
use params::path_variable;

use self::params::query_params;

pub fn get_routes() -> Router<(), Body> {
    Router::new()
        .route("/", get(hello_world))
        .route("/string", post(string_body))
        .route("/json", post(json_body))
        .route("/path/:id/:number", get(path_variable))
        .route("/query", get(query_params))
}
