# Slice Name

Return Target Permanent To Hand

## Status

Implemented.

## Delivered Behavior

- a supported instant can target one permanent currently on the battlefield
- target legality is checked both on cast and on resolution
- the targeted permanent returns to its owner's hand
- if the target is already gone on resolution, the spell does nothing

## Notes

- this slice is limited to battlefield permanents in the currently modeled subset
- it does not introduce modal bounce, replacement effects, or non-battlefield return effects
