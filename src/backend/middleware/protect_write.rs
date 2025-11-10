// backend/middleware/protect_write.rs
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn protect_write_operations(
    request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response<axum::body::Body>, StatusCode> {
    let method = request.method().as_str();
    let path = request.uri().path();

    // Разрешаем GET запросы без аутентификации
    if method == "GET" {
        return Ok(next.run(request).await);
    }

    // Для всех остальных методов требуем аутентификацию
    crate::backend::middleware::auth::auth_middleware(request, next).await
}
