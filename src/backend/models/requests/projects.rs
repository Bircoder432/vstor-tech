use crate::domain::types::ProjectStatus;
use crate::shared::types::ImageSource;
use crate::shared::types::ProjectUrl;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProjectRequest {
    #[schema(example = "My Portfolio")]
    pub name: String,
    #[schema(example = "Personal developer portfolio")]
    pub description: String,
    #[schema(example = "Built with Rust and Leptos")]
    pub long_description: String,
    pub status: ProjectStatus,
    #[schema(example = json!(["Rust", "Leptos"]))]
    pub technologies: Vec<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProjectNameRequest {
    #[schema(example = "Updated Portfolio Name")]
    pub name: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProjectFeaturedRequest {
    #[schema(example = true)]
    pub featured: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProjectStatusRequest {
    pub status: ProjectStatus,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddProjectTechnologyRequest {
    #[schema(example = "TypeScript")]
    pub technology: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DeleteProjectsBatchRequest {
    #[schema(example = json!(["id1", "id2"]))]
    pub project_ids: Vec<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddProjectUrlRequest {
    #[schema(example = "https://github.com/user/repo")]
    pub url: String,
    #[schema(example = "GitHub")]
    pub url_type: crate::shared::types::UrlType,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SetProjectImageRequest {
    #[schema(example = "https://example.com/image.jpg")]
    pub image_url: String,
    #[schema(example = "thumbnail")]
    pub image_type: crate::shared::types::ImageSource,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProjectImageRequest {
    pub image: Option<ImageSource>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProjectDescriptionRequest {
    #[schema(example = "Updated project description")]
    pub description: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProjectLongDescriptionRequest {
    #[schema(example = "Updated long project description")]
    pub long_description: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProjectVisibilityRequest {
    pub visibility: crate::domain::types::ContentVisibility,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProjectDatesRequest {
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RemoveProjectTechnologyRequest {
    #[schema(example = "OldTechnology")]
    pub technology: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RemoveProjectUrlRequest {
    pub url: ProjectUrl,
}
