# ADR 0001 — Use tactical DDD

## Status
Accepted

## Context

DemonicTutor has a meaningful core domain with explicit concepts, state transitions, invariants and observable behavior.

The system is not just a UI over card data. It must model:
- game sessions
- legal actions
- domain events
- replayability
- analytics derived from play

Without a clear modeling approach, the project risks mixing:
- game rules
- UI concerns
- infrastructure concerns
- analytics concerns

This would make the system harder to evolve, test and reason about.

## Decision

The project will use tactical Domain-Driven Design for the core model.

This means the early design will explicitly use concepts such as:
- bounded contexts
- aggregates
- value objects
- commands
- events
- projections

The domain model will remain separate from UI and infrastructure concerns.

## Consequences

### Positive
- better language consistency
- clearer boundaries
- stronger domain model
- easier testing of rules and invariants
- better fit for event-driven and replayable behavior

### Negative
- more up-front modeling effort
- higher discipline required in naming and boundaries
- risk of over-engineering if scope is not controlled

## Notes

DDD will be applied pragmatically.

The goal is not to maximize ceremony.
The goal is to protect clarity and correctness in the game domain.
