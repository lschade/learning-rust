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
    let username = headers
        .get("username")
        .to_owned()
        .map(|v| v.to_str().map(|s| s.to_owned()))
        .and_then(|r| r.ok());

    request
        .extensions_mut()
        .insert(CustomMiddleWareData { username: username });

    Ok(next.run(request).await)
}
