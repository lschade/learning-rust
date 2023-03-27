use axum::{response::{Response, IntoResponse}, http::StatusCode};


pub async fn return_response() -> Response {
    return (StatusCode::OK, "ok").into_response()
}
