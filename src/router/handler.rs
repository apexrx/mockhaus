use axum::{
    extract::{Path, State},
    http::{Method, StatusCode, header::CONTENT_TYPE},
    response::IntoResponse,
};
use sqlx::SqlitePool;
use std::collections::HashMap;

use crate::db::endpoint;
use crate::error::AppError;

pub async fn handle_mock_request(
    method: Method,
    Path((project_id, path)): Path<(String, String)>,
    State(pool): State<SqlitePool>,
) -> Result<impl IntoResponse, AppError> {
    let endpoints =
        endpoint::find_by_project_and_method(&pool, &project_id, method.as_str()).await?;

    for endpoint in endpoints {
        if let Some(_params) = match_path(&endpoint.path, &path) {
            return Ok((
                StatusCode::from_u16(endpoint.status_code as u16).unwrap_or(StatusCode::OK),
                [(CONTENT_TYPE, "application/json")],
                endpoint.response_body,
            ));
        }
    }
    Err(AppError::NotFound(
        "Mock endpoint not configured".to_string(),
    ))
}

pub fn match_path(pattern: &str, incoming: &str) -> Option<HashMap<String, String>> {
    let pattern_parts: Vec<&str> = pattern.split('/').filter(|s| !s.is_empty()).collect();

    let incoming_parts: Vec<&str> = incoming.split('/').filter(|s| !s.is_empty()).collect();

    if pattern_parts.len() != incoming_parts.len() {
        return None;
    }

    let mut params = HashMap::new();

    for (pat, inc) in pattern_parts.iter().zip(incoming_parts.iter()) {
        if let Some(param_name) = pat.strip_prefix(':') {
            params.insert(param_name.to_string(), inc.to_string());
        } else if pat != inc {
            return None;
        }
    }

    Some(params)
}
