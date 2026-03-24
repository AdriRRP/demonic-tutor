# Proposal Slice — Rework Ordered Zone Visible Indexing

## Summary

Replace filtered visible scans in ordered zones with a structure whose indexing and iteration cost tracks visible cards more directly.

## Motivation

- remove the remaining filtered-scan cost in `handle_at()` and visible iteration
- keep hand, graveyard, and exile efficient under repeated movement
- move closer to a storage model whose cost follows live cards instead of recent history

## Target Shape

- visible indexing and visible iteration no longer depend on scanning sparse physical storage
- ordered-zone semantics remain hidden behind the same player-facing APIs
- the implementation stays shared across ordered player-owned zones

## Invariants

- hand, graveyard, and exile preserve observable order
- the change stays internal to zone storage
- this slice does not expand supported Magic rules
