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
    pending_chunks: Vec<Arc<[DomainEvent]>>,
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
        let mut events = self.events.write().map_err(|e| e.to_string())?;
        let entry = events.entry(aggregate_id.to_string()).or_default();
        let new_chunk = Arc::<[DomainEvent]>::from(new_events.to_vec());
        entry.pending_chunks.push(Arc::clone(&new_chunk));
        drop(events);
        Ok(())
    }

    fn get_events(
        &self,
        aggregate_id: &str,
    ) -> Result<Arc<[DomainEvent]>, Box<dyn Error + Send + Sync>> {
        let cached = {
            let events = self.events.read().map_err(|e| e.to_string())?;
            events.get(aggregate_id).and_then(|entry| {
                entry
                    .pending_chunks
                    .is_empty()
                    .then(|| entry.combined.as_ref())
                    .flatten()
                    .map(Arc::clone)
            })
        };
        if let Some(combined) = cached {
            return Ok(combined);
        }

        let mut events = self.events.write().map_err(|e| e.to_string())?;
        let Some(entry) = events.get_mut(aggregate_id) else {
            drop(events);
            return Ok(Arc::<[DomainEvent]>::from(Vec::<DomainEvent>::new()));
        };
        if entry.pending_chunks.is_empty() {
            let combined = entry
                .combined
                .as_ref()
                .map(Arc::clone)
                .unwrap_or_else(|| Arc::<[DomainEvent]>::from(Vec::<DomainEvent>::new()));
            drop(events);
            return Ok(combined);
        }

        let combined_len = entry.combined.as_ref().map_or(0, |combined| combined.len());
        let pending_len = entry
            .pending_chunks
            .iter()
            .map(|chunk| chunk.len())
            .sum::<usize>();
        let mut combined_events = Vec::with_capacity(combined_len + pending_len);
        if let Some(combined) = &entry.combined {
            combined_events.extend(combined.iter().cloned());
        }
        for chunk in &entry.pending_chunks {
            combined_events.extend(chunk.iter().cloned());
        }
        let combined = Arc::<[DomainEvent]>::from(combined_events);
        entry.pending_chunks.clear();
        entry.combined = Some(Arc::clone(&combined));
        drop(events);

        Ok(combined)
    }
}
