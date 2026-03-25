# Slice Name

`MakePlayerHandleBoundaryOnlyCardIdentity`

## Status

Implemented

## Goal

Make `PlayerCardHandle` the canonical runtime card identity inside `Player` so `CardInstanceId` stops being the primary lookup key of the player-owned arena.

## What Changed

- `Player` no longer keeps a public-id-to-handle map as part of its runtime state
- `PlayerCardArena` remains the source of truth for owned cards and live handles
- internal zone and mutation flows continue operating through handles
- public-id-based compatibility queries remain available, but they are now boundary-facing lookups instead of the canonical runtime path

## Supported Behavior

- player-owned runtime state is addressed internally by `PlayerCardHandle`
- card lookup by `CardInstanceId` remains available for commands, tests, and outward-facing compatibility flows
- zone transitions, spell preparation, battlefield mutation, and aggregate lookups preserve current observable behavior

## Invariants / Legality Rules

- every runtime-owned card still has exactly one stable owner
- every live handle still resolves to exactly one card instance
- public card ids remain stable and observable at aggregate boundaries

## Out of Scope

- changing gameplay rules support
- changing public command payloads
- redesigning card-definition identity

## Domain Impact

### Aggregate Impact
- `Game` and `Player` internal card-access flow

### Entity / Value Object Impact
- player-owned card arena
- public-id compatibility lookups inside `Player`

## Ownership Check

This belongs to the `Game` aggregate because card identity, ownership, and zone membership are aggregate-owned runtime concerns.

## Documentation Impact

- `docs/architecture/runtime-abstractions.md`
- `docs/slices/proposals/README.md`
- this implemented slice document

## Test Impact

- focused player-model regressions still prove atomic spell extraction rollback
- focused player-model regressions still prove transactional zone rollback

## Rules Reference

- no additional Comprehensive Rules scope; this slice refines runtime identity only

## Rules Support Statement

This slice does not expand supported Magic rules. It tightens the internal identity model behind the already supported gameplay subset.
