use crate::backend::models::requests::projects::*;
use crate::backend::models::responses::projects::*;
use crate::backend::repositories::ProjectRepository;
use crate::backend::services::ProjectService;
use crate::domain::entities::Project;
use crate::domain::types::UniqueTechnologies;
use crate::shared::types::ProjectUrl;
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post, put},
};
use std::sync::Arc;
use utoipa::IntoParams;

type ProjectServiceType = ProjectService<Box<dyn ProjectRepository>>;

pub fn projects_router() -> Router<Arc<ProjectServiceType>> {
    Router::new()
        .route("/", get(get_projects).post(create_project))
        .route("/{id}", get(get_project).delete(delete_project))
        .route("/featured", get(get_featured_projects))
        .route("/{id}/name", put(update_project_name))
        .route("/{id}/featured", put(update_project_featured))
        .route("/{id}/status", put(update_project_status))
        .route("/{id}/technologies", post(add_project_technology))
        .route("/{id}/description", put(update_project_description))
        .route(
            "/{id}/long-description",
            put(update_project_long_description),
        )
        .route("/{id}/visibility", put(update_project_visibility))
        .route("/{id}/dates", put(update_project_dates))
        .route(
            "/{id}/image",
            put(update_project_image).delete(remove_project_image),
        )
        .route(
            "/{id}/urls",
            post(add_project_url).delete(remove_project_url),
        )
        .route("/batch", delete(delete_projects_batch))
}

#[utoipa::path(
    get,
    path = "/api/projects",
    responses(
        (status = 200, description = "Список видимых проектов", body = [ProjectResponse])
    ),
    tag = "projects"
)]
async fn get_projects(
    State(service): State<Arc<ProjectServiceType>>,
) -> Json<Vec<ProjectResponse>> {
    let projects = service.get_visible_projects().await.unwrap_or_default();
    Json(projects.into_iter().map(ProjectResponse::from).collect())
}

#[utoipa::path(
    get,
    path = "/api/projects/{id}",
    params(
        ("id" = String, Path, description = "UUID проекта")
    ),
    responses(
        (status = 200, description = "Проект найден", body = ProjectResponse),
        (status = 404, description = "Проект не найден")
    ),
    tag = "projects"
)]
async fn get_project(
    State(service): State<Arc<ProjectServiceType>>,
    Path(id): Path<String>,
) -> Result<Json<ProjectResponse>, String> {
    let project = service
        .get_project(&id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Project not found".to_string())?;

    Ok(Json(project.into()))
}

#[utoipa::path(
    post,
    path = "/api/projects",
    request_body = CreateProjectRequest,
    responses(
        (status = 200, description = "Проект создан", body = ProjectResponse),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "projects"
)]
async fn create_project(
    State(service): State<Arc<ProjectServiceType>>,
    Json(payload): Json<CreateProjectRequest>,
) -> Result<Json<ProjectResponse>, String> {
    let technologies =
        UniqueTechnologies::from_vec(payload.technologies).map_err(|e| e.to_string())?;

    let project = Project::new(
        payload.name,
        payload.description,
        payload.long_description,
        payload.status,
        technologies,
    );

    let project = service
        .create_project(project)
        .await
        .map_err(|e| e.to_string())?;

    Ok(Json(project.into()))
}

#[utoipa::path(
    get,
    path = "/api/projects/featured",
    responses(
        (status = 200, description = "Список избранных проектов", body = [ProjectResponse])
    ),
    tag = "projects"
)]
async fn get_featured_projects(
    State(service): State<Arc<ProjectServiceType>>,
) -> Json<Vec<ProjectResponse>> {
    let projects = service.get_featured_projects().await.unwrap_or_default();
    Json(projects.into_iter().map(ProjectResponse::from).collect())
}

#[utoipa::path(
    put,
    path = "/api/projects/{id}/name",
    params(
        ("id" = String, Path, description = "UUID проекта")
    ),
    request_body = UpdateProjectNameRequest,
    responses(
        (status = 200, description = "Имя проекта обновлено", body = ProjectResponse),
        (status = 404, description = "Проект не найден"),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "projects"
)]
async fn update_project_name(
    State(service): State<Arc<ProjectServiceType>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateProjectNameRequest>,
) -> Result<Json<ProjectResponse>, String> {
    let project = service
        .update_project_name(&id, payload.name)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(project.into()))
}

#[utoipa::path(
    put,
    path = "/api/projects/{id}/featured",
    params(
        ("id" = String, Path, description = "UUID проекта")
    ),
    request_body = UpdateProjectFeaturedRequest,
    responses(
        (status = 200, description = "Статус избранного обновлен", body = ProjectResponse),
        (status = 404, description = "Проект не найден"),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "projects"
)]
async fn update_project_featured(
    State(service): State<Arc<ProjectServiceType>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateProjectFeaturedRequest>,
) -> Result<Json<ProjectResponse>, String> {
    let project = service
        .update_project_featured(&id, payload.featured)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(project.into()))
}

#[utoipa::path(
    put,
    path = "/api/projects/{id}/status",
    params(
        ("id" = String, Path, description = "UUID проекта")
    ),
    request_body = UpdateProjectStatusRequest,
    responses(
        (status = 200, description = "Статус проекта обновлен", body = ProjectResponse),
        (status = 404, description = "Проект не найден"),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "projects"
)]
async fn update_project_status(
    State(service): State<Arc<ProjectServiceType>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateProjectStatusRequest>,
) -> Result<Json<ProjectResponse>, String> {
    let project = service
        .update_project_status(&id, payload.status)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(project.into()))
}

#[utoipa::path(
    post,
    path = "/api/projects/{id}/technologies",
    params(
        ("id" = String, Path, description = "UUID проекта")
    ),
    request_body = AddProjectTechnologyRequest,
    responses(
        (status = 200, description = "Технология добавлена", body = ProjectResponse),
        (status = 404, description = "Проект не найден"),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "projects"
)]
async fn add_project_technology(
    State(service): State<Arc<ProjectServiceType>>,
    Path(id): Path<String>,
    Json(payload): Json<AddProjectTechnologyRequest>,
) -> Result<Json<ProjectResponse>, String> {
    let project = service
        .add_project_technology(&id, payload.technology)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(project.into()))
}

#[utoipa::path(
    delete,
    path = "/api/projects/{id}",
    params(
        ("id" = String, Path, description = "UUID проекта")
    ),
    responses(
        (status = 200, description = "Проект удален", body = MessageResponse),
        (status = 404, description = "Проект не найден")
    ),
    tag = "projects"
)]
async fn delete_project(
    State(service): State<Arc<ProjectServiceType>>,
    Path(id): Path<String>,
) -> Result<Json<MessageResponse>, String> {
    service
        .delete_project(&id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(Json(MessageResponse {
        message: "Project deleted successfully".to_string(),
    }))
}

#[utoipa::path(
    delete,
    path = "/api/projects/batch",
    request_body = DeleteProjectsBatchRequest,
    responses(
        (status = 200, description = "Пакетное удаление завершено", body = DeleteBatchResponse),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "projects"
)]
async fn delete_projects_batch(
    State(service): State<Arc<ProjectServiceType>>,
    Json(payload): Json<DeleteProjectsBatchRequest>,
) -> Result<Json<DeleteBatchResponse>, String> {
    let mut results = Vec::new();

    for project_id in payload.project_ids {
        match service.delete_project(&project_id).await {
            Ok(()) => results.push((project_id, "deleted".to_string())),
            Err(e) => results.push((project_id, format!("error: {}", e))),
        }
    }

    Ok(Json(DeleteBatchResponse { results }))
}

#[utoipa::path(
    put,
    path = "/api/projects/{id}/description",
    params(
        ("id" = String, Path, description = "UUID проекта")
    ),
    request_body = UpdateProjectDescriptionRequest,
    responses(
        (status = 200, description = "Описание проекта обновлено", body = ProjectResponse),
        (status = 404, description = "Проект не найден"),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "projects"
)]
async fn update_project_description(
    State(service): State<Arc<ProjectServiceType>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateProjectDescriptionRequest>,
) -> Result<Json<ProjectResponse>, String> {
    let project = service
        .update_project_description(&id, payload.description)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(project.into()))
}

#[utoipa::path(
    put,
    path = "/api/projects/{id}/long-description",
    params(
        ("id" = String, Path, description = "UUID проекта")
    ),
    request_body = UpdateProjectLongDescriptionRequest,
    responses(
        (status = 200, description = "Длинное описание проекта обновлено", body = ProjectResponse),
        (status = 404, description = "Проект не найден"),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "projects"
)]
async fn update_project_long_description(
    State(service): State<Arc<ProjectServiceType>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateProjectLongDescriptionRequest>,
) -> Result<Json<ProjectResponse>, String> {
    let project = service
        .update_project_long_description(&id, payload.long_description)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(project.into()))
}

#[utoipa::path(
    put,
    path = "/api/projects/{id}/visibility",
    params(
        ("id" = String, Path, description = "UUID проекта")
    ),
    request_body = UpdateProjectVisibilityRequest,
    responses(
        (status = 200, description = "Видимость проекта обновлена", body = ProjectResponse),
        (status = 404, description = "Проект не найден"),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "projects"
)]
async fn update_project_visibility(
    State(service): State<Arc<ProjectServiceType>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateProjectVisibilityRequest>,
) -> Result<Json<ProjectResponse>, String> {
    let project = service
        .update_project_visibility(&id, payload.visibility)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(project.into()))
}

#[utoipa::path(
    put,
    path = "/api/projects/{id}/dates",
    params(
        ("id" = String, Path, description = "UUID проекта")
    ),
    request_body = UpdateProjectDatesRequest,
    responses(
        (status = 200, description = "Даты проекта обновлены", body = ProjectResponse),
        (status = 404, description = "Проект не найден"),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "projects"
)]
async fn update_project_dates(
    State(service): State<Arc<ProjectServiceType>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateProjectDatesRequest>,
) -> Result<Json<ProjectResponse>, String> {
    let project = service
        .update_project_dates(&id, payload.start_date, payload.end_date)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(project.into()))
}

#[utoipa::path(
    put,
    path = "/api/projects/{id}/image",
    params(
        ("id" = String, Path, description = "UUID проекта")
    ),
    request_body = UpdateProjectImageRequest,
    responses(
        (status = 200, description = "Изображение проекта обновлено", body = ProjectResponse),
        (status = 404, description = "Проект не найден"),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "projects"
)]
async fn update_project_image(
    State(service): State<Arc<ProjectServiceType>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateProjectImageRequest>,
) -> Result<Json<ProjectResponse>, String> {
    let project = service
        .update_project_image(&id, payload.image)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(project.into()))
}

#[utoipa::path(
    delete,
    path = "/api/projects/{id}/image",
    params(
        ("id" = String, Path, description = "UUID проекта")
    ),
    responses(
        (status = 200, description = "Изображение проекта удалено", body = ProjectResponse),
        (status = 404, description = "Проект не найден")
    ),
    tag = "projects"
)]
async fn remove_project_image(
    State(service): State<Arc<ProjectServiceType>>,
    Path(id): Path<String>,
) -> Result<Json<ProjectResponse>, String> {
    let project = service
        .update_project_image(&id, None)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(project.into()))
}

#[utoipa::path(
    delete,
    path = "/api/projects/{id}/technologies",
    params(
        ("id" = String, Path, description = "UUID проекта")
    ),
    request_body = RemoveProjectTechnologyRequest,
    responses(
        (status = 200, description = "Технология удалена из проекта", body = ProjectResponse),
        (status = 404, description = "Проект не найден"),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "projects"
)]
async fn remove_project_technology(
    State(service): State<Arc<ProjectServiceType>>,
    Path(id): Path<String>,
    Json(payload): Json<RemoveProjectTechnologyRequest>,
) -> Result<Json<ProjectResponse>, String> {
    let project = service
        .remove_project_technology(&id, payload.technology)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(project.into()))
}

#[utoipa::path(
    post,
    path = "/api/projects/{id}/urls",
    params(
        ("id" = String, Path, description = "UUID проекта")
    ),
    request_body = AddProjectUrlRequest,
    responses(
        (status = 200, description = "URL добавлен к проекту", body = ProjectResponse),
        (status = 404, description = "Проект не найден"),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "projects"
)]
async fn add_project_url(
    State(service): State<Arc<ProjectServiceType>>,
    Path(id): Path<String>,
    Json(payload): Json<AddProjectUrlRequest>,
) -> Result<Json<ProjectResponse>, String> {
    let url = ProjectUrl::new(payload.url_type, payload.url, None);
    let project = service
        .add_project_url(&id, url)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(project.into()))
}

#[utoipa::path(
    delete,
    path = "/api/projects/{id}/urls",
    params(
        ("id" = String, Path, description = "UUID проекта")
    ),
    request_body = RemoveProjectUrlRequest,
    responses(
        (status = 200, description = "URL удален из проекта", body = ProjectResponse),
        (status = 404, description = "Проект не найден"),
        (status = 400, description = "Ошибка валидации")
    ),
    tag = "projects"
)]
async fn remove_project_url(
    State(service): State<Arc<ProjectServiceType>>,
    Path(id): Path<String>,
    Json(payload): Json<RemoveProjectUrlRequest>,
) -> Result<Json<ProjectResponse>, String> {
    let project = service
        .remove_project_url(&id, payload.url)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(project.into()))
}
