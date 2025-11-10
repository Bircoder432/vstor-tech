use crate::shared::events::{
    AppEvent, EventBus, ProjectPublished, ProjectStatusChanged, ProjectUpdated, SkillCreated,
    SkillDeleted, SkillUpdated,
};
use leptos::*;
use logging::log;
use tokio::task::spawn_local;

pub struct FrontendEventHandler {
    event_bus: EventBus,

    trigger_skills: RwSignal<u32>,
    trigger_projects: RwSignal<u32>,
    trigger_content: RwSignal<u32>,
}

impl FrontendEventHandler {
    pub fn new(
        event_bus: EventBus,
        trigger_skills: RwSignal<u32>,
        trigger_projects: RwSignal<u32>,
        trigger_content: RwSignal<u32>,
    ) -> Self {
        let handler = Self {
            event_bus,
            trigger_skills,
            trigger_projects,
            trigger_content,
        };
        handler.start_listening();
        handler
    }

    fn start_listening(&self) {
        let mut receiver = self.event_bus.subscribe();
        let trigger_skills = self.trigger_skills;
        let trigger_projects = self.trigger_projects;
        let trigger_content = self.trigger_content;

        spawn_local(async move {
            while let Ok(event) = receiver.recv().await {
                log!("Received event: {:?}", event);
                match event {
                    AppEvent::SkillCreated(SkillCreated { skill_id, .. }) => {
                        trigger_skills.update(|n| *n += 1);
                        Self::handle_skill_added(skill_id).await;
                    }
                    AppEvent::SkillUpdated(SkillUpdated { skill_id, .. }) => {
                        trigger_skills.update(|n| *n += 1);
                        Self::handle_skill_updated(skill_id).await;
                    }
                    AppEvent::SkillDeleted(SkillDeleted { skill_id, .. }) => {
                        trigger_skills.update(|n| *n += 1);
                        Self::handle_skill_deleted(skill_id).await;
                    }

                    AppEvent::ProjectUpdated(ProjectUpdated { project_id, .. }) => {
                        trigger_projects.update(|n| *n += 1);
                        Self::handle_project_updated(project_id).await;
                    }
                    AppEvent::ProjectStatusChanged(ProjectStatusChanged { project_id, .. }) => {
                        trigger_projects.update(|n| *n += 1);
                        Self::handle_project_status_changed(project_id).await;
                    }
                    AppEvent::ProjectPublished(ProjectPublished { project_id, .. }) => {
                        trigger_projects.update(|n| *n += 1);
                        Self::handle_project_published(project_id).await;
                    }

                    AppEvent::ContentChanged => {
                        trigger_content.update(|n| *n += 1);
                        Self::handle_content_changed().await;
                    }
                }
            }
        });
    }

    async fn handle_skill_added(skill_id: String) {
        log!("Handling skill added: {}", skill_id);
    }

    async fn handle_skill_updated(skill_id: String) {
        log!("Handling skill updated: {}", skill_id);
    }

    async fn handle_skill_deleted(skill_id: String) {
        log!("Handling skill deleted: {}", skill_id);
    }

    async fn handle_project_updated(project_id: String) {
        log!("Handling project updated: {}", project_id);
    }

    async fn handle_project_status_changed(project_id: String) {
        log!("Handling project status changed: {}", project_id);
    }

    async fn handle_project_published(project_id: String) {
        log!("Handling project published: {}", project_id);
    }

    async fn handle_content_changed() {
        log!("Handling content changed");
    }
}
