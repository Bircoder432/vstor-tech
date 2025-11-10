// backend/api/skills.rs
use crate::backend::repositories::SkillRepository;
use crate::backend::services::SkillService;
use crate::domain::entities::Skill;
use crate::domain::types::{SkillCategory, SkillLevel};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post, put},
};

// DTOs для запросов/ответов
#[derive(Debug, serde::Deserialize)]
pub struct CreateSkillRequest {
    pub name: String,
    pub category: SkillCategory,
    pub description: String,
    pub years_of_experience: u8,
}

#[derive(Debug, serde::Serialize)]
pub struct SkillResponse {
    pub id: String,
    pub name: String,
    pub category: SkillCategory,
    pub level: SkillLevel,
    pub description: String,
    pub years_of_experience: u8,
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
        }
    }
}

pub fn skills_router<R>() -> Router<SkillService<R>>
where
    R: SkillRepository + Clone + Send + Sync + 'static,
    SkillService<R>: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(get_skills::<R>).post(create_skill::<R>))
        .route(
            "/{id}",
            get(get_skill::<R>)
                .put(update_skill::<R>)
                .delete(delete_skill::<R>),
        )
        .route("/category/{category}", get(get_skills_by_category::<R>))
}

// Handlers
async fn get_skills<R: SkillRepository>(
    State(service): State<SkillService<R>>,
) -> Json<Vec<SkillResponse>> {
    let skills = service.get_all_skills().await.unwrap_or_default();
    Json(skills.into_iter().map(SkillResponse::from).collect())
}

async fn get_skill<R: SkillRepository>(
    State(service): State<SkillService<R>>,
    Path(id): Path<String>,
) -> Result<Json<SkillResponse>, String> {
    let skill = service
        .get_skill(&id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Skill not found".to_string())?;

    Ok(Json(skill.into()))
}

async fn create_skill<R: SkillRepository>(
    State(service): State<SkillService<R>>,
    Json(payload): Json<CreateSkillRequest>,
) -> Result<Json<SkillResponse>, String> {
    let skill = Skill::new(
        payload.name,
        payload.category,
        payload.description,
        payload.years_of_experience,
    );

    let skill = service
        .create_skill(skill)
        .await
        .map_err(|e| e.to_string())?;

    Ok(Json(skill.into()))
}

async fn update_skill<R: SkillRepository>(
    State(service): State<SkillService<R>>,
    Path(id): Path<String>,
    Json(payload): Json<CreateSkillRequest>,
) -> Result<Json<SkillResponse>, String> {
    let skill = service
        .update_skill_experience(&id, payload.years_of_experience)
        .await
        .map_err(|e| e.to_string())?;

    Ok(Json(skill.into()))
}

async fn delete_skill<R: SkillRepository>(
    State(service): State<SkillService<R>>,
    Path(id): Path<String>,
) -> Result<(), String> {
    service.delete_skill(&id).await.map_err(|e| e.to_string())
}

async fn get_skills_by_category<R: SkillRepository>(
    State(service): State<SkillService<R>>,
    Path(category): Path<SkillCategory>,
) -> Json<Vec<SkillResponse>> {
    let skills = service
        .get_skills_by_category(category)
        .await
        .unwrap_or_default();
    Json(skills.into_iter().map(SkillResponse::from).collect())
}
