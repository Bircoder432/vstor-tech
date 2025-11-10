use crate::domain::entities::Project;
use crate::domain::types::ProjectStatus;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ProjectResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: String,
    #[schema(example = "My Portfolio")]
    pub name: String,
    #[schema(example = "Personal developer portfolio")]
    pub description: String,
    pub status: ProjectStatus,
    #[schema(example = json!(["Rust", "Leptos"]))]
    pub technologies: Vec<String>,
    #[schema(example = false)]
    pub featured: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DeleteBatchResponse {
    pub results: Vec<(String, String)>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
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
