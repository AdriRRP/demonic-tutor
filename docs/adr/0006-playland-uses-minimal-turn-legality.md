# ADR 0006 — PlayLand uses minimal turn legality in slice 3

## Status

Accepted

## Context

The third slice introduces the first player-driven gameplay action.

Completely ignoring turn and phase legality would weaken the semantic meaning of "playing a land" and make the domain model misleading.

At the same time, implementing full turn structure, priority and stack handling at this stage would introduce unnecessary complexity.

## Decision

For slice 3, the system introduces a minimal legality model for land play consisting of:

* one active player
* one minimal valid phase: `Phase::Main`
* one-land-per-turn enforcement via `lands_played_this_turn`

## Consequences

### Positive

* preserves meaningful domain legality
* keeps the slice narrow
* avoids premature turn-system complexity
* establishes a clean base for future turn progression

### Negative

* the turn model remains incomplete
* future slices will need to revisit and expand this model
* some real-game timing nuances are intentionally unsupported

## Notes

This decision is intentionally incremental and may be superseded later.
