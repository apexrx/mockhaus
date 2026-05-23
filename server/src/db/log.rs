use sqlx::SqlitePool;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn insert(
    pool: &SqlitePool,
    project_id: &str,
    method: &str,
    path: &str,
    headers: &str,
    body: &str,
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
