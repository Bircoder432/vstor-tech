use crate::domain::errors::DomainError::{self, BusinessRule};
use crate::shared::types::{ProjectUrl, UrlType};
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use utoipa::{
    ToSchema,
    openapi::{Object, Schema},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub enum ProjectStatus {
    Planning,
    InProgress,
    Completed,
    Maintenance,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub enum SkillCategory {
    Backend,
    Frontend,
    DevOps,
    Database,
    Tools,
    Languages,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub enum ContentVisibility {
    Public,
    Private,
    Draft,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, ToSchema)]
pub enum SkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

impl SkillLevel {
    pub fn from_experience(years: u8) -> Self {
        match years {
            0..=1 => SkillLevel::Beginner,
            2..=3 => SkillLevel::Intermediate,
            4..=6 => SkillLevel::Advanced,
            _ => SkillLevel::Expert,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueTechnologies(IndexSet<String>);

impl UniqueTechnologies {
    pub fn new() -> Self {
        Self(IndexSet::new())
    }

    pub fn from_vec(techs: Vec<String>) -> Result<Self, DomainError> {
        let set: IndexSet<String> = techs.into_iter().collect();

        if set.len() > 20 {
            return Err(BusinessRule("Too many technologies".to_string()));
        }

        Ok(Self(set))
    }

    pub fn add(&mut self, tech: String) -> Result<(), DomainError> {
        if self.0.len() >= 20 {
            return Err(BusinessRule("Max technologies reached".to_string()));
        }
        self.0.insert(tech);
        Ok(())
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn remove(&mut self, tech: &str) -> Result<(), DomainError> {
        if self.0.len() <= 0 {
            return Err(BusinessRule(
                "Can't delete from empty technologies".to_string(),
            ));
        }
        self.0.swap_remove(tech);
        Ok(())
    }

    pub fn as_slice(&self) -> Vec<&str> {
        self.0.iter().map(|s| s.as_str()).collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.0.iter()
    }

    pub fn as_vec(&self) -> Vec<String> {
        self.0.iter().cloned().collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectUrls(IndexSet<ProjectUrl>);

impl ProjectUrls {
    pub fn new() -> Self {
        Self(IndexSet::new())
    }

    pub fn from_vec(urls: Vec<ProjectUrl>) -> Result<Self, DomainError> {
        let set: IndexSet<ProjectUrl> = urls.into_iter().collect();

        if set.len() > 10 {
            return Err(BusinessRule("Too many URLs (max 10)".to_string()));
        }

        for url in &set {
            if url.url().trim().is_empty() {
                return Err(BusinessRule("URL cannot be empty".to_string()));
            }

            match url.url_type() {
                UrlType::GitHub if !url.url().contains("github.com") => {
                    return Err(BusinessRule(
                        "GitHub URL must contain 'github.com'".to_string(),
                    ));
                }
                UrlType::LiveDemo if !url.url().starts_with("http") => {
                    return Err(BusinessRule(
                        "Live demo URL must start with http/https".to_string(),
                    ));
                }
                _ => {}
            }
        }

        Ok(Self(set))
    }

    pub fn add(&mut self, url: ProjectUrl) -> Result<(), DomainError> {
        if self.0.len() >= 10 {
            return Err(BusinessRule("Max URLs reached".to_string()));
        }
        self.0.insert(url);
        Ok(())
    }

    pub fn remove(&mut self, url: ProjectUrl) -> Result<(), DomainError> {
        if self.0.len() <= 0 {
            return Err(BusinessRule("Can't delete from empty urls".to_string()));
        }
        self.0.swap_remove(&url);
        Ok(())
    }

    pub fn iter(&self) -> indexmap::set::Iter<ProjectUrl> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn find_by_type(&self, url_type: UrlType) -> Vec<&ProjectUrl> {
        self.0
            .iter()
            .filter(|url| url.url_type() == &url_type)
            .collect()
    }
}
