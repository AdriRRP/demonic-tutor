# Respond After Blockers

## Status

Implemented

## Summary

After blockers are declared and the active player passes priority, the
defending player may cast and resolve an instant while holding priority.

## Supported Behavior

- declaring blockers reopens priority for the active player
- after one pass with an empty stack, the defending player becomes the priority holder
- the defending player may cast an instant while holding that priority
- that interaction now lives in the explicit `CombatDamage` subphase
- the instant resolves after two consecutive passes
- after resolution, priority reopens for the active player while the game remains active

## Out Of Scope

- broader combat-step timing beyond the current simplified `Combat` phase
- non-instant responses after blockers
