# Slice Name

`MakeAggregateCardLocationIndexTrulyIncremental`

## Status

Implemented

## Goal

Turn the aggregate card-location index into a live structure updated by concrete zone transitions instead of refreshing a whole player snapshot after each player-local change.

## What Changed

- `AggregateCardLocationIndex` keeps its bootstrap and broad refresh helpers, but `Game` now updates it through transition-local `upsert` and `remove` calls on concrete card movements.
- `cast_spell`, `pass_priority`, `play_land`, `draw_cards_effect`, `discard_for_cleanup`, `exile_card`, `adjust_player_life_effect`, and `resolve_combat_damage` now synchronize the aggregate location index from the card-specific outcomes they already emit.
- `Game` gained a small helper to resync one card from a player-owned handle and zone when a transition completes.

## Supported Behavior

- aggregate card locations are updated incrementally when cards move or change zones
- player-wide refresh snapshots are no longer the default maintenance path for single-card transitions
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
- player-zone transition helpers that publish location updates

## Ownership Check

This belongs to the `Game` aggregate because cross-player card location is aggregate-owned gameplay truth.

## Documentation Impact

- this implemented slice document
- `docs/slices/proposals/README.md`

## Test Impact

- location-sensitive targeting and exile/destroy flows remain green
- full repository validation stays green after removing the default player-snapshot refreshes from the main single-card transition corridors

## Rules Reference

- no additional Comprehensive Rules scope; this remains runtime bookkeeping behind existing supported behavior

## Rules Support Statement

This slice does not broaden rules support. It makes location bookkeeping more incremental behind the existing subset.
