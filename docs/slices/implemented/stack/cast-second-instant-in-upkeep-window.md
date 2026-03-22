# Slice - Cast Second Instant In Upkeep Window

## Goal

Allow the active player to cast a second instant spell in `Upkeep` before passing priority after the first spell is put on the stack.

## Supported Behavior

- `Upkeep` opens an empty priority window for the active player
- after the first instant is cast, the caster keeps priority
- while still holding priority, the active player may cast a second instant
- the second instant is placed on top of the stack
- after two consecutive passes, the second instant resolves first and the original spell remains on the stack

## Explicit Limits

- this slice only proves self-stacking by the active player in `Upkeep`
- it does not yet add new object types to the stack
- it does not yet model upkeep triggers or broader APNAP timing

## Domain Changes

- no new public command is introduced
- the existing stack and priority model is now covered for consecutive instant casts by the same player in `Upkeep`

## Rules Support Statement

This slice extends the minimal stack model by proving that priority retention after casting is meaningful in a real turn window: the active player may cast one instant, retain priority, cast another instant, and resolve the top object first through the normal two-pass flow.

## Tests

- the active player may cast a second instant while retaining priority in `Upkeep`
- both spells remain on the stack under the active player's control until passes begin
- the top spell resolves first and the original spell remains on the stack afterward
