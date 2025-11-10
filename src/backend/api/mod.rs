pub mod health;
pub mod openapi;
pub mod projects;
pub mod skills;

use crate::backend::config::Config;
use crate::backend::middleware::logging::logging_middleware;
use crate::backend::middleware::protect_write_operations;
use crate::backend::services::{ProjectService, SkillService};
use axum::Router;
use openapi::ApiDoc;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn create_app() -> Router {
    // Загружаем конфиг
    let config = Config::from_env().expect("Failed to load configuration");

    // Создаем репозитории на основе конфига
    let skill_repo = config.database.create_skill_repo();
    let project_repo = config.database.create_project_repo();

    println!("📊 Using database: {:?}", config.database.db_type);

    // Оборачиваем сервисы в Arc
    let skill_service = Arc::new(SkillService::new(skill_repo));
    let project_service = Arc::new(ProjectService::new(project_repo));

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(health::health_router())
        .nest(
            "/api/skills",
            skills::skills_router()
                .with_state(skill_service)
                .layer(axum::middleware::from_fn(protect_write_operations)),
        )
        .nest(
            "/api/projects",
            projects::projects_router()
                .with_state(project_service)
                .layer(axum::middleware::from_fn(protect_write_operations)),
        )
        .layer(axum::middleware::from_fn(logging_middleware))
}
