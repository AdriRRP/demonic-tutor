# Respond In End Step Window

## Status

Implemented

## Summary

After the active player passes an empty `EndStep` priority window, the
non-active player may cast and resolve an instant while holding priority.

## Supported Behavior

- `EndStep` opens priority before cleanup can finish the turn
- after one pass with an empty stack, the non-active player becomes the priority holder
- the non-active player may cast an instant while holding that priority
- the instant resolves after two consecutive passes
- after resolution, priority reopens for the active player while the game remains active

## Out Of Scope

- non-instant responses in end step
- triggered end-step abilities and cleanup loops
