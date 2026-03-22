# Slice - Cast Second Instant In Second Main Window

## Goal

Allow the active player to cast a second instant spell in `SecondMain` before passing priority after the first spell is put on the stack.

## Supported Behavior

- `SecondMain` opens an empty priority window for the active player
- after the first instant is cast, the caster keeps priority
- while still holding priority, the active player may cast a second instant
- the second instant is placed on top of the stack
- after two consecutive passes, the second instant resolves first and the original spell remains on the stack

## Explicit Limits

- this slice only proves self-stacking by the active player in `SecondMain`
- it does not yet broaden main-phase timing beyond the current simplified stack windows
- it does not add sorcery-speed restrictions or priority exceptions beyond the current model

## Domain Changes

- no new public command is introduced
- the existing stack and priority model is now covered for consecutive instant casts in `SecondMain`

## Rules Support Statement

This slice proves that the minimal stack model supports repeated instant casting by the same player in a later main-phase window: after one instant is put on the stack in `SecondMain`, the active player retains priority and may cast a second instant before any player passes.

## Tests

- the active player may cast a second instant while retaining priority in `SecondMain`
- both spells remain on the stack under the active player's control until passes begin
- the top spell resolves first and the original spell remains on the stack afterward
