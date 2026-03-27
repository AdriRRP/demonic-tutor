# Support Untap Target Creature Effects

`SupportUntapTargetCreatureEffects`

## Goal

Add the first explicit spell corridor that untaps a target creature on the battlefield.

## Supported Behavior

- a supported spell may target exactly one creature on the battlefield
- if the target is still legal on resolution, that creature becomes untapped
- if the target is already untapped, resolution is a no-op
- if the target is gone or illegal on resolution, the spell has no effect

## Out Of Scope

- untapping multiple permanents
- untapping noncreature permanents
- delayed untap effects
- broader activated untap engines

## Test Impact

- resolution untaps a tapped creature
- untapping an untapped target is harmless
- target loss on resolution leaves no effect

## Rules Support Statement

This slice adds only a bounded `untap target creature` spell subset for one creature on the battlefield.
