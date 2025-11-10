use crate::backend::repositories::SkillRepository;
use crate::domain::entities::Skill;
use crate::domain::errors::DomainError;
use crate::domain::types::{SkillCategory, SkillLevel};
use async_trait::async_trait;

pub struct SkillService<R: SkillRepository> {
    repo: R,
}

impl<R: SkillRepository> SkillService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_skill(&self, skill: Skill) -> Result<Skill, DomainError> {
        skill.validate()?;

        let skill = self
            .repo
            .save(skill)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))?;

        Ok(skill)
    }

    pub async fn get_skill(&self, id: &str) -> Result<Option<Skill>, DomainError> {
        self.repo
            .find_by_id(id)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))
    }

    pub async fn get_all_skills(&self) -> Result<Vec<Skill>, DomainError> {
        self.repo
            .find_all()
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))
    }

    pub async fn get_skills_by_category(
        &self,
        category: SkillCategory,
    ) -> Result<Vec<Skill>, DomainError> {
        self.repo
            .find_by_category(category)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))
    }

    pub async fn update_skill_experience(
        &self,
        id: &str,
        new_experience: u8,
    ) -> Result<Skill, DomainError> {
        let mut skill = self
            .repo
            .find_by_id(id)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))?
            .ok_or_else(|| DomainError::BusinessRule("Skill not found".to_string()))?;

        skill.set_years_of_experience(new_experience)?;

        let skill = self
            .repo
            .save(skill)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))?;

        Ok(skill)
    }

    pub async fn update_skill_level(
        &self,
        id: &str,
        new_level: SkillLevel,
    ) -> Result<Skill, DomainError> {
        let mut skill = self
            .repo
            .find_by_id(id)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))?
            .ok_or_else(|| DomainError::BusinessRule("Skill not found".to_string()))?;

        //TODO: make removing skill
        let skill = self
            .repo
            .save(skill)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))?;

        Ok(skill)
    }

    pub async fn delete_skill(&self, id: &str) -> Result<(), DomainError> {
        self.repo
            .delete(id)
            .await
            .map_err(|e| DomainError::BusinessRule(e.to_string()))
    }
}

impl<R: SkillRepository + Clone> Clone for SkillService<R> {
    fn clone(&self) -> Self {
        Self {
            repo: self.repo.clone(),
        }
    }
}
