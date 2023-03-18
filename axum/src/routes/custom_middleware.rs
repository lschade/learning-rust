use axum::Extension;
use serde::{Deserialize};

#[derive(Clone, Debug, Deserialize)]
pub struct CustomMiddleWareData {
    pub username: String
}

pub async fn custom_middleware(Extension(message): Extension<CustomMiddleWareData>) -> String {
    message.username.to_owned()
}

