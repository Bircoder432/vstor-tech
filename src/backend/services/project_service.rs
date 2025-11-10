// backend/services/project_service.rs
use crate::backend::repositories::ProjectRepository;
use crate::domain::entities::Project;
use crate::domain::errors::DomainError;
use crate::domain::types::{ContentVisibility, ProjectStatus};
use crate::shared::types::{ProjectUrl, UrlType};

pub struct ProjectService<R: ProjectRepository> {
    repo: R,
}

impl<R: ProjectRepository> ProjectService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_project(&self, project: Project) -> Result<Project, DomainError> {
        project.validate()?;

        let project = self
            .repo
            .save(project)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))?;

        Ok(project)
    }

    pub async fn get_project(&self, id: &str) -> Result<Option<Project>, DomainError> {
        self.repo
            .find_by_id(id)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))
    }

    pub async fn get_all_projects(&self) -> Result<Vec<Project>, DomainError> {
        self.repo
            .find_all()
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))
    }

    pub async fn get_visible_projects(&self) -> Result<Vec<Project>, DomainError> {
        self.repo
            .find_visible()
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))
    }

    pub async fn get_featured_projects(&self) -> Result<Vec<Project>, DomainError> {
        self.repo
            .find_featured()
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))
    }

    pub async fn get_projects_by_status(
        &self,
        status: ProjectStatus,
    ) -> Result<Vec<Project>, DomainError> {
        self.repo
            .find_by_status(status)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))
    }

    pub async fn add_project_url(
        &self,
        project_id: &str,
        url: ProjectUrl,
    ) -> Result<Project, DomainError> {
        let mut project = self
            .repo
            .find_by_id(project_id)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))?
            .ok_or_else(|| DomainError::BusinessRule("Project not found".to_string()))?;

        project.add_url(url)?;
        project.validate()?;

        let project = self
            .repo
            .save(project)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))?;

        Ok(project)
    }

    pub async fn update_project_status(
        &self,
        project_id: &str,
        new_status: ProjectStatus,
    ) -> Result<Project, DomainError> {
        let mut project = self
            .repo
            .find_by_id(project_id)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))?
            .ok_or_else(|| DomainError::BusinessRule("Project not found".to_string()))?;

        // В домене должен быть метод для смены статуса с валидацией
        // project.set_status(new_status)?;
        project.validate()?;

        let project = self
            .repo
            .save(project)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))?;

        Ok(project)
    }

    pub async fn feature_project(&self, project_id: &str) -> Result<Project, DomainError> {
        let mut project = self
            .repo
            .find_by_id(project_id)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))?
            .ok_or_else(|| DomainError::BusinessRule("Project not found".to_string()))?;

        // Бизнес-логика featured
        if !project.is_visible() {
            return Err(DomainError::BusinessRule(
                "Only public projects can be featured".to_string(),
            ));
        }

        // Нужно добавить метод в Project для установки featured с валидацией
        // project.set_featured(true)?;
        project.validate()?;

        let project = self
            .repo
            .save(project)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))?;

        Ok(project)
    }

    pub async fn unfeature_project(&self, project_id: &str) -> Result<Project, DomainError> {
        let mut project = self
            .repo
            .find_by_id(project_id)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))?
            .ok_or_else(|| DomainError::BusinessRule("Project not found".to_string()))?;

        // project.set_featured(false)?;
        project.validate()?;

        let project = self
            .repo
            .save(project)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))?;

        Ok(project)
    }

    pub async fn delete_project(&self, id: &str) -> Result<(), DomainError> {
        self.repo
            .delete(id)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))
    }
}

impl<R: ProjectRepository + Clone> Clone for ProjectService<R> {
    fn clone(&self) -> Self {
        Self {
            repo: self.repo.clone(),
        }
    }
}
