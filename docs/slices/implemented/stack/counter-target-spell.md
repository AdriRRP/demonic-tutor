# Slice Name

Counter Target Spell

## Status

Implemented.

## Delivered Behavior

- a supported instant spell can target one spell currently on the stack
- legality is checked both when cast and when the counterspell resolves
- only spell stack objects are legal targets in the current subset
- the countered spell is removed from the stack and moved to graveyard
- if the targeted spell is already gone on resolution, the counterspell does nothing

## Notes

- this slice does not support countering activated abilities
- this slice does not support tax counters, uncounterable text, or replacement effects that change the destination of the countered spell
