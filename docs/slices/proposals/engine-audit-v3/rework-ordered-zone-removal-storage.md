# Proposal Slice — Rework Ordered Zone Removal Storage

## Summary

Replace suffix-reindexing ordered-zone removals with a storage strategy that preserves observable order without O(n) positional rewrites on every remove.

## Motivation

- cut repeated reindexing cost in `Hand`, `Graveyard`, and `Exile`
- reduce bookkeeping churn as more effects move cards across ordered zones
- keep one shared abstraction for ordered id-backed storage

## Target Shape

- shared ordered-zone storage remains internal
- removals preserve observable order
- position tracking avoids full suffix rewrites on each removal

## Invariants

- visible hand and graveyard order stay truthful
- gameplay legality remains unchanged
- this slice does not expand supported Magic rules
