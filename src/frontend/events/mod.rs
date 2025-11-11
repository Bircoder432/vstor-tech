// src/frontend/events/mod.rs
use crate::shared::events::{AppEvent, EventBus};
use leptos::*;

pub struct FrontendEventHandler;

impl FrontendEventHandler {
    pub fn new(
        _event_bus: EventBus,
        _skills_trigger: WriteSignal<u32>,
        _projects_trigger: WriteSignal<u32>,
    ) -> Self {
        // Пока просто заглушка
        Self
    }
}
