# Implemented Slice — Make Player Handle The Canonical Runtime Identity

## Summary

Push the player runtime farther toward a handle-first model so player-local card transitions and aggregate-assisted zone work operate on `PlayerCardHandle` internally while public `CardInstanceId` stays at true boundaries.

## Supported Behavior

- player-local hot paths now prefer `PlayerCardHandle` for internal battlefield and zone transitions
- aggregate-assisted exile and creature-destruction corridors reuse owner plus handle instead of re-resolving public ids through player-local hashing
- outward-facing commands, events, errors, and tests still expose stable public card ids

## Invariants

- public card ids remain stable and deterministic at aggregate boundaries
- each live handle still maps to exactly one player-owned runtime card slot
- this slice does not expand supported Magic rules

## Implementation Notes

- `Player` now exposes a smaller set of handle-first helpers for internal zone transitions and battlefield iteration
- state-based actions and stack-resolution effects use owner-plus-handle paths when the aggregate already knows the target location
- explicit exile commands now prefer the aggregate location index and only fall back to public-id lookup when needed for a user-facing boundary path

## Tests

- full repository validation remains green after the handle-first player refactor
