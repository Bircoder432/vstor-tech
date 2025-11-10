use crate::domain::types::{ProjectStatus, SkillCategory, SkillLevel};
use crate::shared::events::{
    AppEvent, EventBus, EventMetadata, ProjectPublished, ProjectStatusChanged, SkillCreated,
    SkillDeleted, SkillUpdated,
};
use chrono::{DateTime, Utc};
use tokio::sync::broadcast;
use uuid::Uuid;

pub struct BackendEventSystem {
    event_bus: EventBus,
    source: String,
}

impl BackendEventSystem {
    pub fn new(event_bus: EventBus, source: impl Into<String>) -> Self {
        Self {
            event_bus,
            source: source.into(),
        }
    }

    fn metadata(&self, event_type: &str) -> EventMetadata {
        EventMetadata {
            event_id: Uuid::new_v4().to_string(),
            event_type: event_type.to_string(),
            timestamp: Utc::now(),
            source: self.source.clone(),
        }
    }

    pub fn skill_created(
        &self,
        skill_id: String,
        skill_name: String,
        category: SkillCategory,
    ) -> Result<usize, broadcast::error::SendError<AppEvent>> {
        let metadata = self.metadata("SkillCreated");
        let payload = SkillCreated {
            metadata,
            skill_id,
            skill_name,
            category,
        };
        self.event_bus.publish(AppEvent::SkillCreated(payload))
    }

    pub fn skill_level_updated(
        &self,
        skill_id: String,
        old_level: SkillLevel,
        new_level: SkillLevel,
    ) -> Result<usize, broadcast::error::SendError<AppEvent>> {
        let metadata = self.metadata("SkillUpdated");
        let payload = SkillUpdated {
            metadata,
            skill_id,
            old_level,
            new_level,
        };
        self.event_bus.publish(AppEvent::SkillUpdated(payload))
    }

    pub fn skill_deleted(
        &self,
        skill_id: String,
        skill_name: String,
    ) -> Result<usize, broadcast::error::SendError<AppEvent>> {
        let metadata = self.metadata("SkillDeleted");
        let payload = SkillDeleted {
            metadata,
            skill_id,
            skill_name,
        };
        self.event_bus.publish(AppEvent::SkillDeleted(payload))
    }

    pub fn project_status_changed(
        &self,
        project_id: String,
        old_status: ProjectStatus,
        new_status: ProjectStatus,
    ) -> Result<usize, broadcast::error::SendError<AppEvent>> {
        let metadata = self.metadata("ProjectStatusChanged");
        let payload = ProjectStatusChanged {
            metadata,
            project_id,
            old_status,
            new_status,
        };
        self.event_bus
            .publish(AppEvent::ProjectStatusChanged(payload))
    }

    pub fn project_updated(
        &self,
        project_id: String,
    ) -> Result<usize, broadcast::error::SendError<AppEvent>> {
        let metadata = self.metadata("ProjectUpdated");
        let payload = crate::shared::events::ProjectUpdated {
            metadata,
            project_id,
        };
        self.event_bus.publish(AppEvent::ProjectUpdated(payload))
    }

    pub fn project_published(
        &self,
        project_id: String,
        project_name: String,
        published_url: Option<String>,
    ) -> Result<usize, broadcast::error::SendError<AppEvent>> {
        let metadata = self.metadata("ProjectPublished");
        let payload = ProjectPublished {
            metadata,
            project_id,
            project_name,
            published_url,
        };
        self.event_bus.publish(AppEvent::ProjectPublished(payload))
    }
}
