use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, FromRow, Row};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Event {
    pub id: i64,
    pub activity: String,
    pub start_date: i64,
    pub end_date: Option<i64>,
    pub location: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct EventEgg {
    pub activity: String,
    pub start_date: i64,
    pub end_date: Option<i64>,
    pub location: Option<String>,
}

impl<'c> FromRow<'c, SqliteRow> for Event {
    fn from_row(row: &SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            activity: row.try_get("activity")?,
            start_date: row.try_get("start_date")?,
            end_date: row.try_get("end_date")?,
            location: row.try_get("location")?,
        })
    }
}
