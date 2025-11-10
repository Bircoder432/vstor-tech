use crate::backend::repositories::ProjectRepository;
use crate::domain::entities::Project;
use crate::domain::errors::DomainError;
use crate::domain::types::{ContentVisibility, ProjectStatus};
use crate::shared::types::{ImageSource, ProjectUrl, UrlType};

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

        if !project.is_visible() {
            return Err(DomainError::BusinessRule(
                "Only public projects can be featured".to_string(),
            ));
        }

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
    pub async fn update_project_name(
        &self,
        id: &str,
        new_name: String,
    ) -> Result<Project, DomainError> {
        let mut project = self.get_project_by_id(id).await?;
        project.set_name(new_name)?;
        self.repo
            .save(project)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))
    }

    pub async fn update_project_image(
        &self,
        id: &str,
        image: Option<ImageSource>,
    ) -> Result<Project, DomainError> {
        let mut project = self.get_project_by_id(id).await?;
        project.set_image(image);
        self.repo
            .save(project)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))
    }

    pub async fn remove_project_url(
        &self,
        project_id: &str,
        url: ProjectUrl,
    ) -> Result<Project, DomainError> {
        let mut project = self.get_project_by_id(project_id).await?;
        project.remove_url(url)?;
        project.validate()?;
        self.repo
            .save(project)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))
    }

    pub async fn update_project_featured(
        &self,
        id: &str,
        featured: bool,
    ) -> Result<Project, DomainError> {
        let mut project = self.get_project_by_id(id).await?;
        project.set_featured(featured)?;
        self.repo
            .save(project)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))
    }

    pub async fn update_project_description(
        &self,
        id: &str,
        description: String,
    ) -> Result<Project, DomainError> {
        let mut project = self.get_project_by_id(id).await?;
        project.set_description(description)?;
        self.repo
            .save(project)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))
    }

    pub async fn update_project_long_description(
        &self,
        id: &str,
        long_description: String,
    ) -> Result<Project, DomainError> {
        let mut project = self.get_project_by_id(id).await?;
        project.set_long_description(long_description)?;
        self.repo
            .save(project)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))
    }

    pub async fn update_project_visibility(
        &self,
        id: &str,
        visibility: ContentVisibility,
    ) -> Result<Project, DomainError> {
        let mut project = self.get_project_by_id(id).await?;
        project.set_visibility(visibility)?;
        self.repo
            .save(project)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))
    }

    pub async fn update_project_dates(
        &self,
        id: &str,
        start_date: Option<chrono::NaiveDate>,
        end_date: Option<chrono::NaiveDate>,
    ) -> Result<Project, DomainError> {
        let mut project = self.get_project_by_id(id).await?;
        project.set_dates(start_date, end_date)?;
        self.repo
            .save(project)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))
    }

    pub async fn remove_project_technology(
        &self,
        id: &str,
        tech: String,
    ) -> Result<Project, DomainError> {
        let mut project = self.get_project_by_id(id).await?;
        project.remove_technology(&tech)?;
        self.repo
            .save(project)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))
    }

    pub async fn add_project_technology(
        &self,
        id: &str,
        tech: String,
    ) -> Result<Project, DomainError> {
        let mut project = self.get_project_by_id(id).await?;
        project.add_technology(tech)?;
        self.repo
            .save(project)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))
    }

    async fn get_project_by_id(&self, id: &str) -> Result<Project, DomainError> {
        self.repo
            .find_by_id(id)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))?
            .ok_or_else(|| DomainError::BusinessRule("Project not found".to_string()))
    }
}

impl<R: ProjectRepository + Clone> Clone for ProjectService<R> {
    fn clone(&self) -> Self {
        Self {
            repo: self.repo.clone(),
        }
    }
}
