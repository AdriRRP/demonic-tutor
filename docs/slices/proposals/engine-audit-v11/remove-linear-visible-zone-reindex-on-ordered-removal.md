# Slice Name

`RemoveLinearVisibleZoneReindexOnOrderedRemoval`

## Status

Proposed

## Goal

Refine ordered zone storage so removing a visible card no longer rewrites the entire visible suffix while preserving deterministic hand, graveyard, and exile order.

## Why This Slice Exists Now

The current zone storage already fixed the expensive positional lookup, but ordered removals still pay linear visible reindexing after each deletion. That means the cost moved rather than disappeared. Closing this debt would make ordered zone mutation finally match the rest of the runtime’s compact, handle-first direction.

## Supported Behavior

- hand, graveyard, and exile continue preserving visible order
- visible-position lookup stays efficient
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
- this slice document

## Test Impact

- ordered-zone regressions covering repeated removals and insertions
- focused performance-shape regressions proving visible order stays stable without suffix-wide maintenance

## Rules Reference

- no additional Comprehensive Rules scope; this is a storage refinement behind existing zone behavior

## Rules Support Statement

This slice preserves the current supported zone subset while tightening the mutation cost of ordered visible zones.
