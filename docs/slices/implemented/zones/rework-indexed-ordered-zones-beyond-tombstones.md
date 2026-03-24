# Implemented Slice — Rework Indexed Ordered Zones Beyond Tombstones

## Summary

Keep ordered-zone removals cheap in the common path while compacting historical tombstones before sparse history can dominate visible zone operations.

## Supported Behavior

- hand, graveyard, and exile still preserve observable order
- removals still avoid eager suffix reindexing in the common case
- sparse historical tombstones are compacted away before they dominate zone scans

## Invariants

- player-facing zone semantics remain unchanged
- the ordered-zone storage stays internal and shared
- this slice does not expand supported Magic rules

## Implementation Notes

- `IndexedOrderedZone` now compacts itself when tombstones become as numerous as live entries
- empty ordered zones also release historical storage immediately
- visible iteration and indexed access now scale with bounded sparsity instead of unbounded history

## Tests

- full repository validation remains green after the ordered-zone storage refinement
