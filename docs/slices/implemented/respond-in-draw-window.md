# Respond In Draw Window

## Status

Implemented

## Summary

After the active player passes an empty `Draw` priority window, the non-active
player may cast and resolve an instant while holding priority.

## Supported Behavior

- the automatic turn draw happens before the `Draw` priority window opens
- after one pass with an empty stack, the non-active player becomes the priority holder
- the non-active player may cast an instant while holding that priority
- the instant resolves after two consecutive passes
- after resolution, priority reopens for the active player while the game remains active

## Out Of Scope

- non-instant responses in draw
- triggered abilities and replacement effects around drawing cards
