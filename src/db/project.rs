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

// Add arguments in the future if needed so the same function can be reused to modify the SELECT query
pub async fn list(pool: &SqlitePool) -> Result<Vec<Project>, sqlx::Error> {
    let projects = sqlx::query_as!(
        Project,
        r#"
        SELECT
            id as "id!",
            name as "name!",
            created_at as "created_at!"
        FROM projects
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(projects)
}

pub async fn get(pool: &SqlitePool, id: &str) -> Result<Project, sqlx::Error> {
    let project = sqlx::query_as!(
        Project,
        r#"
        SELECT
            id as "id!",
            name as "name!",
            created_at as "created_at!"
        FROM projects
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(project)
}

pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM projects WHERE id = ?", id,)
        .execute(pool)
        .await?;

    Ok(())
}
