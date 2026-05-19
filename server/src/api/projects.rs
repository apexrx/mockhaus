use axum::extract::{Path, State};
use axum::{Json, http::StatusCode, response::IntoResponse};
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
        id: Uuid::new_v4().to_string(),
        name: payload.name,
        created_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| AppError::Internal("system time error".into()))?
            .as_secs() as i64,
    };

    project::create(&pool, &pro).await?;

    Ok(Json(json!(pro)))
}

pub async fn list_projects(State(pool): State<SqlitePool>) -> Result<Json<Value>, AppError> {
    let projects = project::list(&pool).await?;
    Ok(Json(json!(projects)))
}

pub async fn get_project(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
) -> Result<Json<Value>, AppError> {
    let project = project::get(&pool, &id).await?;
    Ok(Json(json!(project)))
}

pub async fn delete_project(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
) -> Result<impl IntoResponse, AppError> {
    project::delete(&pool, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}
