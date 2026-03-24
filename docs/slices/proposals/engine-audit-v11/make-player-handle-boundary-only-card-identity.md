# Slice Name

`MakePlayerHandleBoundaryOnlyCardIdentity`

## Status

Proposed

## Goal

Make `PlayerCardHandle` the only canonical runtime card identity inside the player aggregate so `CardInstanceId` becomes a boundary concern rather than the primary lookup key of the card arena.

## Why This Slice Exists Now

The current runtime already moves most hot paths through `owner_index + handle`, but `Player` still keeps `id_to_handle` as the main lookup door for card access. That keeps public ids, hashing, and clone pressure in the core of the aggregate even though the surrounding architecture is already handle-first.

## Supported Behavior

- player-owned runtime state is addressed internally by `PlayerCardHandle`
- card lookup by `CardInstanceId` is restricted to aggregate boundaries, commands, events, and compatibility helpers
- internal zone, combat, stack, and SBA corridors continue working through handles without observable gameplay change

## Invariants / Legality Rules

- every runtime-owned card still has exactly one stable owner
- every live handle still resolves to exactly one card instance
- public card ids remain stable and observable at the aggregate boundary

## Out of Scope

- changing gameplay rules support
- changing public command payloads
- redesigning card-definition identity

## Domain Impact

### Aggregate Impact
- `Game` and `Player` internal card access flow

### Entity / Value Object Impact
- player-owned card arena
- aggregate location carriers that still re-enter through public card ids

## Ownership Check

This belongs to the `Game` aggregate because card identity, ownership, and zone membership are aggregate-owned runtime concerns.

## Documentation Impact

- `docs/domain/aggregate-game.md`
- `docs/domain/current-state.md`
- `docs/architecture/runtime-abstractions.md`
- this slice document

## Test Impact

- focused regressions proving internal card flows still work when handles are the canonical runtime identity
- compatibility regressions proving public ids remain stable at commands, events, and read-side queries

## Rules Reference

- no additional Comprehensive Rules scope; this slice refines runtime identity only

## Rules Support Statement

This slice does not expand supported Magic rules. It tightens the internal identity model behind the already supported gameplay subset.
