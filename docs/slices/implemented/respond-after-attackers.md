# Respond After Attackers

## Status

Implemented

## Summary

After attackers are declared and the active player passes priority, the
defending player may cast and resolve an instant while holding priority.

## Supported Behavior

- declaring attackers reopens priority for the active player
- after one pass with an empty stack, the defending player becomes the priority holder
- the defending player may cast an instant while holding that priority
- that interaction now lives in the explicit `DeclareBlockers` subphase
- the instant resolves after two consecutive passes
- after resolution, priority reopens for the active player while the game remains active

## Out Of Scope

- broader combat-step timing beyond the current simplified `Combat` phase
- non-instant responses after attackers
