//! Supports infrastructure events store.

use {
    crate::{application::EventStore, domain::play::events::DomainEvent},
    std::{
        collections::HashMap,
        error::Error,
        sync::{Arc, RwLock},
    },
};

pub struct InMemoryEventStore {
    events: RwLock<HashMap<String, Arc<[DomainEvent]>>>,
}

impl InMemoryEventStore {
    #[must_use]
    pub fn new() -> Self {
        Self {
            events: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryEventStore {
    fn default() -> Self {
        Self::new()
    }
}

impl EventStore for InMemoryEventStore {
    fn append(
        &self,
        aggregate_id: &str,
        new_events: &[DomainEvent],
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut events = self.events.write().map_err(|e| e.to_string())?;
        let key = aggregate_id.to_string();
        let combined_events = events.get(&key).map_or_else(Vec::new, |existing| {
            existing.iter().cloned().collect::<Vec<_>>()
        });
        let mut combined_events = combined_events;
        combined_events.extend(new_events.iter().cloned());
        events.insert(key, Arc::<[DomainEvent]>::from(combined_events));
        drop(events);
        Ok(())
    }

    fn get_events(
        &self,
        aggregate_id: &str,
    ) -> Result<Arc<[DomainEvent]>, Box<dyn Error + Send + Sync>> {
        let events = self.events.read().map_err(|e| e.to_string())?;
        Ok(events
            .get(aggregate_id)
            .cloned()
            .unwrap_or_else(|| Arc::<[DomainEvent]>::from(Vec::<DomainEvent>::new())))
    }
}
