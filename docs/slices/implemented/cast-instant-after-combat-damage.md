# Slice - Cast Instant After Combat Damage

## Goal

Allow the active player to cast and resolve an instant spell during the priority window that reopens after combat damage resolves.

## Supported Behavior

- once combat damage resolves and the game remains active, the active player receives priority again
- during that combat priority window, the active player may cast an instant spell from hand
- the instant is put on the stack and the caster keeps priority
- after two consecutive passes, the instant resolves from the stack to the graveyard
- when the game remains active, priority reopens for the active player in `Combat`

## Explicit Limits

- this slice only formalizes instant casting after combat damage resolves
- the runtime still uses one `Combat` phase rather than full combat-step structure
- post-damage triggered abilities and broader combat-end timing remain out of scope

## Domain Changes

- no new public command is introduced; the slice relies on the existing `ResolveCombatDamage`, `CastSpell`, and `PassPriority` semantics
- the post-damage combat window is now covered by executable behavior, not only by generic legality rules

## Rules Support Statement

This slice proves that the minimal stack model remains coherent after combat damage. Once damage resolves and the game stays active, the active player may cast a zero-cost instant, resolve it through the standard two-pass flow, and remain in `Combat` with priority reopened before leaving combat for `SecondMain`.

## Tests

- the active player may cast an instant after combat damage resolves
- the instant resolves from the stack to the graveyard through the normal two-pass flow
- priority reopens for the active player after the stack resolves while the game remains active
