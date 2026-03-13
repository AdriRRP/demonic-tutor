use crate::application::EventStore;
use crate::domain::events::DomainEvent;
use std::collections::HashMap;
use std::error::Error;
use std::sync::RwLock;

#[allow(clippy::significant_drop_tightening)]
pub struct InMemoryEventStore {
    events: RwLock<HashMap<String, Vec<DomainEvent>>>,
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
    #[allow(clippy::significant_drop_tightening)]
    fn append(
        &self,
        aggregate_id: &str,
        new_events: &[DomainEvent],
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut events = self.events.write().map_err(|e| e.to_string())?;
        let entry = events
            .entry(aggregate_id.to_string())
            .or_insert_with(Vec::new);
        entry.extend(new_events.iter().cloned());
        Ok(())
    }

    fn get_events(
        &self,
        aggregate_id: &str,
    ) -> Result<Vec<DomainEvent>, Box<dyn Error + Send + Sync>> {
        let events = self.events.read().map_err(|e| e.to_string())?;
        Ok(events.get(aggregate_id).cloned().unwrap_or_default())
    }
}
