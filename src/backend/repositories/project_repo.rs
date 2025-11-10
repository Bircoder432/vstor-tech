// backend/repositories/project_repo.rs
use crate::backend::errors::DbError;
use crate::domain::entities::Project;
use crate::domain::types::{ContentVisibility, ProjectStatus};
use async_trait::async_trait;

#[async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn save(&self, project: Project) -> Result<Project, DbError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Project>, DbError>;
    async fn find_all(&self) -> Result<Vec<Project>, DbError>;
    async fn find_visible(&self) -> Result<Vec<Project>, DbError>;
    async fn find_featured(&self) -> Result<Vec<Project>, DbError>;
    async fn find_by_status(&self, status: ProjectStatus) -> Result<Vec<Project>, DbError>;
    async fn delete(&self, id: &str) -> Result<(), DbError>;
}
