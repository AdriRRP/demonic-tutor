# Slice Name

`MakePlayerHandleTheCanonicalRuntimeIdentity`

## Goal

Make `PlayerCardHandle` the canonical card identity inside the `Game` runtime so player-local logic no longer depends on public `CardInstanceId` lookups in hot paths.

## Why This Slice Exists Now

The runtime already relies heavily on dense arenas, zone-local handles, and aggregate-level indices. The remaining `CardInstanceId -> PlayerCardHandle` lookup inside `Player` is now one of the clearest identity taxes left in the core.

## Supported Behavior

- internal player-owned card access uses `PlayerCardHandle` as the canonical runtime identity
- public `CardInstanceId` continues to be accepted at aggregate boundaries where commands, events, serialization, and tests need it
- hot player-local zone and card access no longer require public-id hashing as their primary lookup model

## Invariants / Legality Rules

- public card ids remain stable and deterministic at aggregate boundaries
- each live handle still identifies exactly one player-owned runtime card slot
- this slice does not expand supported Magic rules

## Out of Scope

- replacing all public ids across the whole codebase
- changing command or event payloads
- changing supported gameplay behavior

## Domain Impact

### Aggregate Impact
- `Game` and `Player` identity flow becomes more explicitly handle-first internally

### Entity / Value Object Impact
- `Player`
- `PlayerCardArena`
- aggregate-local card location helpers

## Ownership Check

This behavior belongs to the `Game` aggregate because player-owned runtime identity is part of the aggregate’s authoritative gameplay state.

## Documentation Impact

- this slice document
- `docs/slices/proposals/README.md`

## Test Impact

- player-local lookups still succeed through public commands
- existing gameplay regressions stay green after the internal identity inversion

## Rules Reference

- no additional Comprehensive Rules scope; this is a runtime-identity refactor only

## Rules Support Statement

This slice does not broaden Magic rules support. It tightens the internal identity model behind the same supported gameplay subset.

## Open Questions

- none
