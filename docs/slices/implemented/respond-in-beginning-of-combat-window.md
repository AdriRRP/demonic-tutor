# Respond In Beginning Of Combat Window

## Status

Implemented

## Summary

After the active player passes the empty priority window that opens when the
game enters `Combat`, the non-active player may cast and resolve an instant.

## Supported Behavior

- entering `Combat` opens priority for the active player
- after one pass with an empty stack, the non-active player becomes the priority holder
- the non-active player may cast an instant while holding that priority
- the instant resolves after two consecutive passes
- after resolution, priority reopens for the active player while the game remains active

## Out Of Scope

- broader combat-step timing beyond the current simplified `Combat` phase
- non-instant responses and triggered beginning-of-combat abilities
