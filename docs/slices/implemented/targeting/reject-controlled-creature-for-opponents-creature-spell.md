# Reject Controlled Creature For Opponents Creature Spell

## Status

Implemented

## Scope

- a spell that requires `creature an opponent controls` is rejected when the acting player chooses their own creature
- the supported rejection is exercised outside combat in `FirstMain`
- rejection happens before the spell is put on the stack

## Out Of Scope

- multiplayer opponent sets
- target changes after casting
- full resolution coverage for this rule family
