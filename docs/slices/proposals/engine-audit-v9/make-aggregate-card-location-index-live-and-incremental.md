# Proposal Slice — Make Aggregate Card Location Index Live And Incremental

## Summary

Stop rebuilding `AggregateCardLocationIndex` from scratch after public operations and maintain it incrementally as cards move between zones.

## Motivation

- remove full-card rescans from a central aggregate structure
- make location maintenance closer to constant-time updates
- prepare the runtime for higher-throughput and embedded-class constraints

## Target Shape

- aggregate card location entries are updated alongside semantic zone transitions
- full refresh is no longer the steady-state maintenance path
- location lookup remains deterministic and truthful to aggregate ownership

## Invariants

- ownership and visible zone remain synchronized
- the index never claims a card is in two zones at once
- this slice does not expand supported Magic rules
