# Slice Name

Destroy Target Artifact Or Enchantment

## Status

Implemented.

## Delivered Behavior

- a supported spell can target an artifact on the battlefield
- a supported spell can target an enchantment on the battlefield
- legality is checked on cast and again on resolution
- the permanent is moved from battlefield to graveyard if still legal
- if the target is already gone on resolution, the spell does nothing

## Notes

- this slice does not support indestructible, regeneration, or mixed-type permanent rules beyond the current subset
