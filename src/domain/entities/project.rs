use crate::domain::entities::create_up::CreateUp;
use crate::domain::errors::DomainError::{self, BusinessRule};
use crate::domain::types::{ContentVisibility, ProjectStatus, ProjectUrls, UniqueTechnologies};
use crate::domain::validators::CommonValidator;
use crate::shared::types::{ImageSource, ProjectUrl, UrlType};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    id: String,
    name: String,
    description: String,
    long_description: String,
    status: ProjectStatus,
    technologies: UniqueTechnologies,
    urls: ProjectUrls,
    image: Option<ImageSource>,
    visibility: ContentVisibility,
    featured: bool,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    created_updated: CreateUp,
}

impl Project {
    pub fn new(
        name: String,
        description: String,
        long_description: String,
        status: ProjectStatus,
        technologies: UniqueTechnologies,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            long_description,
            status,
            technologies,
            urls: ProjectUrls::new(),
            image: None,
            visibility: ContentVisibility::Public,
            featured: false,
            start_date: None,
            end_date: None,
            created_updated: CreateUp::new(),
        }
    }

    pub fn with_dates(
        id: String,
        name: String,
        description: String,
        long_description: String,
        status: ProjectStatus,
        technologies: UniqueTechnologies,
        urls: ProjectUrls,
        image: Option<ImageSource>,
        visibility: ContentVisibility,
        featured: bool,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            long_description,
            status,
            technologies,
            urls,
            image,
            visibility,
            featured,
            start_date,
            end_date,
            created_updated: CreateUp::with_dates(created_at, updated_at),
        }
    }

    pub fn with_creation_date(
        name: String,
        description: String,
        long_description: String,
        status: ProjectStatus,
        technologies: UniqueTechnologies,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            long_description,
            status,
            technologies,
            urls: ProjectUrls::new(),
            image: None,
            visibility: ContentVisibility::Public,
            featured: false,
            start_date: None,
            end_date: None,
            created_updated: CreateUp::with_created_at(created_at),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn long_description(&self) -> &str {
        &self.long_description
    }
    pub fn status(&self) -> &ProjectStatus {
        &self.status
    }
    pub fn technologies(&self) -> &UniqueTechnologies {
        &self.technologies
    }
    pub fn urls(&self) -> &ProjectUrls {
        &self.urls
    }
    pub fn image(&self) -> Option<&ImageSource> {
        self.image.as_ref()
    }
    pub fn visibility(&self) -> &ContentVisibility {
        &self.visibility
    }
    pub fn featured(&self) -> bool {
        self.featured
    }
    pub fn start_date(&self) -> Option<NaiveDate> {
        self.start_date
    }
    pub fn end_date(&self) -> Option<NaiveDate> {
        self.end_date
    }
    pub fn created_updated(&self) -> &CreateUp {
        &self.created_updated
    }

    pub fn set_name(&mut self, name: String) -> Result<(), DomainError> {
        if name.trim().is_empty() {
            return Err(BusinessRule("Project name cannot be empty".to_string()));
        }
        self.name = name;
        self.created_updated.update();
        Ok(())
    }

    pub fn set_description(&mut self, description: String) -> Result<(), DomainError> {
        self.description = description;
        self.validate()?;
        self.created_updated.update();
        Ok(())
    }
    pub fn set_long_description(&mut self, description: String) -> Result<(), DomainError> {
        self.description = description;
        self.validate()?;
        self.created_updated.update();
        Ok(())
    }

    pub fn add_url(&mut self, url: ProjectUrl) -> Result<(), DomainError> {
        self.urls.add(url)?;
        self.created_updated.update();
        Ok(())
    }

    pub fn remove_url(&mut self, url: ProjectUrl) -> Result<(), DomainError> {
        self.urls.remove(url)?;
        self.created_updated.update();
        Ok(())
    }

    pub fn find_urls_by_type(&self, url_type: UrlType) -> Vec<&ProjectUrl> {
        self.urls
            .iter()
            .filter(|url| url.url_type() == &url_type)
            .collect()
    }

    pub fn set_image(&mut self, image: Option<ImageSource>) {
        self.image = image;
        self.created_updated.update();
    }

    pub fn set_featured(&mut self, featured: bool) -> Result<(), DomainError> {
        self.featured = featured;
        self.created_updated.update();
        self.validate()?;
        Ok(())
    }

    pub fn set_visibility(&mut self, visibility: ContentVisibility) -> Result<(), DomainError> {
        self.visibility = visibility;
        self.created_updated.update();
        self.validate()?;
        Ok(())
    }

    pub fn set_dates(
        &mut self,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> Result<(), DomainError> {
        self.start_date = start;
        self.end_date = end;
        self.created_updated.update();
        self.validate()?;
        Ok(())
    }

    pub fn add_technology(&mut self, tech: String) -> Result<(), DomainError> {
        self.technologies.add(tech)?;
        self.created_updated.update();
        self.validate()?;
        Ok(())
    }

    pub fn remove_technology(&mut self, tech: &str) -> Result<(), DomainError> {
        self.technologies.remove(tech)?;
        self.created_updated.update();
        self.validate()?;
        Ok(())
    }

    pub fn is_visible(&self) -> bool {
        matches!(self.visibility, ContentVisibility::Public)
    }

    pub fn has_github_repo(&self) -> bool {
        self.urls
            .iter()
            .any(|url| matches!(url.url_type(), &UrlType::GitHub))
    }

    pub fn has_live_demo(&self) -> bool {
        self.urls
            .iter()
            .any(|url| matches!(url.url_type(), &UrlType::LiveDemo))
    }

    fn validate_id(&self) -> Result<(), DomainError> {
        CommonValidator::validate_id(&self.id)
    }

    fn validate_name(&self) -> Result<(), DomainError> {
        CommonValidator::validate_name(&self.name, "Project")
    }

    fn validate_descriptions(&self) -> Result<(), DomainError> {
        CommonValidator::validate_description(&self.description, 150)?;
        CommonValidator::validate_description(&self.long_description, 300)?;
        Ok(())
    }

    fn validate_dates(&self) -> Result<(), DomainError> {
        if let (Some(start), Some(end)) = (self.start_date, self.end_date) {
            if start > end {
                return Err(BusinessRule(
                    "Start date cannot be after end date".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn validate_status_consistency(&self) -> Result<(), DomainError> {
        if self.status == ProjectStatus::Completed
            && self.end_date.is_none()
            && self.start_date().is_none()
        {
            return Err(BusinessRule(
                "Complete project must have start date and end date".to_string(),
            ));
        }
        if self.status == ProjectStatus::Planning && self.start_date.is_some() {
            return Err(BusinessRule(
                "Planning project can't have start date".to_string(),
            ));
        }
        if (self.status == ProjectStatus::InProgress || self.status == ProjectStatus::Maintenance)
            && self.start_date.is_none()
        {
            return Err(BusinessRule(
                "InProgress and Maintenance projects must have start date".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_featured_rules(&self) -> Result<(), DomainError> {
        if self.featured {
            if !self.is_visible() {
                return Err(BusinessRule("Featured project must be public".to_string()));
            }

            match self.status {
                ProjectStatus::Planning | ProjectStatus::InProgress => {
                    return Err(BusinessRule(
                        "Featured project must be completed or in maintenance".to_string(),
                    ));
                }
                ProjectStatus::Completed | ProjectStatus::Maintenance => {}
            }

            if self.long_description.trim().is_empty() {
                return Err(BusinessRule(
                    "Featured project must have a long description".to_string(),
                ));
            }

            if self.technologies.len() < 1 {
                return Err(BusinessRule(
                    "Featured project must have at least 1 technologies".to_string(),
                ));
            }

            if !self.has_github_repo() && !self.has_live_demo() {
                return Err(BusinessRule(
                    "Featured project must have GitHub repo or live demo".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<(), DomainError> {
        self.validate_id()?;
        self.validate_name()?;
        self.validate_descriptions()?;
        self.validate_dates()?;
        self.validate_status_consistency()?;
        self.validate_featured_rules()?;
        Ok(())
    }
}
