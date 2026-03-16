use {
    crate::{application::EventBus, domain::events::DomainEvent},
    std::sync::Arc,
};

pub type EventHandler = Arc<dyn Fn(&DomainEvent) + Send + Sync>;

pub struct InMemoryEventBus {
    handlers: Vec<EventHandler>,
}

impl InMemoryEventBus {
    #[must_use]
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn subscribe(&mut self, handler: EventHandler) {
        self.handlers.push(handler);
    }
}

impl Default for InMemoryEventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus for InMemoryEventBus {
    fn publish(&self, event: &DomainEvent) {
        for handler in &self.handlers {
            handler(event);
        }
    }
}
