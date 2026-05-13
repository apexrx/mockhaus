use axum::{extract::FromRef, routing::get, routing::post, Router};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::net::SocketAddr;

mod api;
mod db;
mod logger;
mod router;
mod error;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

impl FromRef<AppState> for SqlitePool {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = SqliteConnectOptions::new().filename("mimic.db").create_if_missing(true);
    let pool = SqlitePool::connect_with(opts).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = AppState { db: pool };

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/admin/projects", post(api::projects::create_project))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 7070));
    println!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
