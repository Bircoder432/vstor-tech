use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Skill error: {0}")]
    Skill(#[from] SkillError),

    #[error("Project error: {0}")]
    Project(#[from] ProjectError),

    #[error("Business rule violation: {0}")]
    BusinessRule(String),
}

#[derive(Error, Debug)]
pub enum SkillError {
    #[error("Invalid skill level")]
    InvalidLevel,

    #[error("Skill name too short: {0}")]
    NameTooShort(String),

    #[error("Skill already exists: {0}")]
    AlreadyExists(String),
}

#[derive(Error, Debug)]
pub enum ProjectError {
    #[error("Invalid project status transition")]
    InvalidStatusTransition,

    #[error("Project dates invalid: start {start} > end {end}")]
    InvalidDates { start: String, end: String },

    #[error("Invalid URL: {url} - {reason}")]
    InvalidUrl { url: String, reason: String },

    #[error("Project not visible")]
    NotVisible,
}
