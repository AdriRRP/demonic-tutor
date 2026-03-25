# Slice Name

Creature Hexproof Targeting Foundation

## Goal

Support `Hexproof` on creatures in the current targeted-spell and targeted-ability corridor.

## Why This Slice Exists Now

The engine already has a broad explicit targeting matrix. `Hexproof` is one of the highest-value defensive keywords because it immediately makes many existing interactive cards and abilities behave more like real Magic.

## Supported Behavior

- a creature with `Hexproof` cannot be targeted by spells or abilities controlled by its opponent
- its controller may still target it with their own legal spells or abilities
- illegal hexproof targets are rejected both at cast/activation time and on resolution revalidation

## Invariants / Legality Rules

- this slice only covers creatures in the current keyword subset
- `Hexproof` affects targeting legality, not combat blocking or damage
- untargeted effects are unchanged

## Out of Scope

- shroud
- hexproof on noncreature permanents
- multiplayer-specific “opponent” semantics beyond the current model

## Rules Reference

- 702.11

## Rules Support Statement

This slice adds `Hexproof` only for creature targets in the current model.
