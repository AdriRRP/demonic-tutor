# Slice Name

`RemoveRepeatedLinearLookupsFromCombatDamageStep`

## Goal

Replace repeated linear lookups inside combat damage resolution with temporary keyed structures that preserve the same combat semantics.

## Why This Slice Exists Now

Combat is now rich enough that the damage step has become a real hot path. The current corridor repeatedly scans already materialized participants and accumulated assignments.

## Supported Behavior

- preserve the current supported combat behavior exactly
- avoid re-scanning blocker lists to reconstruct blocker order per attacker
- avoid re-scanning accumulated damage assignments when multiple sources hit the same creature

## Invariants / Legality Rules

- blocked attackers remain blocked across the later supported pass
- trample, deathtouch, lifelink, first strike, and double strike preserve current semantics

## Out of Scope

- adding new combat keywords
- changing combat damage rules
- introducing a generic combat engine

## Domain Impact

### Aggregate Impact
- none at the boundary

### Entity / Value Object Impact
- temporary keyed resolution structures inside combat damage rules

## Ownership Check

This belongs to gameplay domain combat rules because it changes the internal resolution strategy of an existing authoritative rules corridor.

## Documentation Impact

- this slice document

## Test Impact

- existing combat damage regression suite remains green
- multi-block, trample, deathtouch, and double-strike regressions remain green

## Rules Reference

- 510.1 — combat damage assignment, simplified to the supported subset
- 702.2, 702.4, 702.13, 702.15, 702.19 — deathtouch, double strike, trample, first strike, lifelink in the supported subset

## Rules Support Statement

This slice does not add combat support. It optimizes the current supported combat-damage corridor.

## Open Questions

- none
