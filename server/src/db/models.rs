use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProject {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Endpoint {
    pub id: String,
    pub project_id: String,
    pub method: String,
    pub path: String,
    pub status_code: i64,
    pub response_body: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEndpoint {
    pub method: String,
    pub path: String,
    pub status_code: i64,
    pub response_body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEndpoint {
    pub method: Option<String>,
    pub path: Option<String>,
    pub status_code: Option<i64>,
    pub response_body: Option<String>,
}
