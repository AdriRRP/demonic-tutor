# Respond After Combat Damage

## Status

Implemented

## Summary

Once combat damage resolves and `EndOfCombat` opens, the non-active player may
cast and resolve an instant after the active player passes priority.

## Supported Behavior

- resolving combat damage moves the game into `EndOfCombat` and reopens priority for the active player while the game remains active
- after one pass with an empty stack, the non-active player becomes the priority holder
- the non-active player may cast an instant while holding that priority
- the instant resolves after two consecutive passes
- after resolution, priority reopens for the active player while the game remains active

## Out Of Scope

- broader end-of-combat timing beyond the current simplified `EndOfCombat` window
- non-instant responses in `EndOfCombat`
