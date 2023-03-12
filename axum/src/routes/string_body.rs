use axum::Json;
use serde::{Serialize, Deserialize};

pub async fn string_body(body: String) -> String {
    body
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TestBody {
    content: String
}

pub async fn json_body(body: Json<TestBody>) -> String {
    println!("{:?}", body);
    body.0.content
}