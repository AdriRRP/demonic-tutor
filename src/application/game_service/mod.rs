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
    recency_nodes: Vec<CacheRecencyNode>,
    free_recency_nodes: Vec<usize>,
    oldest: Option<usize>,
    newest: Option<usize>,
    total_estimated_bytes: usize,
}

#[derive(Debug)]
struct CachedPublicEventLog {
    entries: Arc<[PublicEventLogEntry]>,
    estimated_bytes: usize,
    recency_node: usize,
}

#[derive(Debug, Default)]
struct CacheRecencyNode {
    game_id: String,
    previous: Option<usize>,
    next: Option<usize>,
}

impl PublicEventLogCache {
    fn get(&self, game_id: &str) -> Option<Arc<[PublicEventLogEntry]>> {
        self.entries
            .get(game_id)
            .map(|cached| Arc::clone(&cached.entries))
    }

    fn insert(&mut self, game_id: &str, entries: Arc<[PublicEventLogEntry]>) {
        let estimated_bytes = approximate_public_event_log_bytes(entries.as_ref());
        let recency_node = if let Some(previous) = self.entries.remove(game_id) {
            self.total_estimated_bytes = self
                .total_estimated_bytes
                .saturating_sub(previous.estimated_bytes);
            self.unlink_recency_node(previous.recency_node);
            self.recycle_recency_node(previous.recency_node);
            self.allocate_recency_node(game_id)
        } else {
            self.allocate_recency_node(game_id)
        };
        self.total_estimated_bytes += estimated_bytes;
        self.link_recency_node_as_newest(recency_node);
        self.entries.insert(
            game_id.to_string(),
            CachedPublicEventLog {
                entries,
                estimated_bytes,
                recency_node,
            },
        );

        while self.entries.len() > PUBLIC_EVENT_LOG_CACHE_CAPACITY
            || self.total_estimated_bytes > PUBLIC_EVENT_LOG_CACHE_MAX_BYTES
        {
            let Some(oldest_node) = self.pop_oldest_recency_node() else {
                break;
            };
            let oldest_game_id = self.recency_nodes[oldest_node].game_id.clone();
            if let Some(evicted) = self.entries.remove(&oldest_game_id) {
                self.total_estimated_bytes = self
                    .total_estimated_bytes
                    .saturating_sub(evicted.estimated_bytes);
                self.recycle_recency_node(oldest_node);
            }
        }
    }

    fn remove(&mut self, game_id: &str) {
        if let Some(removed) = self.entries.remove(game_id) {
            self.total_estimated_bytes = self
                .total_estimated_bytes
                .saturating_sub(removed.estimated_bytes);
            self.unlink_recency_node(removed.recency_node);
            self.recycle_recency_node(removed.recency_node);
        }
    }

    fn allocate_recency_node(&mut self, game_id: &str) -> usize {
        if let Some(index) = self.free_recency_nodes.pop() {
            self.recency_nodes[index] = CacheRecencyNode {
                game_id: game_id.to_string(),
                previous: None,
                next: None,
            };
            return index;
        }

        self.recency_nodes.push(CacheRecencyNode {
            game_id: game_id.to_string(),
            previous: None,
            next: None,
        });
        self.recency_nodes.len() - 1
    }

    fn recycle_recency_node(&mut self, node_index: usize) {
        self.recency_nodes[node_index] = CacheRecencyNode::default();
        self.free_recency_nodes.push(node_index);
    }

    fn link_recency_node_as_newest(&mut self, node_index: usize) {
        self.recency_nodes[node_index].previous = self.newest;
        self.recency_nodes[node_index].next = None;

        if let Some(previous_newest) = self.newest {
            self.recency_nodes[previous_newest].next = Some(node_index);
        } else {
            self.oldest = Some(node_index);
        }
        self.newest = Some(node_index);
    }

    fn unlink_recency_node(&mut self, node_index: usize) {
        let previous = self.recency_nodes[node_index].previous;
        let next = self.recency_nodes[node_index].next;

        if let Some(previous_index) = previous {
            self.recency_nodes[previous_index].next = next;
        } else {
            self.oldest = next;
        }

        if let Some(next_index) = next {
            self.recency_nodes[next_index].previous = previous;
        } else {
            self.newest = previous;
        }

        self.recency_nodes[node_index].previous = None;
        self.recency_nodes[node_index].next = None;
    }

    fn pop_oldest_recency_node(&mut self) -> Option<usize> {
        let oldest = self.oldest?;
        self.unlink_recency_node(oldest);
        Some(oldest)
    }
}

fn approximate_public_event_log_bytes(entries: &[PublicEventLogEntry]) -> usize {
    // Coarse byte budget: the contiguous slice backing the cached projection.
    size_of_val(entries)
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
