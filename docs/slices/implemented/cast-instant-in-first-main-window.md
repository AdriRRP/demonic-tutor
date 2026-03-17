# Slice - Cast Instant In First Main Window

## Goal

Allow the active player to cast and resolve an instant spell during the empty priority window opened in `FirstMain`.

## Supported Behavior

- when `FirstMain` opens its empty priority window, the active player may cast an instant spell from hand
- the instant is put on the stack and the caster keeps priority
- after two consecutive passes, the instant resolves from the stack to the graveyard
- when the game remains active, priority reopens for the active player in `FirstMain`

## Explicit Limits

- this slice only formalizes instant casting in the `FirstMain` priority window
- it does not broaden the current simplification that non-active spell responses are limited to instants
- it does not yet cover active-player self-stacking in `FirstMain`; that remains a separate slice

## Domain Changes

- no new public command is introduced; the slice relies on the existing `CastSpell` and `PassPriority` semantics
- the `FirstMain` priority window is now covered by executable behavior, not only by shared legality rules

## Rules Support Statement

This slice proves that the minimal stack model works in the empty `FirstMain` window opened by turn progression. The active player may cast a zero-cost instant, resolve it through the normal two-pass flow, and remain in `FirstMain` with priority reopened.

## Tests

- the active player may cast an instant during `FirstMain`
- the instant resolves from the stack to the graveyard through the normal two-pass flow
- priority reopens for the active player after the stack resolves while the game remains active
