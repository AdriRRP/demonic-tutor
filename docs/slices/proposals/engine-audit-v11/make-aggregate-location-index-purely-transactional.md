# Slice Name

`MakeAggregateLocationIndexPurelyTransactional`

## Status

Proposed

## Goal

Turn the aggregate card-location index into a purely transactional structure updated directly by zone transitions, so player-wide refreshes become fallback-only instead of routine maintenance.

## Why This Slice Exists Now

The location index already improved from full rebuilds to player-level refreshes, but that still treats it like a snapshot that gets regenerated after change. The next coherent step is to make each semantically known movement update the index directly so the aggregate keeps a live location model at all times.

## Supported Behavior

- card location lookups continue working for battlefield, graveyard, exile, stack, and targeting corridors
- transitions that move a known card update the location index immediately
- fallback rebuild or refresh paths remain exceptional safety tools rather than normal maintenance

## Invariants / Legality Rules

- every live indexed card still resolves to one owner and one runtime location
- location updates stay aligned with zone transitions
- missing or unsupported cards are not implied to have indexed locations

## Out of Scope

- adding new zones
- changing public query shape
- broad event redesign

## Domain Impact

### Aggregate Impact
- `Game` transition orchestration for card movement

### Entity / Value Object Impact
- aggregate card-location index
- player zone transition helpers

## Ownership Check

This belongs to the `Game` aggregate because card location is aggregate-owned truth and must stay aligned with legal state transitions.

## Documentation Impact

- `docs/domain/aggregate-game.md`
- `docs/architecture/runtime-abstractions.md`
- this slice document

## Test Impact

- transition-focused regressions proving the index stays aligned through supported card moves
- safety regressions proving fallback refresh remains unnecessary in normal paths

## Rules Reference

- no additional Comprehensive Rules scope; this is aggregate-state maintenance refinement

## Rules Support Statement

This slice does not change supported gameplay rules. It makes aggregate-owned location truth more incremental and trustworthy.
