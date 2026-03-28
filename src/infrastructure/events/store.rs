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
    events: RwLock<HashMap<String, Vec<Arc<[DomainEvent]>>>>,
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
        events
            .entry(aggregate_id.to_string())
            .or_default()
            .push(Arc::<[DomainEvent]>::from(new_events.to_vec()));
        Ok(())
    }

    fn get_events(
        &self,
        aggregate_id: &str,
    ) -> Result<Arc<[DomainEvent]>, Box<dyn Error + Send + Sync>> {
        let events = self.events.read().map_err(|e| e.to_string())?;
        let Some(chunks) = events.get(aggregate_id) else {
            return Ok(Arc::<[DomainEvent]>::from(Vec::<DomainEvent>::new()));
        };

        let total_len = chunks.iter().map(|chunk| chunk.len()).sum();
        let mut combined_events = Vec::with_capacity(total_len);
        for chunk in chunks {
            combined_events.extend(chunk.iter().cloned());
        }

        Ok(Arc::<[DomainEvent]>::from(combined_events))
    }
}
