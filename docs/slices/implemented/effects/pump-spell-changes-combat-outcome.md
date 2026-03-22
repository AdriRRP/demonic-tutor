# Implemented Slice — Pump Spell Changes Combat Outcome

## Summary

Prove that the currently supported temporary pump effect can change a combat result in the same turn.

## Supported Behavior

- a supported pump spell may be cast in the post-blockers combat priority window
- the temporary stat change affects later combat damage in that same turn
- the changed stats can turn a mutual trade into a surviving attacker and a dead blocker

## Invariants

- the buff lasts through the rest of the current turn
- combat damage and survival still use the shared combat and SBA corridors
- this slice does not add broader combat mechanics beyond the existing single-damage-step model

## Tests

- unit coverage proves the temporary pump changes combat damage that turn
- executable BDD proves a post-blockers pump changes the combat outcome before damage resolves
