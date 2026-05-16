use crate::db::models::Project;
use sqlx::SqlitePool;

pub async fn create(pool: &SqlitePool, project: &Project) -> Result<(), sqlx::Error> {
    let id = project.id.to_string();

    sqlx::query!(
        "INSERT INTO projects (id, name, created_at) VALUES (?, ?, ?)",
        id,
        project.name,
        project.created_at,
    )
    .execute(pool)
    .await?;

    Ok(())
}
