use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, FromRow, Row};

/* #region Event */

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: i64,
    pub project_id: i64,
    pub case_id: i64,
    pub activity: String,
    pub start_date: i64,
    pub end_date: Option<i64>,
    pub location: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventEgg {
    pub project_id: i64,
    pub case_id: i64,
    pub activity: String,
    pub start_date: i64,
    pub end_date: Option<i64>,
    pub location: Option<String>,
}

impl<'c> FromRow<'c, SqliteRow> for Event {
    fn from_row(row: &SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            project_id: row.try_get("project_id")?,
            case_id: row.try_get("case")?,
            activity: row.try_get("activity")?,
            start_date: row.try_get("start_date")?,
            end_date: row.try_get("end_date")?,
            location: row.try_get("location")?,
        })
    }
}
/* #endregion */

/* #region Case */

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Case {
    pub id: i64,
    pub project_id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CaseEgg {
    pub project_id: i64,
    pub name: String,
    pub description: Option<String>,
}

impl<'c> FromRow<'c, SqliteRow> for Case {
    fn from_row(row: &SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            project_id: row.try_get("project_id")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
        })
    }
}

/* #endregion */

/* #region Planned Case */

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlannedCase {
    pub id: i64,
    pub project_id: i64,
    pub name: String,
    pub description: String,
    pub due_date: Option<i64>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlannedCaseEgg {
    pub project_id: i64,
    pub name: String,
    pub description: String,
    pub due_date: Option<i64>,
}

impl<'c> FromRow<'c, SqliteRow> for PlannedCase {
    fn from_row(row: &SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            project_id: row.try_get("project_id")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            due_date: row.try_get("due_date")?,
        })
    }
}

/* #endregion */

/* #region Planned Event */

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlannedEvent {
    pub id: i64,
    pub project_id: i64,
    pub case_id: i64,
    pub activity: String,
    pub description: Option<String>,
    pub earliest_start_date: i64,
    pub due_date: i64,
    pub completed: bool,
    pub event_id: Option<i64>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlannedEventEgg {
    pub project_id: i64,
    pub activity: String,
    pub description: String,
    pub earliest_start_date: i64,
    pub due_date: i64,
    pub case_id: i64,
}

impl<'c> FromRow<'c, SqliteRow> for PlannedEvent {
    fn from_row(row: &SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            project_id: row.try_get("project_id")?,
            activity: row.try_get("activity")?,
            description: row.try_get("description")?,
            due_date: row.try_get("due_date")?,
            earliest_start_date: row.try_get("earliest_start_date")?,
            completed: row.try_get("completed")?,
            event_id: row.try_get("event_id")?,
            case_id: row.try_get("case_id")?,
        })
    }
}

/* #endregion */

/* #region Event Dependency */

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct EventDependency {
    pub id: i64,
    pub case_id: i64,
    pub dependency_id: i64,
}

/* #endregion */

/* #region Project */

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectEgg {
    pub name: String,
    pub description: String,
}

impl<'c> FromRow<'c, SqliteRow> for Project {
    fn from_row(row: &SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
        })
    }
}

/* #endregion */
