# ADR 0012 — Game aggregate internals are organized as model, rules, and invariants

## Status
Accepted

## Context

As the `Game` aggregate accumulated more supported slices, its internal implementation had been split into many flat files under the Play context.

That kept files short, but it also created a few problems:

- aggregate-owned state, invariants, and gameplay rules were mixed at the same directory level
- the folder started to read like a list of verbs rather than a coherent aggregate structure
- the old `helpers.rs` name was semantically weak for code that actually enforced aggregate legality

The project still wants a single `Game` aggregate, explicit command methods, and minimal architectural overhead.

## Decision

The internal structure of `src/domain/play/game/` will be organized into three categories:

- `model/` for aggregate-owned state and entities
- `rules/` for gameplay behavior grouped by domain capability
- `invariants.rs` for aggregate legality checks and internal lookup helpers

`mod.rs` remains the aggregate facade and keeps the explicit command entrypoints on `Game`.

## Consequences

### Positive

- clearer separation between aggregate state, legality checks, and gameplay behavior
- stronger semantic naming than a generic `helpers.rs`
- easier navigation without introducing new aggregates or extra abstraction layers
- a structure that scales better as turn flow and combat grow in complexity

### Negative

- some behavior that used to live in small single-purpose files now shares broader rule modules
- internal imports become slightly deeper than in the previous flat layout
- future growth may still require another reorganization inside `rules/` if a capability becomes large

## Notes

This decision refines the aggregate modularization guidance in `docs/domain/aggregate-game.md` and `docs/architecture/game-aggregate-structure.md`.
