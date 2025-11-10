use crate::backend::errors::DbError;
use crate::backend::repositories::ProjectRepository;
use crate::domain::entities::Project;
use crate::domain::types::{ContentVisibility, ProjectStatus};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct InMemoryProjectRepo {
    projects: Arc<RwLock<HashMap<String, Project>>>,
}

impl InMemoryProjectRepo {
    pub fn new() -> Self {
        Self {
            projects: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl ProjectRepository for InMemoryProjectRepo {
    async fn save(&self, project: Project) -> Result<Project, DbError> {
        let id = project.id().to_string();
        let mut projects = self
            .projects
            .write()
            .map_err(|e| DbError::Query(e.to_string()))?;

        projects.insert(id, project.clone());
        Ok(project)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Project>, DbError> {
        let projects = self
            .projects
            .read()
            .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(projects.get(id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<Project>, DbError> {
        let projects = self
            .projects
            .read()
            .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(projects.values().cloned().collect())
    }

    async fn find_visible(&self) -> Result<Vec<Project>, DbError> {
        let projects = self
            .projects
            .read()
            .map_err(|e| DbError::Query(e.to_string()))?;

        let result: Vec<Project> = projects
            .values()
            .filter(|project| project.is_visible())
            .cloned()
            .collect();

        Ok(result)
    }

    async fn find_featured(&self) -> Result<Vec<Project>, DbError> {
        let projects = self
            .projects
            .read()
            .map_err(|e| DbError::Query(e.to_string()))?;

        let result: Vec<Project> = projects
            .values()
            .filter(|project| project.featured())
            .cloned()
            .collect();

        Ok(result)
    }

    async fn find_by_status(&self, status: ProjectStatus) -> Result<Vec<Project>, DbError> {
        let projects = self
            .projects
            .read()
            .map_err(|e| DbError::Query(e.to_string()))?;

        let result: Vec<Project> = projects
            .values()
            .filter(|project| project.status() == &status)
            .cloned()
            .collect();

        Ok(result)
    }

    async fn delete(&self, id: &str) -> Result<(), DbError> {
        let mut projects = self
            .projects
            .write()
            .map_err(|e| DbError::Query(e.to_string()))?;

        projects.remove(id);
        Ok(())
    }
}
