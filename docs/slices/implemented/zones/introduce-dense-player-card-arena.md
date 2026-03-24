# Implemented Slice — Introduce Dense Player Card Arena

## Summary

Replace the per-player hash-owned card store with a dense internal arena keyed by compact player-local handles.

## Supported Behavior

- player zones now store compact internal handles instead of public card ids
- public lookup by `CardInstanceId` still works through `Player` semantic accessors
- gameplay behavior and zone semantics remain unchanged

## Invariants

- zone ownership remains explicit and deterministic
- cards only move between zones through player-owned transitions
- this slice does not expand supported Magic rules

## Implementation Notes

- `Player` now owns a dense card arena plus an id-to-handle bridge
- `Library`, `Hand`, `Battlefield`, `Graveyard`, and `Exile` store `PlayerCardHandle`
- public ids remain available at aggregate boundaries and tests

## Tests

- full repository validation remains green after the storage migration
