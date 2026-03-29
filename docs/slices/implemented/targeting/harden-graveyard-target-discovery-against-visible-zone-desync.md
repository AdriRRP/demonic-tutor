# Slice: Harden Graveyard Target Discovery Against Visible-Zone Desync

Status: implemented

## Summary

Graveyard target discovery now walks the canonical visible graveyard iterator instead of traversing raw zone handles and silently dropping any handle that no longer resolves in the player arena.

## What this slice adds

- graveyard targeting now derives candidate cards from `graveyard_cards()`
- visible graveyard desynchronization now fails explicitly through the same hardened visible-zone access path used elsewhere in the runtime

## Why this matters

- legal target discovery no longer understates visible graveyard state when the underlying runtime is inconsistent
- the aggregate keeps one clearer policy for visible-zone reads: semantic iterators are canonical, storage handles are an internal detail
- replay, legality, and UI-facing behavior stay closer to the same visible truth

## Boundaries kept explicit

- this slice does not broaden graveyard-targeting rules
- this slice does not change supported spell families
- this slice only hardens how the existing visible graveyard state is read
