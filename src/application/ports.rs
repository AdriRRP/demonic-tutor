//! Supports application ports.

use {crate::domain::play::events::DomainEvent, std::error::Error};

pub trait EventBus: Send + Sync {
    fn publish(&self, event: &DomainEvent);
}

pub trait EventStore: Send + Sync {
    /// Appends events to the event store for a given aggregate.
    ///
    /// # Errors
    ///
    /// Returns an error if the write fails.
    fn append(
        &self,
        aggregate_id: &str,
        events: &[DomainEvent],
    ) -> Result<(), Box<dyn Error + Send + Sync>>;

    /// Retrieves all events for a given aggregate.
    ///
    /// # Errors
    ///
    /// Returns an error if the read fails.
    fn get_events(
        &self,
        aggregate_id: &str,
    ) -> Result<Vec<DomainEvent>, Box<dyn Error + Send + Sync>>;
}
