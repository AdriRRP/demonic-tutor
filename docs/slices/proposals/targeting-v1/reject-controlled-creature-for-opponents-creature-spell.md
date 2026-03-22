# Slice Proposal — Reject Controlled Creature For Opponents Creature Spell

## Goal

Reject a non-combat targeted spell when the acting player targets their own creature but the rule requires an opponent-controlled creature.

## Why This Slice Exists Now

The positive `opponents creature` case is incomplete without the symmetric negative case. This slice keeps the target-legality corridor honest.

## Supported Behavior

- a spell requiring `creature an opponent controls` is rejected when the acting player chooses a self-controlled creature

## Invariants / Legality Rules

- illegal contextual targets are rejected before the spell is put on the stack
- rejection leaves hand, stack, and mana untouched

## Out of Scope

- multiplayer opponent sets
- target changes after casting

## Domain Impact

- no new abstractions beyond the `opponents creature` target rule
- strengthen negative-path coverage

## Ownership Check

This is aggregate-owned target legality.

## Documentation Impact

- current-state only if it lists the supported rule family
- the implemented slice doc for this capability

## Test Impact

- unit and executable BDD rejection coverage

## Rules Reference

- 114
- 601.2c

## Rules Support Statement

This slice adds the required rejection path for the non-combat `opponents creature` rule. It does not broaden the supported targeting subset beyond that explicit restriction.
