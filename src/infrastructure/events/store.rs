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

fn empty_event_stream() -> Arc<[DomainEvent]> {
    Arc::<[DomainEvent]>::from(Vec::<DomainEvent>::new())
}

fn combine_event_chunks(
    combined: Option<&Arc<[DomainEvent]>>,
    pending_chunks: &[Arc<[DomainEvent]>],
) -> Arc<[DomainEvent]> {
    let combined_len = combined.as_ref().map_or(0, |events| events.len());
    let pending_len = pending_chunks
        .iter()
        .map(|chunk| chunk.len())
        .sum::<usize>();
    let mut events = Vec::with_capacity(combined_len + pending_len);
    if let Some(combined) = combined {
        events.extend(combined.iter().cloned());
    }
    for chunk in pending_chunks {
        events.extend(chunk.iter().cloned());
    }
    Arc::<[DomainEvent]>::from(events)
}

fn same_cached_stream(
    current: Option<&Arc<[DomainEvent]>>,
    expected: Option<&Arc<[DomainEvent]>>,
) -> bool {
    match (current, expected) {
        (Some(current), Some(expected)) => Arc::ptr_eq(current, expected),
        (None, None) => true,
        _ => false,
    }
}

fn same_pending_chunks(current: &[Arc<[DomainEvent]>], expected: &[Arc<[DomainEvent]>]) -> bool {
    current.len() == expected.len()
        && current
            .iter()
            .zip(expected)
            .all(|(current, expected)| Arc::ptr_eq(current, expected))
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
        loop {
            let Some((combined_snapshot, pending_snapshot)) = ({
                let events = self.events.read().map_err(|e| e.to_string())?;
                let Some(entry) = events.get(aggregate_id) else {
                    return Ok(empty_event_stream());
                };
                if entry.pending_chunks.is_empty() {
                    return Ok(entry
                        .combined
                        .as_ref()
                        .map_or_else(empty_event_stream, Arc::clone));
                }
                Some((
                    entry.combined.as_ref().map(Arc::clone),
                    entry.pending_chunks.clone(),
                ))
            }) else {
                return Ok(empty_event_stream());
            };

            let combined = combine_event_chunks(combined_snapshot.as_ref(), &pending_snapshot);

            let mut events = self.events.write().map_err(|e| e.to_string())?;
            let Some(entry) = events.get_mut(aggregate_id) else {
                return Ok(combined);
            };
            if !same_cached_stream(entry.combined.as_ref(), combined_snapshot.as_ref())
                || !same_pending_chunks(&entry.pending_chunks, &pending_snapshot)
            {
                continue;
            }

            entry.pending_chunks.clear();
            entry.combined = Some(Arc::clone(&combined));
            return Ok(combined);
        }
    }
}
