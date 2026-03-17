# Slice — Respond With Second Instant In Beginning Of Combat Window

## Goal

Allow the non-active player to cast a second instant spell at the beginning of `Combat` before passing priority after already responding once in that window.

## Supported Behavior

- entering `Combat` opens an empty priority window for the active player
- after Alice passes, Bob receives priority at the beginning of `Combat`
- Bob may cast an instant response while holding that priority
- after the first response is put on the stack, Bob keeps priority
- Bob may cast a second instant response before any player passes
- after two consecutive passes, the second response resolves first and Bob's original response remains on the stack
- when the game remains active, priority reopens for Alice in `Combat`

## Explicit Limits

- only instant responses are currently supported
- this slice only formalizes responding-player self-stacking at the beginning of `Combat`
- richer combat-step timing remains out of scope

## Domain Changes

- no new public command is introduced
- the existing stack and priority model is now covered for consecutive non-active instant responses at the beginning of `Combat`

## Rules Support Statement

This slice extends the minimal beginning-of-combat timing model. Once the active player passes the empty combat-entry window, the non-active player may respond with an instant, keep priority, and cast a second instant before passes begin, with LIFO resolution preserved.

## Tests

- the non-active player may cast a second instant while retaining priority at the beginning of `Combat`
- both responses remain on the stack under Bob's control until passes begin
- the top response resolves first and Bob's original response remains on the stack afterward
