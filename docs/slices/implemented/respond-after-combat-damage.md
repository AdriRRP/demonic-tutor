# Respond After Combat Damage

## Status

Implemented

## Summary

After combat damage resolves and the active player passes priority, the
non-active player may cast and resolve an instant while holding priority.

## Supported Behavior

- resolving combat damage reopens priority for the active player while the game remains active
- after one pass with an empty stack, the non-active player becomes the priority holder
- the non-active player may cast an instant while holding that priority
- the instant resolves after two consecutive passes
- after resolution, priority reopens for the active player while the game remains active

## Out Of Scope

- broader post-damage combat timing beyond the current simplified `Combat` phase
- non-instant responses after combat damage
