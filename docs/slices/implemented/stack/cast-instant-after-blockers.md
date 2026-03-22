# Slice - Cast Instant After Blockers

## Goal

Allow the active player to cast and resolve an instant spell during the priority window that opens after blockers are declared.

## Supported Behavior

- once blockers are declared, the active player receives priority
- during that combat priority window, the active player may cast an instant spell from hand
- the instant is put on the stack and the caster keeps priority
- after two consecutive passes, the instant resolves from the stack to the graveyard
- when the game remains active, priority reopens for the active player in `Combat`

## Explicit Limits

- this slice only formalizes instant casting after blockers are declared
- combat damage is still resolved only after the current combat priority window has been closed
- the currently supported targeted instant subset is allowed, but richer combat tricks, abilities, and broader combat timing remain out of scope

## Domain Changes

- no new public command is introduced; the slice relies on the existing `DeclareBlockers`, `CastSpell`, and `PassPriority` semantics
- the post-blockers combat window is now covered by executable behavior, not only by generic legality rules

## Rules Support Statement

This slice proves that the minimal stack model also works in the second post-declaration combat window. After blockers are declared, the active player may cast a zero-cost instant, resolve it through the standard two-pass flow, and remain in `CombatDamage` timing with priority reopened before combat damage is resolved.

## Tests

- the active player may cast an instant after blockers are declared
- the instant resolves from the stack to the graveyard through the normal two-pass flow
- priority reopens for the active player after the stack resolves while the game remains active
