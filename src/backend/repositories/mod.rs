pub mod inmemory;
pub mod project_repo;
pub mod skill_repo;

pub use inmemory::{InMemoryProjectRepo, InMemorySkillRepo};
pub use project_repo::ProjectRepository;
pub use skill_repo::SkillRepository;

#[async_trait::async_trait]
impl<T: SkillRepository + ?Sized + Send + Sync> SkillRepository for Box<T> {
    async fn save(
        &self,
        skill: crate::domain::entities::Skill,
    ) -> Result<crate::domain::entities::Skill, crate::backend::errors::DbError> {
        self.as_ref().save(skill).await
    }

    async fn find_by_id(
        &self,
        id: &str,
    ) -> Result<Option<crate::domain::entities::Skill>, crate::backend::errors::DbError> {
        self.as_ref().find_by_id(id).await
    }

    async fn find_all(
        &self,
    ) -> Result<Vec<crate::domain::entities::Skill>, crate::backend::errors::DbError> {
        self.as_ref().find_all().await
    }

    async fn delete(&self, id: &str) -> Result<(), crate::backend::errors::DbError> {
        self.as_ref().delete(id).await
    }

    async fn find_by_category(
        &self,
        category: crate::domain::types::SkillCategory,
    ) -> Result<Vec<crate::domain::entities::Skill>, crate::backend::errors::DbError> {
        self.as_ref().find_by_category(category).await
    }
}

#[async_trait::async_trait]
impl<T: ProjectRepository + ?Sized + Send + Sync> ProjectRepository for Box<T> {
    async fn save(
        &self,
        project: crate::domain::entities::Project,
    ) -> Result<crate::domain::entities::Project, crate::backend::errors::DbError> {
        self.as_ref().save(project).await
    }

    async fn find_by_id(
        &self,
        id: &str,
    ) -> Result<Option<crate::domain::entities::Project>, crate::backend::errors::DbError> {
        self.as_ref().find_by_id(id).await
    }

    async fn find_all(
        &self,
    ) -> Result<Vec<crate::domain::entities::Project>, crate::backend::errors::DbError> {
        self.as_ref().find_all().await
    }

    async fn find_visible(
        &self,
    ) -> Result<Vec<crate::domain::entities::Project>, crate::backend::errors::DbError> {
        self.as_ref().find_visible().await
    }

    async fn find_featured(
        &self,
    ) -> Result<Vec<crate::domain::entities::Project>, crate::backend::errors::DbError> {
        self.as_ref().find_featured().await
    }

    async fn find_by_status(
        &self,
        status: crate::domain::types::ProjectStatus,
    ) -> Result<Vec<crate::domain::entities::Project>, crate::backend::errors::DbError> {
        self.as_ref().find_by_status(status).await
    }

    async fn delete(&self, id: &str) -> Result<(), crate::backend::errors::DbError> {
        self.as_ref().delete(id).await
    }
}
