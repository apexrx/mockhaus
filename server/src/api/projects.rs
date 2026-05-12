use axum::{extract::State, Json};
use serde_json::{json, Value};
use sqlx::SqlitePool;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::db::models::{CreateProject, Project};

pub async fn create_project(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateProject>,
) -> Json<Value> {
    let id = Uuid::new_v4();
    let created_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    sqlx::query("INSERT INTO projects (id, name, created_at) VALUES (?, ?, ?)")
        .bind(id.to_string())
        .bind(&payload.name)
        .bind(created_at)
        .execute(&pool)
        .await
        .unwrap();

    let project = Project {
        id,
        name: payload.name,
        created_at,
    };

    Json(json!(project))
}
