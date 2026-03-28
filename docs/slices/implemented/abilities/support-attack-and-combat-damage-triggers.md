# Slice Implemented - Support Attack And Combat Damage Triggers

## Outcome

The engine now supports one explicit combat-trigger corridor for `when this attacks` and one explicit corridor for `when this deals combat damage to a player`.

## What Landed

- one bounded `Attacks` triggered-ability profile
- one bounded `DealsCombatDamageToPlayer` triggered-ability profile
- enqueue on attacker declaration through the shared combat-to-stack trigger path
- enqueue after combat damage resolves only when damage actually hit a player
- public command envelopes and application events now surface those triggered abilities when they are put on the stack
- focused combat and public-envelope regressions proving both trigger families resolve through the existing stack model

## Notes

- this slice intentionally stops at combat triggers that reuse the existing single-trigger-card profile and life-gain effect
- it does not add generic attack-trigger payloads, combat-damage-to-creature triggers, saboteur choice trees, or simultaneous-trigger ordering beyond the currently supported stack discipline
