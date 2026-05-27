use axum::extract::{Path, Query, State};
use axum::{Json, http::StatusCode, response::IntoResponse};
use sqlx::SqlitePool;

use crate::db::{log, models::RequestLog};
use crate::error::AppError;

#[derive(serde::Deserialize)]
pub struct LogQuery {
    pub limit: Option<i64>,
}

pub async fn delete_log(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    log::delete_by_project(&pool, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_logs(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Query(query): Query<LogQuery>,
) -> Result<Json<Vec<RequestLog>>, AppError> {
    let limit = query.limit;
    let logs = log::list(&pool, &id, limit).await?;
    Ok(Json(logs))
}
