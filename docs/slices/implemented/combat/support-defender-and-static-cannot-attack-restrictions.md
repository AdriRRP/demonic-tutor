# Support Defender And Static Cannot Attack Restrictions

`SupportDefenderAndStaticCannotAttackRestrictions`

## Goal

Add the first bounded static attack-restriction subset for creatures that cannot attack while on the battlefield.

## Supported Behavior

- explicit defender-like creatures may be cast and exist on the battlefield normally
- those creatures are rejected during attacker declaration
- those creatures may still block if no other supported rule forbids it
- the restriction is static and profile-based while the creature remains on the battlefield

## Out Of Scope

- effects that let a creature attack as though it did not have defender
- conditional attack restrictions based on board state or controller choices
- static `cannot block` profiles
- generic grant/remove-defender engines

## Test Impact

- a supported defender creature cannot be declared as an attacker
- ordinary creatures remain unaffected

## Rules Support Statement

This slice adds only the first bounded static `cannot attack` subset, centered on explicit defender-like creatures.
