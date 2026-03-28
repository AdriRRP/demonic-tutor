//! Supports application game service.

pub(crate) mod combat;
mod common;
mod lifecycle;
pub(crate) mod resource_actions;
mod rollback;
pub(crate) mod stack;
pub(crate) mod turn_flow;

use crate::{
    application::public_game::PublicEventLogEntry,
    application::{EventBus, EventStore},
    domain::play::{
        errors::{DomainError, GameError},
        events::DomainEvent,
        game::Game,
    },
};
use std::{
    collections::{HashMap, VecDeque},
    mem::size_of_val,
    sync::{Arc, RwLock},
};

use self::rollback::GameRollback;

pub struct GameService<E, B>
where
    E: EventStore,
    B: EventBus,
{
    event_store: E,
    event_bus: B,
    public_event_log_cache: RwLock<PublicEventLogCache>,
}

const PUBLIC_EVENT_LOG_CACHE_CAPACITY: usize = 64;
const PUBLIC_EVENT_LOG_CACHE_MAX_BYTES: usize = 512 * 1024;

#[derive(Debug, Default)]
struct PublicEventLogCache {
    entries: HashMap<String, CachedPublicEventLog>,
    recency: VecDeque<String>,
    total_estimated_bytes: usize,
}

#[derive(Debug)]
struct CachedPublicEventLog {
    entries: Arc<[PublicEventLogEntry]>,
    estimated_bytes: usize,
}

impl PublicEventLogCache {
    fn get(&self, game_id: &str) -> Option<Arc<[PublicEventLogEntry]>> {
        self.entries
            .get(game_id)
            .map(|cached| Arc::clone(&cached.entries))
    }

    fn insert(&mut self, game_id: &str, entries: Arc<[PublicEventLogEntry]>) {
        let estimated_bytes = approximate_public_event_log_bytes(entries.as_ref());
        if let Some(previous) = self.entries.insert(
            game_id.to_string(),
            CachedPublicEventLog {
                entries,
                estimated_bytes,
            },
        ) {
            self.total_estimated_bytes = self
                .total_estimated_bytes
                .saturating_sub(previous.estimated_bytes);
        }
        self.total_estimated_bytes += estimated_bytes;
        self.recency.push_back(game_id.to_string());

        while self.entries.len() > PUBLIC_EVENT_LOG_CACHE_CAPACITY
            || self.total_estimated_bytes > PUBLIC_EVENT_LOG_CACHE_MAX_BYTES
        {
            let Some(oldest) = self.recency.pop_front() else {
                break;
            };
            if let Some(evicted) = self.entries.remove(&oldest) {
                self.total_estimated_bytes = self
                    .total_estimated_bytes
                    .saturating_sub(evicted.estimated_bytes);
                break;
            }
        }
    }

    fn remove(&mut self, game_id: &str) {
        if let Some(removed) = self.entries.remove(game_id) {
            self.total_estimated_bytes = self
                .total_estimated_bytes
                .saturating_sub(removed.estimated_bytes);
        }
    }
}

fn approximate_public_event_log_bytes(entries: &[PublicEventLogEntry]) -> usize {
    // Coarse byte budget: fixed slice storage plus a debug-text estimate of event payload growth.
    size_of_val(entries)
        + entries
            .iter()
            .map(|entry| format!("{:?}", entry.event).len())
            .sum::<usize>()
}

impl<E, B> GameService<E, B>
where
    E: EventStore,
    B: EventBus,
{
    #[must_use]
    pub fn new(event_store: E, event_bus: B) -> Self {
        Self {
            event_store,
            event_bus,
            public_event_log_cache: RwLock::new(PublicEventLogCache::default()),
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
            self.invalidate_public_event_log_cache(game_id);
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
        rollback: GameRollback,
        apply: F,
        map_events: M,
    ) -> Result<T, DomainError>
    where
        F: FnOnce(&mut Game) -> Result<T, DomainError>,
        M: FnOnce(&T) -> Vec<DomainEvent>,
    {
        let outcome = match apply(game) {
            Ok(outcome) => outcome,
            Err(err) => {
                rollback.restore(game)?;
                return Err(err);
            }
        };
        let domain_events = map_events(&outcome);
        if let Err(err) = self.persist_and_publish_events(game.id().as_str(), &domain_events) {
            rollback.restore(game)?;
            return Err(err);
        }

        Ok(outcome)
    }

    fn apply_persisted_event<T, F>(
        &self,
        game: &mut Game,
        rollback: GameRollback,
        apply: F,
    ) -> Result<T, DomainError>
    where
        T: Clone + Into<DomainEvent>,
        F: FnOnce(&mut Game) -> Result<T, DomainError>,
    {
        self.apply_persisted(game, rollback, apply, |event| vec![event.clone().into()])
    }

    pub(crate) fn load_persisted_events(
        &self,
        game_id: &str,
    ) -> Result<Arc<[DomainEvent]>, DomainError> {
        self.event_store.get_events(game_id).map_err(|err| {
            DomainError::Game(GameError::InternalInvariantViolation(format!(
                "failed to load persisted domain events for aggregate {game_id}: {err}"
            )))
        })
    }

    pub(crate) fn cached_public_event_log(
        &self,
        game_id: &str,
    ) -> Option<Arc<[PublicEventLogEntry]>> {
        self.public_event_log_cache
            .read()
            .ok()
            .and_then(|cache| cache.get(game_id))
    }

    pub(crate) fn store_public_event_log_cache(
        &self,
        game_id: &str,
        entries: Arc<[PublicEventLogEntry]>,
    ) {
        if let Ok(mut cache) = self.public_event_log_cache.write() {
            cache.insert(game_id, entries);
        }
    }

    fn invalidate_public_event_log_cache(&self, game_id: &str) {
        if let Ok(mut cache) = self.public_event_log_cache.write() {
            cache.remove(game_id);
        }
    }
}
