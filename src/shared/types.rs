use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUp {
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl CreateUp {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_dates(created_at: DateTime<Utc>, updated_at: DateTime<Utc>) -> Self {
        Self {
            created_at,
            updated_at,
        }
    }

    pub fn with_created_at(created_at: DateTime<Utc>) -> Self {
        Self {
            created_at,
            updated_at: created_at,
        }
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn update(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn set_dates(&mut self, created_at: DateTime<Utc>, updated_at: DateTime<Utc>) {
        self.created_at = created_at;
        self.updated_at = updated_at;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UrlType {
    GitHub,
    LiveDemo,
    Documentation,
    Docker,
    CratesIo,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectUrl {
    url_type: UrlType,
    url: String,
    label: Option<String>,
}

impl ProjectUrl {
    pub fn new(url_type: UrlType, url: String, label: Option<String>) -> Self {
        Self {
            url_type,
            url,
            label,
        }
    }

    pub fn github(url: String) -> Self {
        Self::new(UrlType::GitHub, url, None)
    }

    pub fn live_demo(url: String, label: Option<String>) -> Self {
        Self::new(UrlType::LiveDemo, url, label)
    }

    pub fn url_type(&self) -> &UrlType {
        &self.url_type
    }
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImageSource {
    Url(String),
    Path(String),
}

impl ImageSource {
    pub fn url(url: String) -> Self {
        Self::Url(url)
    }

    pub fn path(path: String) -> Self {
        Self::Path(path)
    }

    pub fn as_url(&self) -> Option<&str> {
        match self {
            ImageSource::Url(url) => Some(url),
            ImageSource::Path(_) => None,
        }
    }

    pub fn as_path(&self) -> Option<&str> {
        match self {
            ImageSource::Path(path) => Some(path),
            ImageSource::Url(_) => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectStatus {
    Planning,
    InProgress,
    Completed,
    Maintenance,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SkillCategory {
    Backend,
    Frontend,
    DevOps,
    Database,
    Tools,
    Languages,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentVisibility {
    Public,
    Private,
    Draft,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    id: String,
    name: String,
    category: SkillCategory,
    level: SkillLevel,
    description: String,
    years_of_experience: f32,
    visibility: ContentVisibility,
    created_updated: CreateUp,
}

impl Skill {
    pub fn new(
        name: String,
        category: SkillCategory,
        level: SkillLevel,
        description: String,
        years_of_experience: f32,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            category,
            level,
            description,
            years_of_experience,
            visibility: ContentVisibility::Public,
            created_updated: CreateUp::new(),
        }
    }

    pub fn with_dates(
        id: String,
        name: String,
        category: SkillCategory,
        level: SkillLevel,
        description: String,
        years_of_experience: f32,
        visibility: ContentVisibility,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            category,
            level,
            description,
            years_of_experience,
            visibility,
            created_updated: CreateUp::with_dates(created_at, updated_at),
        }
    }

    // Геттеры
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
    pub fn years_of_experience(&self) -> f32 {
        self.years_of_experience
    }
    pub fn visibility(&self) -> &ContentVisibility {
        &self.visibility
    }
    pub fn created_updated(&self) -> &CreateUp {
        &self.created_updated
    }

    // Сеттеры с валидацией
    pub fn set_name(&mut self, name: String) -> Result<(), String> {
        if name.trim().is_empty() {
            return Err("Skill name cannot be empty".to_string());
        }
        self.name = name;
        self.created_updated.update();
        Ok(())
    }

    pub fn set_years_of_experience(&mut self, years: f32) -> Result<(), String> {
        if years < 0.0 {
            return Err("Years of experience cannot be negative".to_string());
        }
        self.years_of_experience = years;
        self.created_updated.update();
        Ok(())
    }

    pub fn set_visibility(&mut self, visibility: ContentVisibility) {
        self.visibility = visibility;
        self.created_updated.update();
    }

    // Бизнес-методы
    pub fn is_visible(&self) -> bool {
        matches!(self.visibility, ContentVisibility::Public)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    id: String,
    name: String,
    description: String,
    long_description: String,
    status: ProjectStatus,
    technologies: Vec<String>,
    urls: Vec<ProjectUrl>,
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
        technologies: Vec<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            long_description,
            status,
            technologies,
            urls: Vec::new(),
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
        technologies: Vec<String>,
        urls: Vec<ProjectUrl>,
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
        technologies: Vec<String>,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            long_description,
            status,
            technologies,
            urls: Vec::new(),
            image: None,
            visibility: ContentVisibility::Public,
            featured: false,
            start_date: None,
            end_date: None,
            created_updated: CreateUp::with_created_at(created_at),
        }
    }

    // Геттеры
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
    pub fn technologies(&self) -> &[String] {
        &self.technologies
    }
    pub fn urls(&self) -> &[ProjectUrl] {
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

    // Сеттеры с валидацией
    pub fn set_name(&mut self, name: String) -> Result<(), String> {
        if name.trim().is_empty() {
            return Err("Project name cannot be empty".to_string());
        }
        self.name = name;
        self.created_updated.update();
        Ok(())
    }

    pub fn set_description(&mut self, description: String) -> Result<(), String> {
        if description.trim().is_empty() {
            return Err("Project description cannot be empty".to_string());
        }
        self.description = description;
        self.created_updated.update();
        Ok(())
    }

    // Методы для работы с URLs
    pub fn add_url(&mut self, url: ProjectUrl) {
        self.urls.push(url);
        self.created_updated.update();
    }

    pub fn remove_url(&mut self, index: usize) -> Option<ProjectUrl> {
        if index < self.urls.len() {
            let removed = self.urls.remove(index);
            self.created_updated.update();
            Some(removed)
        } else {
            None
        }
    }

    pub fn find_urls_by_type(&self, url_type: UrlType) -> Vec<&ProjectUrl> {
        self.urls
            .iter()
            .filter(|url| url.url_type == url_type)
            .collect()
    }

    // Методы для работы с изображением
    pub fn set_image(&mut self, image: Option<ImageSource>) {
        self.image = image;
        self.created_updated.update();
    }

    // Бизнес-методы
    pub fn is_visible(&self) -> bool {
        matches!(self.visibility, ContentVisibility::Public)
    }

    pub fn has_github_repo(&self) -> bool {
        self.urls
            .iter()
            .any(|url| matches!(url.url_type, UrlType::GitHub))
    }

    pub fn has_live_demo(&self) -> bool {
        self.urls
            .iter()
            .any(|url| matches!(url.url_type, UrlType::LiveDemo))
    }
}
