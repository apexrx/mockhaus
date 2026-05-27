use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub enum AppError {
    Database(sqlx::Error),
    NotFound(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(err) => {
                eprintln!("Database error: {:?}", err);

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }

            AppError::NotFound(msg) => {
                eprintln!("Not found: {}", msg);

                (StatusCode::NOT_FOUND, msg)
            }

            AppError::Internal(msg) => {
                eprintln!("Internal error: {}", msg);

                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(inner: sqlx::Error) -> Self {
        match inner {
            sqlx::Error::RowNotFound => AppError::NotFound("Project not found".to_string()),

            _ => AppError::Database(inner),
        }
    }
}
