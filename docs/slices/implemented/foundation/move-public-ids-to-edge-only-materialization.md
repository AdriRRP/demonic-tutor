# Implemented Slice — Move Public Ids To Edge-Only Materialization

## Summary

Push public id materialization farther toward aggregate edges so hot internal paths can rely more on player-local handles and player indices.

## Supported Behavior

- gameplay behavior remains unchanged
- zone ownership and target lookup still expose stable public ids at command, event, and test boundaries
- internal ownership checks in hot paths now rely less on cloned public ids

## Invariants

- domain identity remains explicit and deterministic
- public-facing ids stay stable at aggregate boundaries
- this slice does not expand supported Magic rules

## Implementation Notes

- target-location helpers now prefer `player_index` over carrying cloned `PlayerId`
- resolution paths materialize `PlayerId` only when emitting or crossing outward-facing boundaries
- this is an incremental step toward compact edge-only public ids, not the final replacement of public string ids inside the whole engine

## Tests

- full repository validation remains green after the ownership-path refactor
