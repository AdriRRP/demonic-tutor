# Slice Name

Lifelink Combat Damage Foundation

## Goal

Support `Lifelink` on creature combat damage in the current combat corridor.

## Why This Slice Exists Now

Combat already resolves damage through explicit damage events and shared life-change semantics. `Lifelink` adds a high-visibility reward pattern to creature combat without opening a generic replacement-effects engine.

## Supported Behavior

- combat damage dealt by a creature with `Lifelink` causes its controller to gain that much life
- this applies to damage dealt to players and creatures
- `Double strike` and `First strike` reuse the existing split passes and may therefore gain life in more than one pass

## Invariants / Legality Rules

- life gain is based on damage actually dealt in the supported combat resolution model
- `Lifelink` does not change damage assignment
- life gain happens in the same combat-resolution corridor before SBA review completes

## Out of Scope

- non-combat lifelink sources
- replacement or prevention effects on damage
- static effect systems that grant or remove lifelink dynamically

## Rules Reference

- 702.15

## Rules Support Statement

This slice adds `Lifelink` only for the current creature-combat subset.
