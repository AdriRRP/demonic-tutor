# Slice 7 — Infrastructure

## Goal

Add basic event-driven infrastructure to support:
- EventBus for pub/sub of domain events
- EventStore for persistence (in-memory)
- GameLogProjection for human-readable game event log
- Generic GameService that uses EventStore and EventBus

## Supported behavior

### EventBus
- `subscribe(handler)` - register an event handler
- `publish(event)` - notify all subscribers
- Synchronous, in-process

### EventStore
- `append(aggregate_id, events)` - persist events
- `get_events(aggregate_id)` - retrieve events for replay
- In-memory, vector-based

### GameLogProjection
- Maintains a `Vec<String>` with human-readable event descriptions
- Updates on each published event
- Inspectable for debugging or future serialization

### GameService (updated)
- Generic over `EventStore` and `EventBus` ports
- Persists events after each command execution
- Publishes events to the bus for subscribers
- Returns domain events to callers

## Architecture

```
Application(Ports) → Domain ← Infrastructure(Implementations)

src/application/
  └── ports.rs      # EventBus, EventStore ports

src/infrastructure/
  ├── events/
  │   ├── bus.rs     # InMemoryEventBus
  │   └── store.rs   # InMemoryEventStore
  └── projections/
      └── game_log.rs # GameLogProjection

src/domain/play/
  └── events.rs     # DomainEvent enum
```

Following ADR 0003:
- Aggregates emit events (return values)
- Application service orchestrates persistence and publication
- Domain remains decoupled from infrastructure

## Invariants

- EventBus handlers must not panic
- EventStore must append atomically
- Projection must handle all event types gracefully

## Out of scope

- Serialization (JSON, binary)
- Persistence to disk or database
- Multiple projections
- Async handling
- Event replay to reconstruct aggregate state
- CommandBus

## Tests

| Test | Description |
|------|-------------|
| `event_bus_publishes_to_handler` | Handler receives published event |
| `event_bus_multiple_handlers` | All handlers receive event |
| `event_store_append_and_retrieve` | Events can be stored and loaded |
| `projection_logs_events` | GameLogProjection accumulates event strings |

## Notes

This slice establishes the infrastructure foundation for future slices:
- Analytics projections
- Event sourcing
- Game replay
- Persistence layers
