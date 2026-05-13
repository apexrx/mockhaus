use axum::{extract::State, Json};
use serde_json::{json, Value};
use sqlx::SqlitePool;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::db::models::{CreateProject, Project};
use crate::error::AppError;

pub async fn create_project(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateProject>,
) -> Result<Json<Value>, AppError> {
    let id = Uuid::new_v4();
    let created_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| AppError::Internal("system time error".to_string()))?
        .as_secs() as i64;

    let name = payload.name;
    let id_str = id.to_string();
    sqlx::query!(
        "INSERT INTO projects (id, name, created_at) VALUES (?, ?, ?)",
        id_str,
        name,
        created_at,
    )
    .execute(&pool)
    .await?;

    let project = Project {
        id,
        name,
        created_at,
    };

    Ok(Json(json!(project)))
}
