# Proposal Slice — Introduce Aggregate-Level Card Location Index

## Summary

Promote aggregate-wide card ownership and location lookup to a maintained index so helpers and effect corridors stop scanning players zone by zone.

## Motivation

- remove repeated aggregate scans for battlefield and graveyard ownership lookup
- make targeting and resolution ownership cheaper and more explicit
- prepare broader handle-first identity across the full aggregate

## Target Shape

- the aggregate maintains a primary lookup from runtime card identity to owning player and current player-owned zone
- battlefield and graveyard location helpers read that index directly
- semantic transitions keep the index synchronized

## Invariants

- ownership stays explicit and deterministic
- location-aware rules remain truthful to current supported behavior
- this slice does not expand supported Magic rules
