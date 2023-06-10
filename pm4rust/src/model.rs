use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Event {
    #[serde(rename = "Activity")]
    pub activity: String,
    #[serde(rename = "time:timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "case:concept:name")]
    pub case_id: String,
}

#[derive(Debug, Clone)]
pub struct Trace {
    pub case_id: String,
    pub events: Vec<Event>,
}