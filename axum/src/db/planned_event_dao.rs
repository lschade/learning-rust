use super::models::{Event, EventEgg, PlannedEvent, PlannedEventEgg};

pub struct PlannedEventDao {
    pub pool: sqlx::SqlitePool,
}

impl PlannedEventDao {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<PlannedEvent>, sqlx::Error> {
        sqlx::query_as!(PlannedEvent, "SELECT * FROM planned_event")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn list_by_project(&self, project_id: u32) -> Result<Vec<PlannedEvent>, sqlx::Error> {
        sqlx::query_as!(
            PlannedEvent,
            "SELECT * FROM planned_event WHERE `project_id` = ?",
            project_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get(&self, id: u32) -> Result<PlannedEvent, sqlx::Error> {
        sqlx::query_as!(
            PlannedEvent,
            "SELECT * FROM planned_event WHERE `id` = ?",
            id
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create(&self, event: &PlannedEventEgg) -> Result<PlannedEvent, sqlx::Error> {
        let result = sqlx::query_as!(
            PlannedEvent,
            "INSERT INTO planned_event (project_id, case_id, activity, description, earliest_start_date, due_date, completed) VALUES (?, ?, ?, ?, ?, ?, False) RETURNING *",
            event.project_id,
            event.case_id,
            event.activity,
            event.description,
            event.earliest_start_date,
            event.due_date,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn delete(&self, id: u32) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM planned_event WHERE `id` = ?", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn update(&self, event: &PlannedEvent) -> Result<PlannedEvent, sqlx::Error> {
        let result = sqlx::query_as!(
            PlannedEvent,
            r#"UPDATE planned_event SET project_id = ?, case_id = ?, activity = ?, description = ?, earliest_start_date = ?, due_date = ?, completed = ?, event_id = ? WHERE id = ? RETURNING id as "id!", project_id as "project_id!", case_id as "case_id!", activity, description, earliest_start_date, due_date, completed, event_id"#,
            event.project_id,
            event.case_id,
            event.activity,
            event.description,
            event.earliest_start_date,
            event.due_date,
            event.completed,
            event.event_id,
            event.id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn complete(&self, planned_event: PlannedEvent) -> Result<Event, sqlx::Error> {
        let mut transaction = self.pool.begin().await.unwrap();

        let new_event = EventEgg {
            project_id: planned_event.project_id,
            case_id: planned_event.case_id,
            activity: planned_event.activity.clone(),
            start_date: planned_event.earliest_start_date,
            end_date: Some(planned_event.due_date),
            location: None,
        };

        let new_event = sqlx::query_as!(
            Event,
            r#"INSERT INTO event (project_id, case_id, activity, start_date, end_date, location) VALUES (?, ?, ?, ?, ?, ?) RETURNING *"#,
            new_event.project_id,
            new_event.case_id,
            new_event.activity,
            new_event.start_date,
            new_event.end_date,
            new_event.location,
        ).fetch_one(&mut *transaction).await?;

        let _ = sqlx::query_as!(
            PlannedEvent,
            r#"UPDATE planned_event SET completed = True, event_id = ? WHERE id = ? 
            RETURNING id as "id!", project_id as "project_id!", case_id as "case_id!", activity, description, earliest_start_date, due_date, completed, event_id"#,
            new_event.id,
            planned_event.id
        )
        .fetch_one(&mut *transaction)
        .await?;

        let _ = transaction.commit().await?;

        return Ok(new_event);
    }
}
