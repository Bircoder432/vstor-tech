// backend/api/mod.rs
pub mod health;
pub mod projects;
pub mod skills;

use crate::backend::repositories::{InMemoryProjectRepo, InMemorySkillRepo};
use crate::backend::services::{ProjectService, SkillService};
use axum::Router;

pub fn create_app() -> Router {
    let skill_repo = InMemorySkillRepo::new();
    let project_repo = InMemoryProjectRepo::new();

    Router::new()
        .merge(health::health_router())
        .nest(
            "/api/skills",
            skills::skills_router::<InMemorySkillRepo>().with_state(SkillService::new(skill_repo)),
        )
        .nest(
            "/api/projects",
            projects::projects_router::<InMemoryProjectRepo>()
                .with_state(ProjectService::new(project_repo)),
        )
}
