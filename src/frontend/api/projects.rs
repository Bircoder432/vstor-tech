// src/frontend/api/projects.rs
use super::fetch_api;
use crate::backend::models::responses::projects::ProjectResponse;

pub async fn get_projects() -> Result<Vec<ProjectResponse>, crate::frontend::errors::FrontendError>
{
    fetch_api("/projects").await
}

pub async fn get_featured_projects()
-> Result<Vec<ProjectResponse>, crate::frontend::errors::FrontendError> {
    fetch_api("/projects/featured").await
}

pub async fn get_project(
    _id: &str,
) -> Result<ProjectResponse, crate::frontend::errors::FrontendError> {
    fetch_api("/projects/1").await
}
