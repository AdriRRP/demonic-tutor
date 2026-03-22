# Reject Non-Blocking Creature For Block-Only Spell

## Summary

Require a `BlockingCreature` spell target to be a creature that is actively blocking in the current combat state.

## Scope

- prove that battlefield existence alone is not enough for the `BlockingCreature` rule
- keep the rejection in the shared target-legality corridor used by cast-time spell validation

## Out Of Scope

- casting the spell in a post-blockers window
- damage resolution against a valid blocking creature
