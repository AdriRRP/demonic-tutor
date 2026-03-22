# Implemented Slice — Pump Target Creature Until End Of Turn

## Summary

Support a spell that gives a target creature `+N/+N until end of turn`.

## Supported Behavior

- a supported spell may target a legal creature on the battlefield
- on resolution, the target creature gets the explicit temporary stat increase
- the temporary increase changes the creature's current power and toughness for the rest of the turn
- the increase expires when the game leaves `EndStep`

## Invariants

- the effect applies only if the target remains legal on resolution
- the slice supports only direct temporary `+N/+N` stat changes
- this does not introduce a general continuous-effects or layers engine

## Implementation Notes

- temporary stat bonuses live in creature runtime
- end-of-turn cleanup clears both marked damage and the currently supported temporary stat bonuses
- combat reads current creature stats, so the temporary increase affects combat damage during the same turn

## Tests

- unit coverage for apply, expire, and same-turn combat impact
- executable BDD coverage for a positive targeted cast in `FirstMain`
