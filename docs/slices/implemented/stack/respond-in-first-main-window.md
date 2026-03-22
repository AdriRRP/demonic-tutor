# Respond In First Main Window

## Status

Implemented

## Summary

After the active player passes an empty `FirstMain` priority window, the
non-active player may cast and resolve an instant while holding priority.

## Supported Behavior

- `FirstMain` opens priority for the active player
- after one pass with an empty stack, the non-active player becomes the priority holder
- the non-active player may cast an instant while holding that priority
- the instant resolves after two consecutive passes
- after resolution, priority reopens for the active player while the game remains active

## Out Of Scope

- non-instant responses in main phase with an empty stack
- activated abilities and richer stack interaction
