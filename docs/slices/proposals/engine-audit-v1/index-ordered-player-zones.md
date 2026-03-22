# Slice Proposal — Index Ordered Player Zones

## Goal

Add explicit membership/index support to ordered player-owned zones so common `contains` and `remove` operations stop depending on repeated linear scans.

## Why This Slice Exists Now

After moving hand, battlefield, graveyard, and exile to id-backed carriers, many reads still do `zone.contains(card_id)` and then a second lookup in the player store. The next honest performance step is to make ordered zones cheaper without changing their semantics.

## Supported Behavior

- ordered zones keep current visible order semantics
- membership and removal become explicit zone responsibilities
- player accessors stop re-checking the same card id through multiple linear scans

## Invariants / Legality Rules

- zone ordering remains unchanged
- zone membership remains unique per player-owned zone
- no gameplay behavior changes

## Out of Scope

- global card indexing
- stack indexing
- battlefield control changes across players

## Domain Impact

- enrich ordered zone carriers with explicit membership/index support
- simplify `Player` zone accessors so they rely on one zone answer instead of a scan plus a store check

## Ownership Check

Zone membership and lookup remain aggregate-owned runtime behavior under `Player`.

## Documentation Impact

- `docs/domain/aggregate-game.md` only if zone responsibilities need clarification
- this proposal file

## Test Impact

- unit tests for ordered zone membership and removal semantics
- regression tests around hand discard, draw, and battlefield removal

## Rules Reference

- none beyond current supported zone semantics

## Rules Support Statement

This slice optimizes zone lookup and removal only. It does not imply new gameplay rules.
