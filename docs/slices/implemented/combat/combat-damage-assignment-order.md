# Slice Name

Combat Damage Assignment Order

## Goal

Support explicit damage-assignment order when one attacker is blocked by multiple creatures.

## Why This Slice Exists Now

Once multi-blocker combat exists, the next correctness step is the order in which attackers assign damage. Without this, the combat model still diverges too far from real gameplay.

## Supported Behavior

- require an explicit blocker order for a multi-blocked attacker
- assign attacker combat damage in that order
- preserve current first-strike and trample support on top of that order where compatible

## Invariants / Legality Rules

- an attacker must assign lethal-or-more forward through its ordered blockers before assigning later damage in the current subset
- the declared order must remain stable for the combat damage step

## Out of Scope

- full defending-player damage assignment choices for all special cases
- damage prevention and replacement effects
- unusual combat redirection cases

## Domain Impact

### Aggregate Impact

- extend combat state with attacker damage-assignment order

## Ownership Check

This belongs to the `Game` aggregate because combat damage assignment is a core aggregate rule.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/rules/rules-map.md`
- this slice doc

## Test Impact

- attacker assigns damage across two blockers in declared order
- first strike respects the same order
- trample uses the same order before excess reaches player

## Rules Reference

- 510.1c
- 702.19
- 702.4

## Rules Support Statement

This slice adds the minimal ordering rule needed for multi-blocker combat. It does not yet imply full combat-damage replacement or prevention support.

