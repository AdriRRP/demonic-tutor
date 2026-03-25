# Slice Name

Deathtouch Foundation

## Goal

Introduce the first supported `Deathtouch` combat corridor.

## Why This Slice Exists Now

`Deathtouch` has a large gameplay impact relative to its modeling footprint, especially once combat assignment and multi-blocker scenarios become richer.

## Supported Behavior

- a creature with `Deathtouch` marks combat damage as lethal for SBA purposes with any nonzero damage
- deathtouch interacts with combat damage assigned to creatures in the current subset

## Invariants / Legality Rules

- only nonzero damage from a deathtouch source counts as lethal-by-deathtouch
- deathtouch changes lethal assessment, not the amount of marked damage itself

## Out of Scope

- deathtouch outside combat if noncombat damage from creatures is not yet modeled
- interaction with damage prevention
- interaction with indestructible

## Domain Impact

### Entity / Value Object Impact

- extend supported creature keyword set

### Aggregate Impact

- extend SBA lethal review semantics

## Ownership Check

This belongs to the `Game` aggregate because SBA lethal review and combat keywords are aggregate-owned gameplay semantics.

## Documentation Impact

- `docs/domain/current-state.md`
- this slice doc

## Test Impact

- one damage from deathtouch kills a creature
- zero damage does not
- ordinary non-deathtouch lethal rules remain unchanged

## Rules Reference

- 702.2
- 704.5g

## Rules Support Statement

This slice adds a minimal combat-focused deathtouch corridor only. It does not imply full support for all deathtouch interactions across the game.

