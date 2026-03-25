# Slice Name

`MakeAggregateLocationIndexPurelyTransactional`

## Status

Implemented

## Goal

Turn the aggregate card-location index into a purely transactional structure updated directly by known card moves, so player-wide refreshes disappear from normal runtime maintenance.

## What Changed

- `AggregateCardLocationIndex` is now built directly from initial player state instead of exposing player-refresh maintenance as part of normal flow
- `Game` no longer refreshes card locations before and after `advance_turn`
- turn progression now updates the aggregate location index only when an actual turn-step draw moves a known card
- explicit draw, discard, exile, stack resolution, and combat-death corridors continue updating the index through point synchronization

## Supported Behavior

- card location lookups continue working for battlefield, graveyard, exile, stack, and targeting corridors
- known card transitions update the location index immediately
- turn progression preserves correct indexed locations without player-wide refreshes

## Invariants / Legality Rules

- every live indexed card still resolves to one owner and one runtime location
- location updates stay aligned with supported card transitions
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
- turn-flow synchronization

## Ownership Check

This belongs to the `Game` aggregate because card location is aggregate-owned truth and must stay aligned with legal state transitions.

## Documentation Impact

- `docs/architecture/runtime-abstractions.md`
- `docs/slices/proposals/README.md`
- this implemented slice document

## Test Impact

- focused turn-progression and draw-effect regressions remain green
- full repo validation proves location lookups still stay aligned through supported card moves

## Rules Reference

- no additional Comprehensive Rules scope; this is aggregate-state maintenance refinement

## Rules Support Statement

This slice does not change supported gameplay rules. It makes aggregate-owned location truth more incremental and trustworthy.
