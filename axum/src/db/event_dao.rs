use super::models::{Event, EventEgg};

pub struct EventDao {
    pub pool: sqlx::SqlitePool,
}

impl EventDao {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<Event>, sqlx::Error> {
        sqlx::query_as!(Event, "SELECT * FROM event")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn list_by_case(&self, case_id: u32) -> Result<Vec<Event>, sqlx::Error> {
        sqlx::query_as!(Event, "SELECT * FROM event WHERE `case_id` = ?", case_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get(&self, id: u32) -> Result<Event, sqlx::Error> {
        sqlx::query_as!(Event, "SELECT * FROM event WHERE `id` = ?", id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn create(&self, event: &EventEgg) -> Result<Event, sqlx::Error> {
        let result = sqlx::query_as!(Event,
            "INSERT INTO event (project_id, case_id, activity, start_date, end_date, location) VALUES (?, ?, ?, ?, ?, ?) RETURNING *",
            event.project_id,
            event.case_id,
            event.activity,
            event.start_date,
            event.end_date,
            event.location,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn delete(&self, id: u32) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM event WHERE `id` = ?", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_activities(&self) -> Result<Vec<String>, sqlx::Error> {
        sqlx::query_scalar!("SELECT DISTINCT activity FROM event")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_depends_on(&self, event_id: u32) -> Result<Vec<i64>, sqlx::Error> {
        sqlx::query_scalar!(
            "SELECT DISTINCT dependency_id FROM event_dependency WHERE `event_id` = ?",
            event_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_dependencies(&self, event_id: u32) -> Result<Vec<Event>, sqlx::Error> {
        sqlx::query_as!(Event, "SELECT * FROM event WHERE `id` IN (SELECT DISTINCT dependency_id FROM event_dependency WHERE `event_id` = ?)", event_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn create_dependency(
        &self,
        event_id: u32,
        dependency_id: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO event_dependency (event_id, dependency_id) VALUES (?, ?)",
            event_id,
            dependency_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
