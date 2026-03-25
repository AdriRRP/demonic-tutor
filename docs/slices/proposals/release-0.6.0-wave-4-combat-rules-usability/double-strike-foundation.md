# Slice Name

Double Strike Foundation

## Goal

Introduce the first supported `Double strike` combat corridor.

## Why This Slice Exists Now

The engine already supports `First strike`. `Double strike` is the natural next keyword because it reuses the same combat-pass shape and increases combat realism significantly.

## Supported Behavior

- a creature with `Double strike` deals damage in the first-strike combat damage pass and again in the regular combat damage pass if still eligible

## Invariants / Legality Rules

- double-strike creatures participate in both supported combat damage passes
- normal creatures still deal damage only in the regular pass unless they have first strike
- if the creature leaves combat before the later pass, it does not deal damage there

## Out of Scope

- replacement effects during damage passes
- interactions with unusual combat rewrites not yet modeled

## Domain Impact

### Entity / Value Object Impact

- extend supported creature keyword set

### Aggregate Impact

- extend the existing split-damage-pass combat model

## Ownership Check

This belongs to the `Game` aggregate because combat-pass participation and damage timing are aggregate-owned rules.

## Documentation Impact

- `docs/domain/current-state.md`
- this slice doc

## Test Impact

- double-strike attacker deals damage in both passes
- first-strike-only creature still deals damage once
- creature removed after first pass does not deal second-pass damage

## Rules Reference

- 702.4

## Rules Support Statement

This slice adds a minimal explicit double-strike corridor on top of the existing first-strike combat-pass model. It does not imply broader keyword completeness.
