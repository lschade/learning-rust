use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use super::custom_middleware::CustomMiddleWareData;

pub async fn read_custom_header<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let message = headers
        .get("username")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?
        .to_owned();
    let username = message.to_str().map_err(|_err| StatusCode::BAD_REQUEST)?;

    request.extensions_mut().insert(CustomMiddleWareData {
        username: username.to_owned(),
    });

    Ok(next.run(request).await)
}
