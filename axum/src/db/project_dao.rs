use super::models::{Project, ProjectEgg};

pub struct ProjectDao {
    pub pool: sqlx::SqlitePool,
}
impl ProjectDao {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_projects(&self) -> Result<Vec<Project>, sqlx::Error> {
        sqlx::query_as!(Project, "SELECT * FROM project")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_project(&self, id: u32) -> Result<Project, sqlx::Error> {
        sqlx::query_as!(Project, "SELECT * FROM project WHERE id = ?", id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn create_project(&self, project: &ProjectEgg) -> Result<Project, sqlx::Error> {
        let result = sqlx::query_as!(
            Project,
            "INSERT INTO project (name, description) VALUES (?, ?) RETURNING *",
            project.name,
            project.description
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn delete_project(&self, id: u32) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM project WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn update_project(&self, project: &Project) -> Result<Project, sqlx::Error> {
        let result = sqlx::query_as!(
            Project,
            r#"UPDATE project SET name = ?, description = ? WHERE id = ? RETURNING id as "id!", name, description"#,
            project.name,
            project.description,
            project.id  
        )
        .fetch_one(&self.pool)
        .await?;
 
        Ok(result)
    }
}
