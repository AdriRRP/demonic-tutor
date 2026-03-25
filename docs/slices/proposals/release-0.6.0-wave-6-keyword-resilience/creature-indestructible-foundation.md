# Slice Name

Creature Indestructible Foundation

## Goal

Support `Indestructible` on creatures in the current destruction and combat-damage subset.

## Why This Slice Exists Now

The engine already models lethal damage SBA and explicit `destroy target creature`. `Indestructible` is a high-return defensive keyword because it changes both combat math and removal outcomes without needing a full continuous-effects system.

## Supported Behavior

- a creature with `Indestructible` is not destroyed by lethal damage in the current SBA subset
- a creature with `Indestructible` is not destroyed by the current explicit destroy-creature effect
- damage can still be marked on an indestructible creature and is removed normally at end of turn

## Invariants / Legality Rules

- `Indestructible` does not stop exile, bounce, sacrifice, or zero-toughness SBA
- this slice only covers creatures in the current keyword subset
- `destroy` effects that do not target creatures remain outside this slice

## Out of Scope

- indestructible on noncreature permanents
- regeneration
- layered interactions with temporary type changes

## Rules Reference

- 702.12
- 704.5g

## Rules Support Statement

This slice adds `Indestructible` only for creatures in the current SBA and destroy-target-creature corridors.
