use axum::{Router, middleware, routing::any};

pub fn router(state: crate::AppState) -> Router<crate::AppState> {
    Router::new()
        .route("/{project_id}/{*path}", any(super::handle_mock_request))
        .layer(middleware::from_fn_with_state(
            state.db.clone(),
            crate::logger::log::log_request,
        ))
}
