use tokio::sync::broadcast;

#[derive(Clone, Debug)]
pub enum AppEvent {
    SkillAdded { skill_id: String },
    ProjectUpdated { project_id: String },
    ContentChanged,
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

    pub fn publish(&self, event: AppEvent) -> Result<(), broadcast::error::SendError<AppEvent>> {
        self.sender.send(event)
    }

    pub fn subscribe(&self) -> broadcast::Receiver<AppEvent> {
        self.sender.subscribe()
    }
}
