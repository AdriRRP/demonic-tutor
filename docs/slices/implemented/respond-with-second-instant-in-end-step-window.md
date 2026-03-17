# Slice — Respond With Second Instant In End Step Window

## Goal

Allow the non-active player to cast a second instant spell in `EndStep` before passing priority after already responding once in that window.

## Supported Behavior

- `EndStep` opens an empty priority window for the active player before cleanup can finish the turn
- after Alice passes, Bob receives priority in `EndStep`
- Bob may cast an instant response while holding that priority
- after the first response is put on the stack, Bob keeps priority
- Bob may cast a second instant response before any player passes
- after two consecutive passes, the second response resolves first and Bob's original response remains on the stack
- when the game remains active, priority reopens for Alice in `EndStep`

## Explicit Limits

- only instant responses are currently supported
- this slice only formalizes responding-player self-stacking in `EndStep`
- cleanup discard and richer end-step timing remain out of scope

## Domain Changes

- no new public command is introduced
- the existing stack and priority model is now covered for consecutive non-active instant responses in `EndStep`

## Rules Support Statement

This slice extends the minimal end-step timing model. Once the active player passes the empty end-step window, the non-active player may respond with an instant, keep priority, and cast a second instant before passes begin, with LIFO resolution preserved.

## Tests

- the non-active player may cast a second instant while retaining priority in `EndStep`
- both responses remain on the stack under Bob's control until passes begin
- the top response resolves first and Bob's original response remains on the stack afterward
