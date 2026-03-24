# Slice Name

`ReworkVisibleZoneIndexingForSublinearPositionLookup`

## Goal

Improve ordered zone indexing so visible-position lookups do not require walking the linked list from `head` on each `handle_at()` query.

## Why This Slice Exists Now

The current zone storage already removed the worst-case suffix rewrite on deletion. The remaining clear weakness is visible indexing by position, which is still linear.

## Supported Behavior

- hand, graveyard, and exile continue preserving visible insertion order
- removal semantics remain stable
- visible-position lookup becomes cheaper than the current linear walk

## Invariants / Legality Rules

- visible zone order remains deterministic
- zone membership remains explicit and reviewable
- this slice does not expand supported Magic rules

## Out of Scope

- changing battlefield storage semantics
- changing public zone behavior
- redesigning library ordering

## Domain Impact

### Entity / Value Object Impact
- ordered zone storage used by `Hand`, `Graveyard`, and `Exile`

## Ownership Check

This belongs to the gameplay domain because zone ordering is part of aggregate-owned runtime state.

## Documentation Impact

- this slice document
- `docs/slices/proposals/README.md`

## Test Impact

- ordered-zone regressions for visible order and slot reuse remain green
- focused regression for stable visible indexing after repeated removals and insertions

## Rules Reference

- no additional Comprehensive Rules scope; this is storage refinement behind existing zone behavior

## Rules Support Statement

This slice preserves the current supported zone subset while tightening the performance characteristics of visible indexing.

## Open Questions

- none
