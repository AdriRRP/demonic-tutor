# Slice - Cast Instant In Draw Window

## Goal

Allow the active player to cast and resolve an instant spell during the empty priority window opened in `Draw`.

## Supported Behavior

- when `Draw` opens its post-draw priority window, the active player may cast an instant spell from hand
- the instant is put on the stack and the caster keeps priority
- after two consecutive passes, the instant resolves from the stack to the graveyard
- when the game remains active, priority reopens for the active player in `Draw`

## Explicit Limits

- this slice only formalizes instant casting in the draw-step priority window
- the automatic turn draw still happens before the window opens
- draw-triggered abilities and other draw-step mechanics remain out of scope

## Domain Changes

- no new public command is introduced; the slice relies on the existing `CastSpell` and `PassPriority` semantics
- the draw-step priority window is now covered by executable behavior, not only by generic legality rules

## Rules Support Statement

This slice proves that the minimal stack model also works in the draw-step window that now follows the automatic turn draw. After the card is drawn, the active player may cast a zero-cost instant, resolve it through the standard two-pass flow, and remain in `Draw` with priority reopened.

## Tests

- the active player may cast an instant during `Draw`
- the instant resolves from the stack to the graveyard through the normal two-pass flow
- priority reopens for the active player after the stack resolves while the game remains active
