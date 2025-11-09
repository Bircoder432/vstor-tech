use crate::shared::events::{AppEvent, EventBus};
use leptos::*;
use tokio::task::spawn_local;

pub struct FrontendEventHandler {
    event_bus: EventBus,
}

impl FrontendEventHandler {
    pub fn new(event_bus: EventBus) -> Self {
        let handler = Self { event_bus };
        handler.start_listening();
        handler
    }

    fn start_listening(&self) {
        let mut reciver = self.event_bus.subscribe();

        spawn_local(async move {
            while let Ok(event) = reciver.recv().await {
                match event {
                    AppEvent::SkillAdded { skill_id } => {
                        logging::log!("Skill added: {}", skill_id);
                        Self::handle_skill_added().await;
                    }
                    AppEvent::ProjectUpdated { project_id } => {
                        logging::log!("Project updated: {}", project_id);
                        Self::handle_project_updated().await;
                    }
                    AppEvent::ContentChanged => {
                        Self::handle_content_changed().await;
                    }
                }
            }
        });
    }

    async fn handle_skill_added() {
        todo!();
    }

    async fn handle_project_updated() {
        todo!();
    }

    async fn handle_content_changed() {
        todo!();
    }
}
