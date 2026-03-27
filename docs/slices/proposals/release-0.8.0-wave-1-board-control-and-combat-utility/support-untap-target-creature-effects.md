# Slice Name

`SupportUntapTargetCreatureEffects`

## Goal

Add the first explicit spell corridor that untaps a target creature on the battlefield.

## Why This Slice Exists Now

`0.8.0 wave 1` is about common limited combat utility. After `tap target creature`, the next highest-return mirror effect is `untap target creature`, because it unlocks combat tricks, surprise blockers, and second-use activation play patterns without requiring broader control or layers support.

## Supported Behavior

- cast a supported spell that targets exactly one creature on the battlefield
- if the target is still legal on resolution, that creature becomes untapped
- if the target is gone or otherwise illegal on resolution, the spell has no effect
- untapping an already untapped creature is a no-op

## Invariants / Legality Rules

- target legality reuses the current shared cast-time and resolution-time targeting corridor
- the slice does not widen priority windows
- the slice does not add new combat-state cleanup semantics beyond changing the tapped flag

## Out Of Scope

- generic activated abilities that untap another creature
- untapping multiple creatures
- effects that untap lands, artifacts, or other noncreature permanents
- "untap during the next untap step" or replacement-like timing text

## Domain Impact

### Aggregate Impact

- one new explicit spell-resolution profile for `untap target creature`

### Commands

- no new commands

### Events

- no new dedicated event beyond the existing spell-resolution corridor

## Test Impact

- successful resolution untaps a tapped creature
- target loss before resolution leaves the spell without effect
- an untapped target remains untapped

## Rules Support Statement

This slice adds only the first bounded spell subset for untapping a target creature on the battlefield.
