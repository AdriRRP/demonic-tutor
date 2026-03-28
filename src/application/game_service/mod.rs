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
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, RwLock,
    },
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
    public_event_log_access_clock: AtomicU64,
}

const PUBLIC_EVENT_LOG_CACHE_CAPACITY: usize = 64;
const PUBLIC_EVENT_LOG_CACHE_MAX_BYTES: usize = 512 * 1024;

#[derive(Debug, Default)]
struct PublicEventLogCache {
    entries: HashMap<String, CachedPublicEventLog>,
    total_estimated_bytes: usize,
}

#[derive(Debug)]
struct CachedPublicEventLog {
    entries: Arc<[PublicEventLogEntry]>,
    estimated_bytes: usize,
    last_access_tick: AtomicU64,
}

impl PublicEventLogCache {
    fn get(&self, game_id: &str, access_tick: u64) -> Option<Arc<[PublicEventLogEntry]>> {
        let cached = self.entries.get(game_id)?;
        cached
            .last_access_tick
            .store(access_tick, Ordering::Relaxed);
        Some(Arc::clone(&cached.entries))
    }

    fn insert(
        &mut self,
        game_id: &str,
        entries: Arc<[PublicEventLogEntry]>,
        estimated_bytes: usize,
        access_tick: u64,
    ) {
        if let Some(previous) = self.entries.remove(game_id) {
            self.total_estimated_bytes = self
                .total_estimated_bytes
                .saturating_sub(previous.estimated_bytes);
        }
        self.total_estimated_bytes += estimated_bytes;
        self.entries.insert(
            game_id.to_string(),
            CachedPublicEventLog {
                entries,
                estimated_bytes,
                last_access_tick: AtomicU64::new(access_tick),
            },
        );

        while self.entries.len() > PUBLIC_EVENT_LOG_CACHE_CAPACITY
            || self.total_estimated_bytes > PUBLIC_EVENT_LOG_CACHE_MAX_BYTES
        {
            let Some(oldest_game_id) = self.oldest_game_id() else {
                break;
            };
            if let Some(evicted) = self.entries.remove(&oldest_game_id) {
                self.total_estimated_bytes = self
                    .total_estimated_bytes
                    .saturating_sub(evicted.estimated_bytes);
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

    fn oldest_game_id(&self) -> Option<String> {
        self.entries
            .iter()
            .min_by_key(|(_, cached)| cached.last_access_tick.load(Ordering::Relaxed))
            .map(|(game_id, _)| game_id.clone())
    }
}

impl<E, B> GameService<E, B>
where
    E: EventStore,
    B: EventBus,
{
    fn cache_error(operation: &str) -> DomainError {
        DomainError::Game(GameError::InternalInvariantViolation(format!(
            "public event log cache lock poisoned while trying to {operation}"
        )))
    }

    #[must_use]
    pub fn new(event_store: E, event_bus: B) -> Self {
        Self {
            event_store,
            event_bus,
            public_event_log_cache: RwLock::new(PublicEventLogCache::default()),
            public_event_log_access_clock: AtomicU64::new(0),
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
            self.invalidate_public_event_log_cache(game_id)?;
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
    ) -> Result<Option<Arc<[PublicEventLogEntry]>>, DomainError> {
        let access_tick = self
            .public_event_log_access_clock
            .fetch_add(1, Ordering::Relaxed)
            + 1;
        let cache = self
            .public_event_log_cache
            .read()
            .map_err(|_| Self::cache_error("read cached public event logs"))?;
        let entries = cache.get(game_id, access_tick);
        drop(cache);
        Ok(entries)
    }

    pub(crate) fn store_public_event_log_cache(
        &self,
        game_id: &str,
        entries: Arc<[PublicEventLogEntry]>,
        estimated_bytes: usize,
    ) -> Result<(), DomainError> {
        let access_tick = self
            .public_event_log_access_clock
            .fetch_add(1, Ordering::Relaxed)
            + 1;
        let mut cache = self
            .public_event_log_cache
            .write()
            .map_err(|_| Self::cache_error("store a cached public event log"))?;
        cache.insert(game_id, entries, estimated_bytes, access_tick);
        drop(cache);
        Ok(())
    }

    fn invalidate_public_event_log_cache(&self, game_id: &str) -> Result<(), DomainError> {
        let mut cache = self
            .public_event_log_cache
            .write()
            .map_err(|_| Self::cache_error("invalidate a cached public event log"))?;
        cache.remove(game_id);
        drop(cache);
        Ok(())
    }
}
