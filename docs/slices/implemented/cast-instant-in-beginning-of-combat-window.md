# Slice - Cast Instant In Beginning Of Combat Window

## Goal

Allow the active player to cast and resolve an instant spell during the empty priority window opened when the game enters `Combat`.

## Supported Behavior

- when `Combat` opens its entry priority window, the active player may cast an instant spell from hand
- the instant is put on the stack and the caster keeps priority
- after two consecutive passes, the instant resolves from the stack to the graveyard
- when the game remains active, priority reopens for the active player in `Combat`

## Explicit Limits

- this slice only formalizes instant casting in the beginning-of-combat window
- the runtime still does not split combat into full Magic substeps
- combat tricks that depend on targeting, abilities, or damage-prevention semantics remain out of scope

## Domain Changes

- no new public command is introduced; the slice relies on the existing `CastSpell` and `PassPriority` semantics
- the beginning-of-combat priority window is now covered by executable behavior, not only by generic legality rules

## Rules Support Statement

This slice proves that the minimal stack model works in the first combat timing window. Once the game enters `Combat`, the active player may cast a zero-cost instant, resolve it through the standard two-pass flow, and remain in `Combat` with priority reopened for the active player before attackers are declared.

## Tests

- the active player may cast an instant at the beginning of `Combat`
- the instant resolves from the stack to the graveyard through the normal two-pass flow
- priority reopens for the active player after the stack resolves while the game remains active
