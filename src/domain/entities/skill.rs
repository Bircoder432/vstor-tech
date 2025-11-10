use crate::domain::entities::create_up::CreateUp;
use crate::domain::errors::DomainError::{self, BusinessRule};
use crate::domain::types::{ContentVisibility, SkillCategory, SkillLevel};
use crate::shared::types::ImageSource;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    id: String,
    name: String,
    category: SkillCategory,
    level: SkillLevel,
    description: String,
    years_of_experience: u8,
    visibility: ContentVisibility,
    image: Option<ImageSource>,
    created_updated: CreateUp,
}

impl Skill {
    pub fn new(
        name: String,
        category: SkillCategory,
        description: String,
        years_of_experience: u8,
    ) -> Self {
        let level: SkillLevel = SkillLevel::from_experience(years_of_experience);
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            category,
            level,
            description,
            years_of_experience,
            visibility: ContentVisibility::Public,
            image: None,
            created_updated: CreateUp::new(),
        }
    }

    pub fn with_dates(
        id: String,
        name: String,
        category: SkillCategory,
        description: String,
        years_of_experience: u8,
        visibility: ContentVisibility,
        image: Option<ImageSource>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        let level: SkillLevel = SkillLevel::from_experience(years_of_experience);
        Self {
            id,
            name,
            category,
            level,
            description,
            years_of_experience,
            visibility,
            image,
            created_updated: CreateUp::with_dates(created_at, updated_at),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn category(&self) -> &SkillCategory {
        &self.category
    }
    pub fn level(&self) -> &SkillLevel {
        &self.level
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn years_of_experience(&self) -> u8 {
        self.years_of_experience
    }
    pub fn image(&self) -> Option<&ImageSource> {
        self.image.as_ref()
    }
    pub fn visibility(&self) -> &ContentVisibility {
        &self.visibility
    }
    pub fn created_updated(&self) -> &CreateUp {
        &self.created_updated
    }

    pub fn set_name(&mut self, name: String) -> Result<(), DomainError> {
        if name.trim().is_empty() {
            return Err(BusinessRule("Skill name cannot be empty".to_string()));
        }
        self.name = name;
        self.created_updated.update();
        Ok(())
    }

    pub fn set_image(&mut self, image: Option<ImageSource>) -> Result<(), DomainError> {
        self.image = image;
        self.created_updated.update();
        Ok(())
    }

    pub fn set_level(&mut self, new_level: SkillLevel) -> Result<(), DomainError> {
        if self.level > new_level {
            return Err(DomainError::BusinessRule(
                "Cannot downgrade skill level without review".into(),
            ));
        }
        self.level = new_level;
        Ok(())
    }

    pub fn update_description(&mut self, description: String) -> Result<(), DomainError> {
        self.description = description;
        self.created_updated.update();
        self.validate()?;
        Ok(())
    }

    pub fn set_category(&mut self, category: SkillCategory) -> Result<(), DomainError> {
        self.category = category;
        self.created_updated.update();
        Ok(())
    }

    pub fn set_visibility(&mut self, visibility: ContentVisibility) -> Result<(), DomainError> {
        self.visibility = visibility;
        self.created_updated.update();
        self.validate()?;
        Ok(())
    }

    pub fn set_years_of_experience(&mut self, years: u8) -> Result<(), DomainError> {
        if years < 0 {
            return Err(BusinessRule(
                "Years of experience cannot be negative".to_string(),
            ));
        }
        self.years_of_experience = years;
        self.calculate_level_auto();
        self.created_updated.update();
        Ok(())
    }

    fn calculate_level_auto(&mut self) {
        self.level = match self.years_of_experience {
            0..=1 => SkillLevel::Beginner,
            2..=3 => SkillLevel::Intermediate,
            4..=6 => SkillLevel::Advanced,
            _ => SkillLevel::Expert,
        };
    }

    fn validate_name(&self) -> Result<(), DomainError> {
        let name = self.name.trim();

        if name.is_empty() {
            return Err(BusinessRule("Skill name cannot be empty".to_string()));
        }

        if name.len() > 50 {
            return Err(BusinessRule(
                "Skill name cannot exceed 50 characters".to_string(),
            ));
        }

        if name.chars().all(|c| c.is_whitespace()) {
            return Err(BusinessRule(
                "Skill name cannot consist only of whitespace".to_string(),
            ));
        }

        let forbidden_chars = ['<', '>', '&', '"', '\''];
        if name.chars().any(|c| forbidden_chars.contains(&c)) {
            return Err(BusinessRule(
                "Skill name contains invalid characters".to_string(),
            ));
        }

        if name.contains("  ") {
            return Err(BusinessRule(
                "Skill name cannot contain multiple spaces and a rows".to_string(),
            ));
        }

        Ok(())
    }
    fn validate_experience(&self) -> Result<(), DomainError> {
        if self.years_of_experience > 50 {
            return Err(BusinessRule("Too may years of experience".to_string()));
        }
        Ok(())
    }

    fn validate_description(&self) -> Result<(), DomainError> {
        let desc = self.description.trim();

        if desc.len() > 1000 {
            return Err(BusinessRule(
                "Description cannot exceed 1000 characters".to_string(),
            ));
        }

        if !desc.is_empty() && desc.len() < 10 {
            return Err(BusinessRule(
                "Description should be at least 10 characters if provided".to_string(),
            ));
        }

        if desc.chars().all(|c| c.is_whitespace()) {
            return Err(BusinessRule(
                "Description cannot consist only of whitespace".to_string(),
            ));
        }

        let forbidden_chars = ['<', '>', '&', '"', '\''];
        if desc.chars().any(|c| forbidden_chars.contains(&c)) {
            return Err(BusinessRule(
                "Description contains invalid characters".to_string(),
            ));
        }

        Ok(())
    }

    fn validate_id(&self) -> Result<(), DomainError> {
        if self.id.is_empty() {
            return Err(BusinessRule("Skill ID cannot be empty".to_string()));
        }

        if uuid::Uuid::parse_str(&self.id).is_err() {
            return Err(BusinessRule("Skill ID must be a valid UUID".to_string()));
        }

        if self.id.len() > 36 {
            return Err(BusinessRule("Skill ID is too long".to_string()));
        }

        Ok(())
    }

    pub fn is_visible(&self) -> bool {
        matches!(self.visibility, ContentVisibility::Public)
    }

    pub fn validate(&self) -> Result<(), DomainError> {
        self.validate_id()?;
        self.validate_name()?;
        self.validate_description()?;
        self.validate_experience()?;
        Ok(())
    }
}
