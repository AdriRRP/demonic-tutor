# Slice — Exile Target Creature Foundation

## Goal

Support a targeted spell that exiles a creature from the battlefield.

## Supported Behavior

- a supported targeted spell may choose a legal creature on the battlefield
- on resolution, the target creature is moved to its owner's exile zone
- the shared resolution corridor now surfaces `CardMovedZone(origin -> Exile)` as the public visible zone-move event for the effect

## Invariants

- the spell requires one legal creature target
- the effect applies only if the target remains legal on resolution
- the creature moves to its owner's exile zone, not the caster's by default

## Notes

- this slice supports exile from battlefield only
- it does not imply blink, return-from-exile, or multi-zone exile effects
