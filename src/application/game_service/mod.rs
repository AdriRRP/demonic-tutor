//! Supports application game service.

pub(crate) mod combat;
mod common;
mod lifecycle;
pub(crate) mod resource_actions;
pub(crate) mod stack;
pub(crate) mod turn_flow;

use crate::{
    application::{EventBus, EventStore},
    domain::play::{
        errors::{DomainError, GameError},
        events::DomainEvent,
        game::Game,
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

    fn apply_persisted<T, F, M>(
        &self,
        game: &mut Game,
        apply: F,
        map_events: M,
    ) -> Result<T, DomainError>
    where
        F: FnOnce(&mut Game) -> Result<T, DomainError>,
        M: FnOnce(&T) -> Vec<DomainEvent>,
    {
        let mut candidate = game.clone();
        let outcome = apply(&mut candidate)?;
        let domain_events = map_events(&outcome);
        self.persist_and_publish_events(candidate.id().as_str(), &domain_events)?;
        *game = candidate;

        Ok(outcome)
    }

    fn apply_persisted_event<T, F>(&self, game: &mut Game, apply: F) -> Result<T, DomainError>
    where
        T: Clone + Into<DomainEvent>,
        F: FnOnce(&mut Game) -> Result<T, DomainError>,
    {
        self.apply_persisted(game, apply, |event| vec![event.clone().into()])
    }

    pub(crate) fn load_persisted_events(
        &self,
        game_id: &str,
    ) -> Result<Vec<DomainEvent>, DomainError> {
        self.event_store.get_events(game_id).map_err(|err| {
            DomainError::Game(GameError::InternalInvariantViolation(format!(
                "failed to load persisted domain events for aggregate {game_id}: {err}"
            )))
        })
    }
}
