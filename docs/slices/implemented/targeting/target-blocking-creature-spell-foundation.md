# Target Blocking Creature Spell Foundation

## Summary

Introduce an explicit legal-target rule for spells that may target exactly one blocking creature.

## Scope

- add a `BlockingCreature` rule to the supported creature-target subset
- keep the rule inside the same explicit target-legality corridor already used for player, controlled-creature, and attacking-creature spells
- prove that the spell rejects an obviously illegal target kind

## Out Of Scope

- richer combat-relative target restrictions beyond a single blocking creature
- post-blockers casting windows
- lethal or nonlethal damage outcomes for that target
