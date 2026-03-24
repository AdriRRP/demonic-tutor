# Slice Name

`ReworkVisibleZoneIndexingForSublinearPositionLookup`

## Status

Implemented

## Goal

Improve ordered zone indexing so visible-position lookups do not require walking the linked list from `head` on each `handle_at()` query.

## What Changed

- ordered zone storage now keeps an explicit visible-slot index alongside linked slot topology
- `handle_at()` for `Hand`, `Graveyard`, and `Exile` now resolves through that visible index instead of traversing from `head`
- removal keeps visible order stable while refreshing the visible-slot index after deletions
- focused regression now proves repeated removals and insertions preserve visible order and indexed lookup stability

## Supported Behavior

- hand, graveyard, and exile continue preserving visible insertion order
- removal semantics remain stable
- visible-position lookup is now direct instead of linear from the list head

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

- this implemented slice document
- `docs/slices/proposals/README.md`

## Test Impact

- ordered-zone regressions for visible order and slot reuse remain green
- focused regression now proves stable visible indexing after repeated removals and insertions

## Rules Reference

- no additional Comprehensive Rules scope; this is storage refinement behind existing zone behavior

## Rules Support Statement

This slice preserves the current supported zone subset while tightening the performance characteristics of visible indexing.
