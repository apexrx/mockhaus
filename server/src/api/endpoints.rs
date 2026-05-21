use axum::extract::{Path, State};
use axum::{Json, http::StatusCode, response::IntoResponse};
use sqlx::SqlitePool;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::db::endpoint;
use crate::db::models::{CreateEndpoint, Endpoint, UpdateEndpoint};
use crate::error::AppError;

pub async fn add_endpoint(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(payload): Json<CreateEndpoint>,
) -> Result<impl IntoResponse, AppError> {
    let epi = Endpoint {
        id: Uuid::new_v4().to_string(),
        project_id: id,
        method: payload.method,
        path: payload.path,
        status_code: payload.status_code,
        response_body: payload.response_body,
        created_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| AppError::Internal("system time error".into()))?
            .as_secs() as i64,
    };

    endpoint::create(&pool, &epi).await?;
    Ok(StatusCode::CREATED)
}

pub async fn update_endpoint(
    State(pool): State<SqlitePool>,
    Path((id, eid)): Path<(String, String)>,
    Json(payload): Json<UpdateEndpoint>,
) -> Result<impl IntoResponse, AppError> {
    let mut epi = endpoint::get(&pool, &eid, &id).await?;

    if let Some(method) = payload.method {
        epi.method = method;
    }
    if let Some(path) = payload.path {
        epi.path = path;
    }
    if let Some(status_code) = payload.status_code {
        epi.status_code = status_code;
    }
    if let Some(response_body) = payload.response_body {
        epi.response_body = response_body;
    }

    endpoint::update(&pool, &eid, &id, &epi).await?;
    Ok(StatusCode::OK)
}

pub async fn delete_endpoint(
    Path((id, eid)): Path<(String, String)>,
    State(pool): State<SqlitePool>,
) -> Result<impl IntoResponse, AppError> {
    endpoint::delete(&pool, &id, &eid).await?;
    Ok(StatusCode::NO_CONTENT)
}
