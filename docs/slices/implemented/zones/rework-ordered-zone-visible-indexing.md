# Implemented Slice — Rework Ordered Zone Visible Indexing

## Summary

Rework ordered player-zone storage so visible positional access and iteration read directly from the live ordered sequence instead of filtering sparse physical storage.

## Supported Behavior

- `Hand`, `Graveyard`, and `Exile` keep their visible insertion order
- `iter()` and `handle_at()` now read direct visible order
- order-preserving removals still preserve observable card order

## Invariants

- player-owned ordered zones remain truthful to the visible game state
- gameplay legality remains unchanged
- this slice does not expand supported Magic rules

## Implementation Notes

- `IndexedOrderedZone` now stores one visible handle sequence plus its position index
- positional lookup is no longer coupled to sparse history or filtered tombstone scans

## Tests

- full draw, discard, graveyard, exile, targeting, and BDD regression coverage remains green
