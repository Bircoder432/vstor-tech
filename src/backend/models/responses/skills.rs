use crate::domain::entities::Skill;
use crate::domain::types::{SkillCategory, SkillLevel};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct SkillResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: String,
    #[schema(example = "Rust")]
    pub name: String,
    pub category: SkillCategory,
    pub level: SkillLevel,
    #[schema(example = "Systems programming language")]
    pub description: String,
    #[schema(example = 2)]
    pub years_of_experience: u8,
    pub image: Option<crate::shared::types::ImageSource>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DeleteBatchResponse {
    pub results: Vec<(String, String)>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

impl From<Skill> for SkillResponse {
    fn from(skill: Skill) -> Self {
        Self {
            id: skill.id().to_string(),
            name: skill.name().to_string(),
            category: skill.category().clone(),
            level: skill.level().clone(),
            description: skill.description().to_string(),
            years_of_experience: skill.years_of_experience(),
            image: skill.image().cloned(),
        }
    }
}
