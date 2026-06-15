use askama::Template;
use axum::{
    Router,
    extract::{FromRef, Path},
    routing::{get, post, put},
};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::net::SocketAddr;
use std::str::FromStr;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

mod api;
mod db;
mod error;
mod logger;
mod router;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

impl FromRef<AppState> for SqlitePool {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

#[derive(Template)]
#[template(path = "projects.html")]
struct ProjectsTemplate;

#[derive(Template)]
#[template(path = "editor.html")]
struct EditorTemplate {
    project_id: String,
}

#[derive(Template)]
#[template(path = "inspector.html")]
struct InspectorTemplate {
    project_id: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:mimic.db".into());
    let opts = SqliteConnectOptions::from_str(&database_url)?
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(opts).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = AppState { db: pool };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route(
            "/admin/projects",
            post(api::projects::create_project).get(api::projects::list_projects),
        )
        .route(
            "/admin/projects/{id}",
            get(api::projects::get_project).delete(api::projects::delete_project),
        )
        .route(
            "/admin/projects/{id}/endpoints",
            post(api::endpoints::add_endpoint).get(api::endpoints::list_endpoints),
        )
        .route(
            "/admin/projects/{id}/endpoints/{eid}",
            put(api::endpoints::update_endpoint).delete(api::endpoints::delete_endpoint),
        )
        .route(
            "/admin/projects/{id}/logs",
            get(api::logs::get_logs).delete(api::logs::delete_log),
        )
        .route(
            "/",
            get(|| async {
                let html = ProjectsTemplate
                    .render()
                    .map_err(|e| crate::error::AppError::Internal(e.to_string()))?;
                Ok::<_, crate::error::AppError>(axum::response::Html(html))
            }),
        )
        .route(
            "/projects/{id}/editor",
            get(|Path(id): Path<String>| async move {
                let html = EditorTemplate { project_id: id.clone() }
                    .render()
                    .map_err(|e| crate::error::AppError::Internal(e.to_string()))?;
                Ok::<_, crate::error::AppError>(axum::response::Html(html))
            }),
        )
        .route(
            "/projects/{id}/inspector",
            get(|Path(id): Path<String>| async move {
                let html = InspectorTemplate { project_id: id.clone() }
                    .render()
                    .map_err(|e| crate::error::AppError::Internal(e.to_string()))?;
                Ok::<_, crate::error::AppError>(axum::response::Html(html))
            }),
        )
        .nest_service("/static", ServeDir::new("static"))
        .nest("/mock", router::routes::router(state.clone()))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 7070));

    println!("listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
