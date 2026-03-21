mod combat;
mod common;
mod lifecycle;
mod resource_actions;
mod stack;
mod turn_flow;

use crate::{
    application::{EventBus, EventStore},
    domain::play::{
        errors::{DomainError, GameError},
        events::DomainEvent,
    },
};

pub struct GameService<E, B>
where
    E: EventStore,
    B: EventBus,
{
    event_store: E,
    event_bus: B,
}

impl<E, B> GameService<E, B>
where
    E: EventStore,
    B: EventBus,
{
    #[must_use]
    pub const fn new(event_store: E, event_bus: B) -> Self {
        Self {
            event_store,
            event_bus,
        }
    }

    fn persist_and_publish_events(
        &self,
        game_id: &str,
        events: &[DomainEvent],
    ) -> Result<(), DomainError> {
        if !events.is_empty() {
            self.event_store.append(game_id, events).map_err(|err| {
                DomainError::Game(GameError::InternalInvariantViolation(format!(
                    "failed to persist domain events for aggregate {game_id}: {err}"
                )))
            })?;
            for event in events {
                self.event_bus.publish(event);
            }
        }

        Ok(())
    }

    fn persist_and_publish_event<T>(&self, game_id: &str, event: &T) -> Result<(), DomainError>
    where
        T: Clone + Into<DomainEvent>,
    {
        self.persist_and_publish_events(game_id, &[event.clone().into()])
    }

    fn persist_and_publish_event_batch<T>(
        &self,
        game_id: &str,
        events: &[T],
    ) -> Result<(), DomainError>
    where
        T: Clone + Into<DomainEvent>,
    {
        if events.is_empty() {
            return Ok(());
        }

        let domain_events = events.iter().cloned().map(Into::into).collect::<Vec<_>>();
        self.persist_and_publish_events(game_id, &domain_events)
    }
}
