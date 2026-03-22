# Slice - Cast Second Instant In Draw Window

## Goal

Allow the active player to cast a second instant spell in `Draw` after the automatic turn draw and before passing priority after the first spell is put on the stack.

## Supported Behavior

- entering `Draw` performs the automatic turn draw and then opens priority for the active player
- after the first instant is cast, the caster keeps priority
- while still holding priority, the active player may cast a second instant
- the second instant is placed on top of the stack
- after two consecutive passes, the second instant resolves first and the original spell remains on the stack

## Explicit Limits

- this slice only proves self-stacking by the active player in `Draw`
- it does not add new draw-step triggered behavior
- it does not yet broaden the supported spell types for responses

## Domain Changes

- no new public command is introduced
- the existing stack and priority model is now covered for consecutive instant casts in the draw-step priority window

## Rules Support Statement

This slice proves that the minimal stack model remains coherent in `Draw` after the automatic turn draw has already happened: the active player may cast an instant, keep priority, cast a second instant, and resolve the top object first through the normal two-pass flow.

## Tests

- the active player may cast a second instant while retaining priority in `Draw`
- both spells remain on the stack under the active player's control until passes begin
- the top spell resolves first and the original spell remains on the stack afterward
