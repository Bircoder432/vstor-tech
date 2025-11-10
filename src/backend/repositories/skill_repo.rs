use crate::backend::errors::DbError;
use crate::domain::entities::Skill;
use crate::domain::types::SkillCategory;
use async_trait::async_trait;

#[async_trait]
pub trait SkillRepository: Send + Sync {
    async fn save(&self, skill: Skill) -> Result<Skill, DbError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Skill>, DbError>;
    async fn find_all(&self) -> Result<Vec<Skill>, DbError>;
    async fn delete(&self, id: &str) -> Result<(), DbError>;
    async fn find_by_category(&self, category: SkillCategory) -> Result<Vec<Skill>, DbError>;
}
