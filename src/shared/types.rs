use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub enum UrlType {
    #[schema(rename = "github")]
    GitHub,
    #[schema(rename = "live_demo")]
    LiveDemo,
    #[schema(rename = "documentation")]
    Documentation,
    #[schema(rename = "docker")]
    Docker,
    #[schema(rename = "crates_io")]
    CratesIo,
    #[schema(rename = "other")]
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProjectUrl {
    url_type: UrlType,
    #[schema(example = "https://github.com/user/repo")]
    url: String,
    #[schema(example = "Source Code")]
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

impl PartialEq for ProjectUrl {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

impl Eq for ProjectUrl {}

impl Hash for ProjectUrl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub enum ImageSource {
    #[schema(rename = "url")]
    Url(String),
    #[schema(rename = "path")]
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
