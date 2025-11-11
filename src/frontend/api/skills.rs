// src/frontend/api/skills.rs
use super::fetch_api;
use crate::backend::models::responses::skills::SkillResponse;

pub async fn get_skills() -> Result<Vec<SkillResponse>, crate::frontend::errors::FrontendError> {
    fetch_api("/skills").await
}

pub async fn get_skills_by_category(
    _category: crate::domain::types::SkillCategory,
) -> Result<Vec<SkillResponse>, crate::frontend::errors::FrontendError> {
    fetch_api("/skills/category/backend").await
}
