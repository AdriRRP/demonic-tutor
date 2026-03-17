# Slice - Cast Instant After Attackers

## Goal

Allow the active player to cast and resolve an instant spell during the priority window that opens after attackers are declared.

## Supported Behavior

- once attackers are declared, the active player receives priority
- during that combat priority window, the active player may cast an instant spell from hand
- the instant is put on the stack and the caster keeps priority
- after two consecutive passes, the instant resolves from the stack to the graveyard
- when the game remains active, priority reopens for the active player in `Combat`

## Explicit Limits

- this slice only formalizes instant casting after attackers are declared
- blockers are still declared only after the current combat priority window has been closed
- targeting, combat tricks with rules text, and broader combat-step structure remain out of scope

## Domain Changes

- no new public command is introduced; the slice relies on the existing `DeclareAttackers`, `CastSpell`, and `PassPriority` semantics
- the post-attackers combat window is now covered by executable behavior, not only by generic legality rules

## Rules Support Statement

This slice proves that the minimal stack model works in the first post-declaration combat window. After attackers are declared, the active player may cast a zero-cost instant, resolve it through the standard two-pass flow, and remain in `Combat` with priority reopened before blockers are declared.

## Tests

- the active player may cast an instant after declaring attackers
- the instant resolves from the stack to the graveyard through the normal two-pass flow
- priority reopens for the active player after the stack resolves while the game remains active
