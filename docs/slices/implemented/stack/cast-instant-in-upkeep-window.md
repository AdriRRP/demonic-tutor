# Slice - Cast Instant In Upkeep Window

## Goal

Allow the active player to cast and resolve an instant spell during the empty priority window opened in `Upkeep`.

## Supported Behavior

- when `Upkeep` opens an empty priority window, the active player may cast an instant spell from hand
- the instant is put on the stack and the caster keeps priority
- after two consecutive passes, the instant resolves from the stack to the graveyard
- when the game remains active, priority reopens for the active player in `Upkeep`

## Explicit Limits

- this slice only formalizes instant casting in the upkeep window
- non-instant responses remain unsupported outside the existing main-phase exception
- upkeep-triggered abilities and other beginning-phase mechanics remain out of scope

## Domain Changes

- no new public command is introduced; the slice relies on the existing `CastSpell` and `PassPriority` semantics
- the upkeep priority window is now covered by executable behavior, not only by generic legality rules

## Rules Support Statement

This slice proves that the minimal stack model works in a real non-main-phase timing window. Once `Upkeep` opens priority for the active player, a zero-cost instant may be cast, put on the stack, resolved through two passes, and leave the game still in `Upkeep` with priority reopened for the active player.

## Tests

- the active player may cast an instant during `Upkeep`
- the instant resolves from the stack to the graveyard through the normal two-pass flow
- priority reopens for the active player after the stack resolves while the game remains active
