# Slice Name

`SupportCannotBlockThisTurnEffects`

## Goal

Add the first explicit temporary combat restriction that prevents a target creature from blocking for the rest of the turn.

## Why This Slice Exists Now

This is one of the most common combat-tempo effects in limited. It adds meaningful attack-step planning without requiring continuous layers, control-changing effects, or generic status engines.

## Supported Behavior

- cast a supported spell that targets exactly one creature on the battlefield
- if the target is still legal on resolution, that creature cannot be declared as a blocker for the rest of the current turn
- the restriction expires during the normal end-of-turn cleanup for temporary effects
- if the target is gone on resolution, the spell has no effect

## Invariants / Legality Rules

- the effect changes blocking legality only; it does not tap, damage, or remove the creature
- the restriction must be visible to the canonical blocking-legality corridor
- the effect must not persist into the next turn

## Out Of Scope

- "cannot block this turn" applied to multiple creatures
- "cannot attack or block this turn" as a combined generic corridor
- static or attachment-based `cannot block` effects that persist while a permanent remains in play
- effects that remove a creature from combat after blockers are already declared

## Domain Impact

### Aggregate Impact

- one bounded temporary combat-restriction flag in creature runtime
- shared end-of-turn cleanup for that temporary flag

### Commands

- no new commands

### Events

- no new dedicated event beyond the existing spell-resolution corridor

## Test Impact

- a targeted creature cannot be declared as a blocker later in the same turn
- the restriction disappears on the next turn
- target loss on resolution leaves no lingering restriction

## Rules Support Statement

This slice adds only a bounded temporary `cannot block this turn` spell subset for one target creature.
