# Reject Non-Attacking Creature For Attack-Only Spell

## Summary

Reject a supported `AttackingCreature` spell target when the chosen creature is on the battlefield but is not currently attacking.

## Scope

- reuse the explicit `AttackingCreature` legal-target rule
- make the cast-time rejection observable for ordinary battlefield creatures outside combat

## Out Of Scope

- richer combat-relative target predicates
