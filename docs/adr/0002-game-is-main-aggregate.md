# ADR 0002 — Game is the main aggregate in the play context

## Status
Accepted

## Context

In early iterations, many relevant actions depend on globally coherent game state.

Examples include:
- whose turn it is
- current phase
- who has priority
- where card instances are
- whether an action is legal now

If these concerns are split too early across multiple aggregates, the project may introduce:
- fragmented invariants
- unclear ownership of legal action checks
- orchestration complexity before the domain is stable

## Decision

In the `play` bounded context, `Game` will be the main aggregate root for early milestones.

`Game` will be responsible for protecting the most important invariants related to:
- turn progression
- phase progression
- priority-aware legality
- zone-aware action validity
- event emission from accepted commands

This does not imply that every future concern must remain inside `Game`.
It only defines the initial aggregate strategy.

## Consequences

### Positive
- simpler initial consistency model
- clearer ownership of game legality
- easier command handling in early vertical slices
- better fit for replayable event history

### Negative
- risk of aggregate growth over time
- requires scope discipline
- future refactoring may be needed as the model matures

## Notes

`Deck` remains a separate aggregate with its own lifecycle and responsibilities.
Analytics concerns do not belong inside `Game`.
