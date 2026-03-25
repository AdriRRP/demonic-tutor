# Slice Name

Enforce One Loyalty Activation Per Turn

## Goal

Make the supported planeswalker subset obey the one-loyalty-ability-per-planeswalker-per-turn limit.

## Why This Slice Exists Now

The first loyalty-ability foundation made planeswalkers usable, but it still allowed repeated activation in the same turn if loyalty remained available. That diverged from both Magic rules and the support statement already claimed by the project.

## Supported Behavior

- allow a supported planeswalker to activate at most one loyalty ability each turn
- reset that restriction when its controller starts a new turn
- keep loyalty payment as part of activation before the ability goes onto the stack

## Invariants / Legality Rules

- a supported planeswalker cannot activate a second loyalty ability in the same turn
- the once-per-turn restriction is tracked on the permanent runtime, not on the stack object
- changing phases within the same turn does not reset the restriction

## Out of Scope

- full planeswalker rules coverage beyond the currently supported subset
- loyalty abilities granted dynamically by other effects
- multiplayer-specific control-change edge cases beyond the current aggregate scope

## Domain Impact

### Aggregate Impact

- extend battlefield runtime state so turn flow can reset loyalty usage at the correct boundary

### Entity / Value Object Impact

- track per-turn loyalty activation usage on supported permanent runtime

## Ownership Check

This belongs to the `Game` aggregate because loyalty timing, activation legality, and turn resets are gameplay-domain responsibilities.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/rules/rules-map.md`
- `docs/slices/implemented/abilities/planeswalker-loyalty-ability-foundation.md`
- this slice doc

## Test Impact

- reject a second loyalty activation from the same planeswalker in the same turn
- allow a new loyalty activation after the controller starts a new turn

## Rules Reference

- 306
- 606.3

## Rules Support Statement

This slice closes the once-per-turn rule only for the currently supported explicit loyalty-ability subset.
