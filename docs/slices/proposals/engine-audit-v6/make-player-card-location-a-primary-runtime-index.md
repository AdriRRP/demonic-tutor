# Proposal Slice — Make Player Card Location A Primary Runtime Index

## Summary

Promote card location to primary runtime data so ownership and zone queries read one maintained index instead of probing every zone.

## Motivation

- remove repeated zone-by-zone checks from player queries
- make location-aware rules cheaper and more explicit
- prepare future handle-first movement semantics on a single source of truth

## Target Shape

- each live player-owned card has a primary runtime location entry
- semantic transitions update both the zone container and the location index
- `card_zone()`, `owns_card()`, and similar queries read that index directly

## Invariants

- a card still belongs to at most one player-owned zone at a time
- visible zone behavior remains unchanged
- this slice does not expand supported Magic rules
