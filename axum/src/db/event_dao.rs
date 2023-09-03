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

    pub async fn get(&self, id: u32) -> Result<Event, sqlx::Error> {
        sqlx::query_as!(Event, "SELECT * FROM event WHERE `id` = ?", id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn create(&self, event: &EventEgg) -> Result<i64, sqlx::Error> {
        let result = sqlx::query!(
            "INSERT INTO event (activity, start_date, end_date,  location) VALUES (?, ?, ?, ?) RETURNING id",
            event.activity,
            event.start_date,
            event.end_date,
            event.location,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.id)
    }
}
