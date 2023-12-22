use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn return_response() -> Response {
    return (StatusCode::OK, "ok").into_response();
}
