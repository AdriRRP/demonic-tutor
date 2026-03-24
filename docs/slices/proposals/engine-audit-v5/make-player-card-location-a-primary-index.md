# Proposal Slice — Make Player Card Location A Primary Index

## Summary

Model card location as primary runtime data so ownership and zone queries stop chaining through every supported zone.

## Motivation

- avoid repeated zone-by-zone checks for `card_zone()` and similar queries
- make location changes explicit and cheap to inspect
- prepare future targeting and move semantics on top of a single location source of truth

## Target Shape

- player-owned cards have a primary location record keyed by the internal card identity
- location-aware queries derive from that record instead of probing all zones
- zone movement helpers update both storage and primary location consistently

## Invariants

- a card can still belong to at most one player-owned zone at a time
- outward zone semantics remain unchanged
- this slice does not expand supported Magic rules
