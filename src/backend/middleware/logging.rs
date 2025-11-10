// backend/middleware/logging.rs
use axum::{
    http::{Request, Response},
    middleware::Next,
};
use std::time::Instant;

pub async fn logging_middleware(
    request: Request<axum::body::Body>,
    next: Next,
) -> Response<axum::body::Body> {
    let start = Instant::now();
    let method = request.method().to_string();
    let path = request.uri().path().to_string();

    println!("➡️  {} {} - started", method, path);

    let response = next.run(request).await;

    let duration = start.elapsed();
    let status = response.status().as_u16();

    println!(
        "⬅️  {} {} - {} - {}ms",
        method,
        path,
        status,
        duration.as_millis()
    );

    response
}
