use crate::domain::types::{ProjectStatus, SkillCategory, SkillLevel};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub event_id: String,
    pub event_type: String,
    pub timestamp: DateTime<Utc>,
    pub source: String,
}

pub trait DomainEvent: Send + Sync {
    fn metadata(&self) -> &EventMetadata;
    fn event_type(&self) -> &str;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectUpdated {
    pub metadata: EventMetadata,
    pub project_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillCreated {
    pub metadata: EventMetadata,
    pub skill_id: String,
    pub skill_name: String,
    pub category: SkillCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillUpdated {
    pub metadata: EventMetadata,
    pub skill_id: String,
    pub old_level: SkillLevel,
    pub new_level: SkillLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDeleted {
    pub metadata: EventMetadata,
    pub skill_id: String,
    pub skill_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectPublished {
    pub metadata: EventMetadata,
    pub project_id: String,
    pub project_name: String,
    pub published_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStatusChanged {
    pub metadata: EventMetadata,
    pub project_id: String,
    pub old_status: ProjectStatus,
    pub new_status: ProjectStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppEvent {
    SkillCreated(SkillCreated),
    SkillUpdated(SkillUpdated),
    SkillDeleted(SkillDeleted),
    ProjectPublished(ProjectPublished),
    ProjectStatusChanged(ProjectStatusChanged),
    ProjectUpdated(ProjectUpdated),
}
#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<AppEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self { sender }
    }

    pub fn publish(&self, event: AppEvent) -> Result<usize, broadcast::error::SendError<AppEvent>> {
        self.sender.send(event)
    }

    pub fn subscribe(&self) -> broadcast::Receiver<AppEvent> {
        self.sender.subscribe()
    }
}
