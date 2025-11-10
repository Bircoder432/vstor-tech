use crate::domain::types::{ContentVisibility, SkillCategory, SkillLevel};
use crate::shared::types::ImageSource;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSkillRequest {
    #[schema(example = "Rust")]
    pub name: String,
    pub category: SkillCategory,
    #[schema(example = "Systems programming language")]
    pub description: String,
    #[schema(example = 2)]
    pub years_of_experience: u8,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateSkillNameRequest {
    #[schema(example = "Updated Rust")]
    pub name: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateSkillCategoryRequest {
    pub category: SkillCategory,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateSkillVisibilityRequest {
    pub visibility: ContentVisibility,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateSkillLevelRequest {
    pub level: SkillLevel,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DeleteSkillsBatchRequest {
    #[schema(example = json!(["id1", "id2"]))]
    pub skill_ids: Vec<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateSkillImageRequest {
    pub image: Option<ImageSource>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateSkillDescriptionRequest {
    #[schema(example = "Updated description for the skill")]
    pub description: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateSkillYearsRequest {
    #[schema(example = 3)]
    pub years_of_experience: u8,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddSkillImageRequest {
    pub image: ImageSource,
}
