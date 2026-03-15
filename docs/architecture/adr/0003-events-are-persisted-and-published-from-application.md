# ADR 0003 — Domain events are persisted and published from the application layer

## Status
Accepted

## Context

The system produces domain events representing gameplay state transitions.

These events must support:

- deterministic gameplay behavior
- replayability of game sessions
- persistence of gameplay history
- projections and analytics derived from play

If aggregates publish events directly, the domain model becomes coupled to:

- event bus concerns
- infrastructure timing
- side effects and orchestration

This would introduce infrastructure dependencies inside the domain model and make domain behavior harder to test and reason about.

A clear boundary is therefore required between **event production** and **event publication**.

## Decision

Aggregates produce domain events as the result of command handling.

Aggregates **do not publish events directly**.

The application layer is responsible for orchestrating event handling.

Application services perform the following steps:

1. load or create the aggregate
2. execute the command
3. collect the domain events produced by the aggregate
4. persist the events
5. publish them to the event bus
6. trigger projections or other side effects

The domain model remains independent from infrastructure concerns.

## Consequences

### Positive

- domain model remains infrastructure-free
- domain logic is easier to unit test
- event persistence and publication are explicit
- deterministic command execution flow
- clear separation between domain and application responsibilities

### Negative

- the application layer becomes responsible for orchestration
- command handling requires explicit plumbing
- some additional boilerplate may be introduced

## Notes

The initial implementation uses an in-process event bus.

More advanced infrastructure patterns (for example actor-based systems or distributed messaging) may be introduced later without changing the domain model.
