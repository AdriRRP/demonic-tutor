# Implemented Slice — Rework Ordered Zone Removal Storage

## Summary

Replace suffix-reindexing ordered-zone removals with tombstone-backed ordered storage that preserves visible order without rewriting every later position.

## Supported Behavior

- `Hand`, `Graveyard`, and `Exile` still preserve observable insertion order
- membership and removals stay semantically unchanged
- positional access still reflects visible order, skipping removed entries

## Invariants

- visible ordered-zone semantics remain truthful
- removals do not reorder surviving cards
- this slice does not expand supported Magic rules

## Implementation Notes

- the shared ordered-zone storage now keeps tombstones for removed entries
- membership stays indexed through a handle-to-position map
- removals no longer rewrite the full suffix of later positions

## Tests

- full repository validation remains green after the storage change
