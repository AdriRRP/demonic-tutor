# Proposal Slice — Rework Ordered Zone Visible Indexing

## Summary

Replace the remaining filtered visible scans in ordered zones with a structure whose visible indexing cost tracks live ordered cards directly.

## Motivation

- remove filtered scans from `handle_at()` and visible iteration
- keep hand, graveyard, and exile efficient under repeated movement
- improve predictability for embedded-class runtime goals

## Target Shape

- visible indexing and iteration no longer depend on scanning sparse physical storage
- ordered-zone semantics remain hidden behind the same player-facing APIs
- the implementation stays shared across ordered zones

## Invariants

- hand, graveyard, and exile preserve observable order
- the change remains internal to storage
- this slice does not expand supported Magic rules
