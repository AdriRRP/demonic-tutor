# Slice — Generalize Flash Support For Noncreature Spells

## Goal

Make the current explicit `Flash`-like support coherent across the currently supported noncreature spell subset.

## Implemented Behavior

- `Artifact`, `Enchantment`, and `Planeswalker` may carry `OpenPriorityWindow` on the card face
- the current exercised noncreature `Flash`-like windows are now covered for that subset:
  - stack response windows
  - `BeginningOfCombat`
  - post-blockers
  - post-combat-damage

## Notes

- this remains an explicit card-face casting permission, not generic keyword inference
- this slice does not imply universal `Flash` support for all noncreature spells
