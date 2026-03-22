# Slice - Cast Second Instant In Beginning Of Combat Window

## Goal

Allow the active player to cast a second instant spell at the beginning of `BeginningOfCombat` before passing priority after the first spell is put on the stack.

## Supported Behavior

- entering `BeginningOfCombat` opens an empty priority window for the active player
- after the first instant is cast, the caster keeps priority
- while still holding priority, the active player may cast a second instant
- the second instant is placed on top of the stack
- after two consecutive passes, the second instant resolves first and the original spell remains on the stack

## Explicit Limits

- this slice only proves self-stacking by the active player at the beginning of `BeginningOfCombat`
- it now sits on top of the explicit combat-subphase foundation
- it does not add attack declaration interactions beyond the existing priority window

## Domain Changes

- no new public command is introduced
- the existing stack and priority model is now covered for consecutive instant casts when entering `BeginningOfCombat`

## Rules Support Statement

This slice proves that the minimal stack model works consistently in the beginning-of-combat window: once combat opens priority for the active player, they may cast an instant, keep priority, cast a second instant, and resolve the top object first before either player moves on.

## Tests

- the active player may cast a second instant while retaining priority at the beginning of `BeginningOfCombat`
- both spells remain on the stack under the active player's control until passes begin
- the top spell resolves first and the original spell remains on the stack afterward
