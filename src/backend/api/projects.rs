// backend/api/projects.rs
use crate::backend::repositories::ProjectRepository;
use crate::backend::services::ProjectService;
use crate::domain::entities::Project;
use crate::domain::types::{ProjectStatus, UniqueTechnologies};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post, put},
};

#[derive(Debug, serde::Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: String,
    pub long_description: String,
    pub status: ProjectStatus,
    pub technologies: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct ProjectResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: ProjectStatus,
    pub technologies: Vec<String>,
    pub featured: bool,
}

impl From<Project> for ProjectResponse {
    fn from(project: Project) -> Self {
        Self {
            id: project.id().to_string(),
            name: project.name().to_string(),
            description: project.description().to_string(),
            status: project.status().clone(),
            technologies: project.technologies().as_vec(),
            featured: project.featured(),
        }
    }
}

pub fn projects_router<R>() -> Router<ProjectService<R>>
where
    R: ProjectRepository + Clone + Send + Sync + 'static,
    ProjectService<R>: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(get_projects::<R>).post(create_project::<R>))
        .route("/{id}", get(get_project::<R>))
        .route("/featured", get(get_featured_projects::<R>))
}

// Handlers
async fn get_projects<R: ProjectRepository>(
    State(service): State<ProjectService<R>>,
) -> Json<Vec<ProjectResponse>> {
    let projects = service.get_visible_projects().await.unwrap_or_default();
    Json(projects.into_iter().map(ProjectResponse::from).collect())
}

async fn get_project<R: ProjectRepository>(
    State(service): State<ProjectService<R>>,
    Path(id): Path<String>,
) -> Result<Json<ProjectResponse>, String> {
    let project = service
        .get_project(&id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Project not found".to_string())?;

    Ok(Json(project.into()))
}

async fn create_project<R: ProjectRepository>(
    State(service): State<ProjectService<R>>,
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

async fn get_featured_projects<R: ProjectRepository>(
    State(service): State<ProjectService<R>>,
) -> Json<Vec<ProjectResponse>> {
    let projects = service.get_featured_projects().await.unwrap_or_default();
    Json(projects.into_iter().map(ProjectResponse::from).collect())
}
