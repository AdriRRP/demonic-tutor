# Slice - Cast Second Instant In EndStep Window

## Goal

Allow the active player to cast a second instant spell in `EndStep` before passing priority after the first spell is put on the stack.

## Supported Behavior

- `EndStep` opens an empty priority window for the active player before cleanup can finish the turn
- after the first instant is cast, the caster keeps priority
- while still holding priority, the active player may cast a second instant
- the second instant is placed on top of the stack
- after two consecutive passes, the second instant resolves first and the original spell remains on the stack

## Explicit Limits

- this slice only proves self-stacking by the active player in `EndStep`
- cleanup discard remains a separate concern after the end-step priority window closes
- end-step triggers and cleanup loops remain out of scope

## Domain Changes

- no new public command is introduced
- the existing stack and priority model is now covered for consecutive instant casts in `EndStep`

## Rules Support Statement

This slice proves that the minimal stack model stays coherent at the end of the turn: while `EndStep` priority is still open, the active player may cast one instant, keep priority, cast a second instant, and resolve the top object first before cleanup can continue.

## Tests

- the active player may cast a second instant while retaining priority in `EndStep`
- both spells remain on the stack under the active player's control until passes begin
- the top spell resolves first and the original spell remains on the stack afterward
