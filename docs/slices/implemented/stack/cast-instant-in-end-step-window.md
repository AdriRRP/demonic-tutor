# Slice - Cast Instant In EndStep Window

## Goal

Allow the active player to cast and resolve an instant spell during the empty priority window opened in `EndStep`.

## Supported Behavior

- when `EndStep` opens an empty priority window, the active player may cast an instant spell from hand
- the instant is put on the stack and the caster keeps priority
- after two consecutive passes, the instant resolves from the stack to the graveyard
- when the game remains active, priority reopens for the active player in `EndStep`

## Explicit Limits

- this slice only formalizes instant casting in the end-step priority window
- end-step triggers, cleanup loops, and broader ending-phase machinery remain out of scope
- cleanup discard still remains a separate concern after the end-step window has been closed

## Domain Changes

- no new public command is introduced; the slice relies on the existing `CastSpell` and `PassPriority` semantics
- the end-step priority window is now covered by executable behavior, not only by generic legality rules

## Rules Support Statement

This slice proves that the minimal stack model remains coherent at the end of the turn. While `EndStep` is open, the active player may cast a zero-cost instant, resolve it through the standard two-pass flow, and remain in `EndStep` with priority reopened before cleanup can finish the turn.

## Tests

- the active player may cast an instant during `EndStep`
- the instant resolves from the stack to the graveyard through the normal two-pass flow
- priority reopens for the active player after the stack resolves while the game remains active
