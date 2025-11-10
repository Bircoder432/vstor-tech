use crate::backend::models::requests::skills::*;
use crate::backend::models::responses::skills::*;
use crate::backend::repositories::SkillRepository;
use crate::backend::services::SkillService;
use crate::domain::entities::Skill;
use crate::domain::types::SkillCategory;
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post, put},
};
use std::sync::Arc;
use utoipa::IntoParams;

type SkillServiceType = SkillService<Box<dyn SkillRepository>>;

pub fn skills_router() -> Router<Arc<SkillServiceType>> {
    Router::new()
        .route("/", get(get_skills).post(create_skill))
        .route(
            "/{id}",
            get(get_skill).put(update_skill).delete(delete_skill),
        )
        .route("/category/{category}", get(get_skills_by_category))
        .route("/{id}/name", put(update_skill_name))
        .route("/{id}/category", put(update_skill_category))
        .route("/{id}/visibility", put(update_skill_visibility))
        .route("/{id}/level", put(update_skill_level))
        .route("/{id}/description", put(update_skill_description))
        .route("/{id}/years", put(update_skill_years))
        .route(
            "/{id}/image",
            put(update_skill_image).delete(remove_skill_image),
        )
        .route("/batch", delete(delete_skills_batch))
}

#[utoipa::path(
    get,
    path = "/api/skills",
    responses(
        (status = 200, description = "Список всех навыков", body = [SkillResponse])
    ),
    tag = "skills"
)]
async fn get_skills(State(service): State<Arc<SkillServiceType>>) -> Json<Vec<SkillResponse>> {
    let skills = service.get_all_skills().await.unwrap_or_default();
    Json(skills.into_iter().map(SkillResponse::from).collect())
}

#[utoipa::path(
    get,
    path = "/api/skills/{id}",
    params(
        ("id" = String, Path, description = "UUID навыка")
    ),
    responses(
        (status = 200, description = "Навык найден", body = SkillResponse),
        (status = 404, description = "Навык не найден")
    ),
    tag = "skills"
)]
async fn get_skill(
    State(service): State<Arc<SkillServiceType>>,
    Path(id): Path<String>,
) -> Result<Json<SkillResponse>, String> {
    let skill = service
        .get_skill(&id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Skill not found".to_string())?;

    Ok(Json(skill.into()))
}

#[utoipa::path(
    post,
    path = "/api/skills",
    request_body = CreateSkillRequest,
    responses(
        (status = 200, description = "Навык создан", body = SkillResponse),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "skills"
)]
async fn create_skill(
    State(service): State<Arc<SkillServiceType>>,
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

#[utoipa::path(
    put,
    path = "/api/skills/{id}",
    params(
        ("id" = String, Path, description = "UUID навыка")
    ),
    request_body = CreateSkillRequest,
    responses(
        (status = 200, description = "Навык обновлен", body = SkillResponse),
        (status = 404, description = "Навык не найден"),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "skills"
)]
async fn update_skill(
    State(service): State<Arc<SkillServiceType>>,
    Path(id): Path<String>,
    Json(payload): Json<CreateSkillRequest>,
) -> Result<Json<SkillResponse>, String> {
    let skill = service
        .update_skill_experience(&id, payload.years_of_experience)
        .await
        .map_err(|e| e.to_string())?;

    Ok(Json(skill.into()))
}

#[utoipa::path(
    delete,
    path = "/api/skills/{id}",
    params(
        ("id" = String, Path, description = "UUID навыка")
    ),
    responses(
        (status = 200, description = "Навык удален", body = MessageResponse),
        (status = 404, description = "Навык не найден")
    ),
    tag = "skills"
)]
async fn delete_skill(
    State(service): State<Arc<SkillServiceType>>,
    Path(id): Path<String>,
) -> Result<Json<MessageResponse>, String> {
    service.delete_skill(&id).await.map_err(|e| e.to_string())?;

    Ok(Json(MessageResponse {
        message: "Skill deleted successfully".to_string(),
    }))
}

#[utoipa::path(
    get,
    path = "/api/skills/category/{category}",
    params(
        ("category" = SkillCategory, Path, description = "Категория навыков")
    ),
    responses(
        (status = 200, description = "Навыки по категории", body = [SkillResponse])
    ),
    tag = "skills"
)]
async fn get_skills_by_category(
    State(service): State<Arc<SkillServiceType>>,
    Path(category): Path<SkillCategory>,
) -> Json<Vec<SkillResponse>> {
    let skills = service
        .get_skills_by_category(category)
        .await
        .unwrap_or_default();
    Json(skills.into_iter().map(SkillResponse::from).collect())
}

#[utoipa::path(
    put,
    path = "/api/skills/{id}/name",
    params(
        ("id" = String, Path, description = "UUID навыка")
    ),
    request_body = UpdateSkillNameRequest,
    responses(
        (status = 200, description = "Имя навыка обновлено", body = SkillResponse),
        (status = 404, description = "Навык не найден"),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "skills"
)]
async fn update_skill_name(
    State(service): State<Arc<SkillServiceType>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateSkillNameRequest>,
) -> Result<Json<SkillResponse>, String> {
    let skill = service
        .update_skill_name(&id, payload.name)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(skill.into()))
}

#[utoipa::path(
    put,
    path = "/api/skills/{id}/category",
    params(
        ("id" = String, Path, description = "UUID навыка")
    ),
    request_body = UpdateSkillCategoryRequest,
    responses(
        (status = 200, description = "Категория навыка обновлена", body = SkillResponse),
        (status = 404, description = "Навык не найден"),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "skills"
)]
async fn update_skill_category(
    State(service): State<Arc<SkillServiceType>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateSkillCategoryRequest>,
) -> Result<Json<SkillResponse>, String> {
    let skill = service
        .update_skill_category(&id, payload.category)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(skill.into()))
}

#[utoipa::path(
    put,
    path = "/api/skills/{id}/visibility",
    params(
        ("id" = String, Path, description = "UUID навыка")
    ),
    request_body = UpdateSkillVisibilityRequest,
    responses(
        (status = 200, description = "Видимость навыка обновлена", body = SkillResponse),
        (status = 404, description = "Навык не найден"),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "skills"
)]
async fn update_skill_visibility(
    State(service): State<Arc<SkillServiceType>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateSkillVisibilityRequest>,
) -> Result<Json<SkillResponse>, String> {
    let skill = service
        .update_skill_visibility(&id, payload.visibility)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(skill.into()))
}

#[utoipa::path(
    put,
    path = "/api/skills/{id}/level",
    params(
        ("id" = String, Path, description = "UUID навыка")
    ),
    request_body = UpdateSkillLevelRequest,
    responses(
        (status = 200, description = "Уровень навыка обновлен", body = SkillResponse),
        (status = 404, description = "Навык не найден"),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "skills"
)]
async fn update_skill_level(
    State(service): State<Arc<SkillServiceType>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateSkillLevelRequest>,
) -> Result<Json<SkillResponse>, String> {
    let skill = service
        .update_skill_level(&id, payload.level)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(skill.into()))
}

#[utoipa::path(
    delete,
    path = "/api/skills/batch",
    request_body = DeleteSkillsBatchRequest,
    responses(
        (status = 200, description = "Пакетное удаление завершено", body = DeleteBatchResponse),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "skills"
)]
async fn delete_skills_batch(
    State(service): State<Arc<SkillServiceType>>,
    Json(payload): Json<DeleteSkillsBatchRequest>,
) -> Result<Json<DeleteBatchResponse>, String> {
    let mut results = Vec::new();

    for skill_id in payload.skill_ids {
        match service.delete_skill(&skill_id).await {
            Ok(()) => results.push((skill_id, "deleted".to_string())),
            Err(e) => results.push((skill_id, format!("error: {}", e))),
        }
    }

    Ok(Json(DeleteBatchResponse { results }))
}

#[utoipa::path(
    put,
    path = "/api/skills/{id}/description",
    params(
        ("id" = String, Path, description = "UUID навыка")
    ),
    request_body = UpdateSkillDescriptionRequest,
    responses(
        (status = 200, description = "Описание навыка обновлено", body = SkillResponse),
        (status = 404, description = "Навык не найден"),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "skills"
)]
async fn update_skill_description(
    State(service): State<Arc<SkillServiceType>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateSkillDescriptionRequest>,
) -> Result<Json<SkillResponse>, String> {
    let skill = service
        .update_skill_description(&id, payload.description)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(skill.into()))
}

#[utoipa::path(
    put,
    path = "/api/skills/{id}/years",
    params(
        ("id" = String, Path, description = "UUID навыка")
    ),
    request_body = UpdateSkillYearsRequest,
    responses(
        (status = 200, description = "Опыт навыка обновлен", body = SkillResponse),
        (status = 404, description = "Навык не найден"),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "skills"
)]
async fn update_skill_years(
    State(service): State<Arc<SkillServiceType>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateSkillYearsRequest>,
) -> Result<Json<SkillResponse>, String> {
    let skill = service
        .update_skill_years_of_experience(&id, payload.years_of_experience)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(skill.into()))
}

#[utoipa::path(
    put,
    path = "/api/skills/{id}/image",
    params(
        ("id" = String, Path, description = "UUID навыка")
    ),
    request_body = UpdateSkillImageRequest,
    responses(
        (status = 200, description = "Изображение навыка обновлено", body = SkillResponse),
        (status = 404, description = "Навык не найден"),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "skills"
)]
async fn update_skill_image(
    State(service): State<Arc<SkillServiceType>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateSkillImageRequest>,
) -> Result<Json<SkillResponse>, String> {
    let skill = service
        .update_skill_image(&id, payload.image)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(skill.into()))
}

#[utoipa::path(
    delete,
    path = "/api/skills/{id}/image",
    params(
        ("id" = String, Path, description = "UUID навыка")
    ),
    responses(
        (status = 200, description = "Изображение навыка удалено", body = SkillResponse),
        (status = 404, description = "Навык не найден")
    ),
    tag = "skills"
)]
async fn remove_skill_image(
    State(service): State<Arc<SkillServiceType>>,
    Path(id): Path<String>,
) -> Result<Json<SkillResponse>, String> {
    let skill = service
        .update_skill_image(&id, None)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(skill.into()))
}
