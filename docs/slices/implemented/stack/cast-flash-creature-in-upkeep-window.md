# Cast Flash Creature In Upkeep Window

## Summary

Allow a supported creature card whose explicit casting rules include `OpenPriorityWindow` to be cast in `Upkeep`.

## Scope

- supported creature cards may carry an extra casting rule on their card face
- that rule can grant `Flash`-like access to currently supported open priority windows
- this slice proves the behavior in `Upkeep`

## Out Of Scope

- a generic keyword engine for `Flash`
- temporary or effect-granted casting permissions
- broader non-creature `Flash` support
