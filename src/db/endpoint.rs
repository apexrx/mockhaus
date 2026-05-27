use crate::db::models::Endpoint;
use sqlx::SqlitePool;

pub async fn create(pool: &SqlitePool, endpoint: &Endpoint) -> Result<(), sqlx::Error> {
    let id = endpoint.id.to_string();

    sqlx::query!(
        "INSERT INTO endpoints (id, project_id, method, path, status_code, response_body, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
        id,
        endpoint.project_id,
        endpoint.method,
        endpoint.path,
        endpoint.status_code,
        endpoint.response_body,
        endpoint.created_at,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete(pool: &SqlitePool, id: &str, project_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM endpoints WHERE id = ? AND project_id = ?",
        id,
        project_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get(pool: &SqlitePool, id: &str, project_id: &str) -> Result<Endpoint, sqlx::Error> {
    sqlx::query_as!(
        Endpoint,
        r#"SELECT id "id!", project_id "project_id!", method "method!", path "path!", status_code "status_code!", response_body "response_body!", created_at "created_at!" FROM endpoints WHERE id = ? AND project_id = ?"#,
        id,
        project_id
    )
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &SqlitePool,
    id: &str,
    project_id: &str,
    endpoint: &Endpoint,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE endpoints
         SET method = ?, path = ?, status_code = ?, response_body = ?
         WHERE id = ? AND project_id = ?",
        endpoint.method,
        endpoint.path,
        endpoint.status_code,
        endpoint.response_body,
        id,
        project_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_by_project_and_method(
    pool: &SqlitePool,
    project_id: &str,
    method: &str,
) -> Result<Vec<Endpoint>, sqlx::Error> {
    sqlx::query_as!(
        Endpoint,
        r#"
        SELECT
            id             as "id!",
            project_id     as "project_id!",
            method         as "method!",
            path           as "path!",
            status_code    as "status_code!",
            response_body  as "response_body!",
            created_at     as "created_at!"
        FROM endpoints
        WHERE project_id = ? AND method = ?
        "#,
        project_id,
        method
    )
    .fetch_all(pool)
    .await
}
