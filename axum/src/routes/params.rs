use axum::{extract::{Path, Query}, Json};
use serde::{Serialize, Deserialize};

pub async fn path_variable(Path((id, number)): Path<(i64, i64)>) -> String {
    id.to_string() + "-" + &number.to_string()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryParams {
    param1: String,
    param2: i8,
    param3: Option<i64>
}

pub async fn query_params(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    
    match query.param3 {
        None => println!("param3 is None"),
        Some(param3) => println!("param3 has value: {:?}", param3)
    }

    println!("{:?}", query);
    Json(query)
}