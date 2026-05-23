use axum::{
    body::{Body, to_bytes},
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use sqlx::SqlitePool;
use std::{collections::HashMap, time::Instant};

pub async fn log_request(State(pool): State<SqlitePool>, req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    let project_id = path.split('/').nth(2).unwrap_or("").to_string();

    let mut header_map = HashMap::new();

    for (name, value) in req.headers() {
        header_map.insert(name.to_string(), value.to_str().unwrap_or("").to_string());
    }

    let headers_json = serde_json::to_string(&header_map).unwrap();

    let (parts, body) = req.into_parts();

    let bytes = to_bytes(body, usize::MAX).await.unwrap();

    let body_string = String::from_utf8_lossy(&bytes).to_string();

    let rebuilt_body = Body::from(bytes.clone());

    let req = Request::from_parts(parts, rebuilt_body);

    let start = Instant::now();

    let response = next.run(req).await;

    let status = response.status();
    let elapsed = start.elapsed();

    let method_str = method.to_string();
    if let Err(e) = crate::db::log::insert(
        &pool,
        &project_id,
        &method_str,
        &path,
        &headers_json,
        &body_string,
    )
    .await
    {
        eprintln!("Failed to insert log: {e}");
    }

    println!("{method} -> {status} ({elapsed:?})");

    response
}
