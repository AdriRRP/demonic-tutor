# Slice Name

`MakeAggregateCardLocationIndexTrulyIncremental`

## Goal

Turn the aggregate card-location index into a live structure updated by concrete zone transitions instead of refreshing a whole player snapshot after each player-local change.

## Why This Slice Exists Now

The current index already avoids global rebuilds, but it still drains and rebuilds all locations for an owner whenever that player changes. The next truthful optimization is transition-local maintenance.

## Supported Behavior

- aggregate card locations are updated incrementally when cards move or change zones
- player-wide refresh snapshots are no longer the default maintenance path
- targeting and location-sensitive gameplay behavior remain unchanged

## Invariants / Legality Rules

- every indexed card location still points to exactly one owner, handle, and visible zone
- zone transitions remain authoritative in `Game`
- this slice does not expand supported Magic rules

## Out of Scope

- redesigning all player-zone APIs at once
- introducing concurrency
- changing targeting semantics

## Domain Impact

### Aggregate Impact
- `Game` transition helpers
- aggregate card-location index maintenance

### Entity / Value Object Impact
- `AggregateCardLocationIndex`
- player-zone transition helpers that must publish location updates

## Ownership Check

This belongs to the `Game` aggregate because cross-player card location is aggregate-owned gameplay truth.

## Documentation Impact

- this slice document
- `docs/slices/proposals/README.md`

## Test Impact

- location-sensitive targeting and exile/destroy flows remain green
- focused regressions proving location index updates stay correct across single-card transitions

## Rules Reference

- no additional Comprehensive Rules scope; this is runtime bookkeeping behind existing supported behavior

## Rules Support Statement

This slice does not broaden rules support. It makes location bookkeeping more incremental behind the existing subset.

## Open Questions

- none
