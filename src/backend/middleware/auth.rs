use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::collections::HashSet;

pub async fn auth_middleware(
    request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response<axum::body::Body>, StatusCode> {
    if is_public_endpoint(request.uri().path()) {
        return Ok(next.run(request).await);
    }

    let auth_header = request
        .headers()
        .get("Authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let auth_str = auth_header.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !auth_str.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_str[7..];

    let valid_token = get_valid_token();
    if token != valid_token {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}

fn is_public_endpoint(path: &str) -> bool {
    let public_paths: HashSet<&str> = [
        "/health",
        "/ready",
        "/api/skills",
        "/api/projects",
        "/api/projects/featured",
        "/swagger-ui",
        "/api-docs/openapi.json",
    ]
    .iter()
    .cloned()
    .collect();

    public_paths.contains(path)
        || path.starts_with("/api/skills/category/")
        || path.starts_with("/swagger-ui/")
}

fn get_valid_token() -> String {
    crate::backend::config::Config::from_env()
        .ok()
        .and_then(|config| config.access_token)
        .unwrap_or_else(|| "default_token".to_string())
}
