use crate::backend::errors::DbError;
use crate::backend::repositories::SkillRepository;
use crate::domain::entities::Skill;
use crate::domain::types::SkillCategory;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct InMemorySkillRepo {
    skills: Arc<RwLock<HashMap<String, Skill>>>,
}

impl InMemorySkillRepo {
    pub fn new() -> Self {
        Self {
            skills: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl SkillRepository for InMemorySkillRepo {
    async fn save(&self, skill: Skill) -> Result<Skill, DbError> {
        let id = skill.id().to_string();
        let mut skills = self
            .skills
            .write()
            .map_err(|e| DbError::Query(e.to_string()))?;

        skills.insert(id, skill.clone());
        Ok(skill)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Skill>, DbError> {
        let skills = self
            .skills
            .read()
            .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(skills.get(id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<Skill>, DbError> {
        let skills = self
            .skills
            .read()
            .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(skills.values().cloned().collect())
    }

    async fn delete(&self, id: &str) -> Result<(), DbError> {
        let mut skills = self
            .skills
            .write()
            .map_err(|e| DbError::Query(e.to_string()))?;

        skills.remove(id);
        Ok(())
    }

    async fn find_by_category(&self, category: SkillCategory) -> Result<Vec<Skill>, DbError> {
        let skills = self
            .skills
            .read()
            .map_err(|e| DbError::Query(e.to_string()))?;

        let result: Vec<Skill> = skills
            .values()
            .filter(|skill| skill.category() == &category)
            .cloned()
            .collect();

        Ok(result)
    }
}
