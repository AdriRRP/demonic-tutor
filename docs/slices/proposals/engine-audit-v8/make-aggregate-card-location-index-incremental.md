# Proposal Slice — Make Aggregate Card Location Index Incremental

## Summary

Stop rebuilding `AggregateCardLocationIndex` from scratch after each public game operation and maintain it incrementally alongside semantic zone transitions.

## Motivation

- remove full-player rescans from a central aggregate index
- make card location lookup closer to O(1) maintenance instead of O(all cards) refresh
- prepare the runtime for higher-throughput and embedded-class constraints

## Target Shape

- aggregate card location entries are updated as cards move between zones
- refresh-style full reconstruction is no longer the steady-state maintenance path
- location lookup remains deterministic and truthful to aggregate ownership

## Invariants

- card ownership and visible zone remain synchronized
- the index never claims a card is in two zones at once
- this slice does not expand supported Magic rules
