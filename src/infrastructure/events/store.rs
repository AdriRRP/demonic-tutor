use {
    crate::{application::EventStore, domain::play::events::DomainEvent},
    std::{collections::HashMap, error::Error, sync::RwLock},
};

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
    fn append(
        &self,
        aggregate_id: &str,
        new_events: &[DomainEvent],
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        {
            let mut events = self.events.write().map_err(|e| e.to_string())?;
            let key = aggregate_id.to_string();
            if let Some(existing) = events.get_mut(&key) {
                existing.extend(new_events.iter().cloned());
            } else {
                events.insert(key, new_events.to_vec());
            }
        }
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
