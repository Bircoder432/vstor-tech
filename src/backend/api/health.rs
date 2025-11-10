use axum::{Router, routing::get};

pub fn health_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/ready", get(ready_check))
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Проверка работоспособности")
    ),
    tag = "health"
)]
async fn health_check() -> &'static str {
    "OK"
}

#[utoipa::path(
    get,
    path = "/ready",
    responses(
        (status = 200, description = "Проверка готовности")
    ),
    tag = "health"
)]
async fn ready_check() -> &'static str {
    "READY"
}
