# Proposal Slice — Rework Indexed Ordered Zones Beyond Tombstones

## Summary

Replace the current tombstone-based ordered-zone storage with a strategy that preserves order without degrading as zone history grows.

## Motivation

- remove the new historical-scan tax introduced by indefinite tombstones
- keep hand, graveyard, and exile operations proportional to live cards instead of lifetime movement
- preserve the semantic benefits of ordered zones while improving throughput

## Target Shape

- ordered-zone lookup and removal no longer depend on indefinite tombstone accumulation
- visible iteration and indexed access remain explicit and semantically ordered
- the shared ordered-zone abstraction remains internal and reusable across supported zones

## Invariants

- hand, graveyard, and exile preserve their observable order semantics
- the change stays hidden behind existing player-facing APIs
- this slice does not expand supported Magic rules
