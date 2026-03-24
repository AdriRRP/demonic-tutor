# Proposal Slice — Recycle Player Card Arena Slots

## Summary

Teach the dense player card arena to reuse released slots instead of growing monotonically after every remove-and-reinsert cycle.

## Motivation

- reduce long-session memory growth
- keep handle-space dense for embedded and cache-sensitive targets
- stop paying permanent historical cost for normal spell movement through stack and zones

## Target Shape

- the arena tracks reusable free slots or another compact reuse strategy
- inserting a new card prefers a recycled slot before extending the backing vector
- public lookup by `CardInstanceId` remains stable through the existing player facade

## Invariants

- live cards still map to exactly one player-local handle
- removed cards must not remain addressable through the arena
- this slice does not expand supported Magic rules
