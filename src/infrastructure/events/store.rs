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
    events: RwLock<HashMap<String, EventChunks>>,
}

#[derive(Default)]
struct EventChunks {
    chunks: Vec<Arc<[DomainEvent]>>,
    combined: Option<Arc<[DomainEvent]>>,
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
        {
            let mut events = self.events.write().map_err(|e| e.to_string())?;
            let entry = events.entry(aggregate_id.to_string()).or_default();
            entry
                .chunks
                .push(Arc::<[DomainEvent]>::from(new_events.to_vec()));
            entry.combined = None;
        }
        Ok(())
    }

    fn get_events(
        &self,
        aggregate_id: &str,
    ) -> Result<Arc<[DomainEvent]>, Box<dyn Error + Send + Sync>> {
        let cached = {
            let events = self.events.read().map_err(|e| e.to_string())?;
            events
                .get(aggregate_id)
                .and_then(|entry| entry.combined.clone())
        };
        if let Some(combined) = cached {
            return Ok(combined);
        }

        let mut events = self.events.write().map_err(|e| e.to_string())?;
        let Some(entry) = events.get_mut(aggregate_id) else {
            return Ok(Arc::<[DomainEvent]>::from(Vec::<DomainEvent>::new()));
        };
        if let Some(combined) = &entry.combined {
            return Ok(Arc::clone(combined));
        }

        let total_len = entry.chunks.iter().map(|chunk| chunk.len()).sum();
        let mut combined_events = Vec::with_capacity(total_len);
        for chunk in &entry.chunks {
            combined_events.extend(chunk.iter().cloned());
        }
        let combined = Arc::<[DomainEvent]>::from(combined_events);
        entry.combined = Some(Arc::clone(&combined));

        Ok(combined)
    }
}
