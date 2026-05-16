use axum::{Json, extract::State};
use serde_json::{Value, json};
use sqlx::SqlitePool;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::db::models::{CreateProject, Project};
use crate::db::project;
use crate::error::AppError;

pub async fn create_project(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateProject>,
) -> Result<Json<Value>, AppError> {
    let pro = Project {
        id: Uuid::new_v4(),
        name: payload.name,
        created_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| AppError::Internal("system time error".into()))?
            .as_secs() as i64,
    };

    project::create(&pool, &pro).await?;

    Ok(Json(json!(pro)))
}
