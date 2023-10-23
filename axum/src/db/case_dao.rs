use super::models::{Case, CaseEgg};

pub struct CaseDao {
    pub pool: sqlx::SqlitePool,
}

impl CaseDao {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<Case>, sqlx::Error> {
        sqlx::query_as!(Case, "SELECT id, project_id, name, description FROM 'case'")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn create(&self, case: &CaseEgg) -> Result<Case, sqlx::Error> {
        sqlx::query_as!(
            Case,
            "INSERT INTO 'case' (project_id, name, description) VALUES (?, ?, ?) returning id, project_id, name, description",
            case.project_id,
            case.name,
            case.description
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete(&self, id: u32) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM 'case' WHERE id = ?", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
