// backend/api/health.rs
use axum::{Router, routing::get};

pub fn health_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/ready", get(ready_check))
}

async fn health_check() -> &'static str {
    "OK"
}

async fn ready_check() -> &'static str {
    "READY"
}
