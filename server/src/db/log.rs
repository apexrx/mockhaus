use sqlx::SqlitePool;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::db::models::RequestLog;

pub async fn insert(
    pool: &SqlitePool,
    project_id: &str,
    method: &str,
    path: &str,
    headers: &str,
    body: Option<&str>,
) -> Result<(), sqlx::Error> {
    let id = uuid::Uuid::new_v4().to_string();
    let received_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    sqlx::query!(
        r#"
        INSERT INTO request_logs (id, project_id, method, path, headers, body, received_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
        id,
        project_id,
        method,
        path,
        headers,
        body,
        received_at,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_by_project(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM request_logs WHERE project_id = ?
        "#,
        id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn list(
    pool: &SqlitePool,
    project_id: &str,
    limit: Option<i64>,
) -> Result<Vec<RequestLog>, sqlx::Error> {
    let limit: i64 = limit.unwrap_or(100);

    let logs = sqlx::query_as!(
        RequestLog,
        r#"
        SELECT
            id,
            project_id,
            endpoint_id,
            method,
            path,
            headers,
            body,
            received_at
        FROM request_logs
        WHERE project_id = ?
        ORDER BY received_at DESC
        LIMIT ?
        "#,
        project_id,
        limit,
    )
    .fetch_all(pool)
    .await?;

    Ok(logs)
}
