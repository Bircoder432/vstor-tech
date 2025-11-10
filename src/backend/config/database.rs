use crate::backend::repositories::{
    InMemoryProjectRepo, InMemorySkillRepo, ProjectRepository, SkillRepository,
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum DatabaseType {
    InMemory,
    Spacetime,
    Postgres,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub db_type: DatabaseType,
    pub url: String,
}

impl DatabaseConfig {
    pub fn create_skill_repo(&self) -> Box<dyn SkillRepository> {
        match self.db_type {
            DatabaseType::InMemory => Box::new(InMemorySkillRepo::new()),
            DatabaseType::Spacetime => {
                // TODO: Реализовать когда будет Spacetime репозиторий
                println!("⚠️  Spacetime DB not implemented yet, using InMemory");
                Box::new(InMemorySkillRepo::new())
            }
            DatabaseType::Postgres => {
                // TODO: Реализовать когда будет Postgres репозиторий
                println!("⚠️  Postgres DB not implemented yet, using InMemory");
                Box::new(InMemorySkillRepo::new())
            }
        }
    }

    pub fn create_project_repo(&self) -> Box<dyn ProjectRepository> {
        match self.db_type {
            DatabaseType::InMemory => Box::new(InMemoryProjectRepo::new()),
            DatabaseType::Spacetime => {
                println!("⚠️  Spacetime DB not implemented yet, using InMemory");
                Box::new(InMemoryProjectRepo::new())
            }
            DatabaseType::Postgres => {
                println!("⚠️  Postgres DB not implemented yet, using InMemory");
                Box::new(InMemoryProjectRepo::new())
            }
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            db_type: DatabaseType::InMemory,
            url: "memory".to_string(),
        }
    }
}
