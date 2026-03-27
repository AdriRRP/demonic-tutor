# Slice Implemented - Support Tap Target Creature Effects

## Outcome

The engine now supports the first explicit spell corridor that taps a target creature on the battlefield.

## What Landed

- a bounded `TapTargetCreature` spell-resolution profile
- one explicit supported targeting shape:
  - `target creature`
- resolution that taps the target creature if it is still legal when the spell resolves
- shared cast-time validation and resolution-time revalidation through the existing targeting corridor
- regression coverage for both successful resolution and target loss before resolution

## Notes

- this slice currently lands the spell corridor only
- generic non-mana activated abilities that tap another creature remain out of scope for now
- tapping an attacking creature does not introduce extra combat semantics beyond the existing runtime state
