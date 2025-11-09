use crate::shared::events::{AppEvent, EventBus};

pub struct BackendEventSystem {
    event_bus: EventBus,
}

impl BackendEventSystem {
    pub fn new(event_bus: EventBus) -> Self {
        Self { event_bus }
    }

    pub async fn skill_added(&self, skill_id: String) {
        let event = AppEvent::SkillAdded { skill_id };
        self.event_bus.publish(event).ok();
    }

    pub async fn project_updated(&self, project_id: String) {
        let event = AppEvent::ProjectUpdated { project_id };
        self.event_bus.publish(event).ok();
    }
}
