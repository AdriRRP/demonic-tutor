# Slice Name

`RemoveLinearVisibleZoneReindexOnOrderedRemoval`

## Status

Implemented

## Goal

Refine ordered zone storage so removing a visible card no longer rewrites the entire visible suffix while preserving deterministic hand, graveyard, and exile order.

## What Changed

- ordered visible zones now keep sparse visible-position entries instead of compacting the visible suffix on every remove
- visible-position lookup uses an index structure that can skip removed positions without rescanning the linked order chain
- ordered removals now clear the removed visible position in place instead of rewriting every later visible index

## Supported Behavior

- hand, graveyard, and exile preserve visible order
- visible-position lookup remains efficient
- ordered removal avoids suffix-wide visible-index rewrites in the common path

## Invariants / Legality Rules

- visible order remains deterministic
- zone membership remains explicit
- battlefield semantics remain unchanged

## Out of Scope

- redesigning battlefield storage
- changing library ordering semantics
- changing public zone APIs

## Domain Impact

### Entity / Value Object Impact
- ordered zone storage used by hand, graveyard, and exile

## Ownership Check

This belongs to the gameplay domain because visible zone order is aggregate-owned runtime state.

## Documentation Impact

- `docs/domain/aggregate-game.md`
- `docs/architecture/runtime-abstractions.md`
- `docs/slices/proposals/README.md`
- this implemented slice document

## Test Impact

- ordered-zone regressions covering repeated removals and insertions
- focused storage regressions proving visible order stays stable without suffix-wide maintenance

## Rules Reference

- no additional Comprehensive Rules scope; this is a storage refinement behind existing zone behavior

## Rules Support Statement

This slice preserves the current supported zone subset while tightening the mutation cost of ordered visible zones.
