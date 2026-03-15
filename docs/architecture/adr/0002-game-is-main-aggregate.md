# ADR 0002 — Game is the main aggregate of the play context

## Status
Accepted

## Context

Gameplay actions depend on globally coherent game state.

Examples include:

- turn progression
- phase progression
- player actions
- card movement between zones
- validation of gameplay legality

If these responsibilities are split across multiple aggregates too early, the system risks introducing:

- fragmented invariants
- unclear ownership of legality checks
- orchestration complexity before the domain model is stable

The early system therefore requires a single authority responsible for protecting gameplay invariants.

## Decision

In the `play` bounded context, `Game` will be the primary aggregate root.

`Game` is responsible for enforcing gameplay invariants related to:

- turn progression
- phase progression
- player participation
- card movement between zones
- validation of gameplay actions
- emission of domain events resulting from accepted commands

Commands that affect gameplay state must be evaluated by the `Game` aggregate.

## Consequences

### Positive

- clear ownership of gameplay legality
- simpler consistency model
- easier command handling in early vertical slices
- deterministic state transitions

### Negative

- the aggregate may grow in scope
- careful modeling discipline is required
- future refactoring may be needed as new mechanics are introduced

## Notes

This decision defines the initial aggregate strategy.

Future iterations may introduce additional aggregates if the gameplay model grows beyond what a single aggregate can safely manage.

Deck management remains outside the `play` context.

Analytics concerns remain observational and must not influence gameplay state.
