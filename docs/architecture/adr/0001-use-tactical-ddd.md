# ADR 0001 — The core gameplay model uses tactical DDD

## Status
Accepted

## Context

DemonicTutor models a domain with explicit state transitions, gameplay legality rules and observable behavior.

The system must represent:

- running game sessions
- player actions and legality checks
- card movement across zones
- domain events produced by gameplay
- replayability and analytics derived from play

Without a clear modeling approach, the implementation risks mixing:

- gameplay rules
- UI concerns
- infrastructure concerns
- analytics concerns

This would weaken the model and make gameplay behavior harder to evolve and test.

The project therefore requires a modeling approach that protects domain clarity and explicit boundaries.

## Decision

The core gameplay model will use **tactical Domain-Driven Design**.

Within the `play` bounded context the system will explicitly model:

- aggregates
- entities
- value objects
- commands expressing intent
- domain events describing state transitions

The domain model will remain independent from UI, infrastructure and analytics concerns.

Gameplay legality and state transitions must be enforced inside the domain model.

## Consequences

### Positive

- consistent ubiquitous language
- clear ownership of gameplay rules
- stronger domain invariants
- improved testability of gameplay behavior
- natural alignment with event-driven design

### Negative

- additional modeling discipline required
- more explicit domain structures
- risk of over-engineering if slice scope is not controlled

## Notes

DDD is applied pragmatically.

The goal is not architectural ceremony, but protecting clarity and correctness in the gameplay domain.
