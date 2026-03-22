# Resolve Opponents Creature Spell In First Main

## Status

Implemented

## Scope

- a supported spell targeting `creature an opponent controls` resolves successfully in `FirstMain`
- resolution reuses the shared legal-target evaluation corridor
- lethal creature damage from this spell flows through the shared SBA-backed targeted-damage path

## Out Of Scope

- target changes after casting beyond the currently modeled subset
- multiple targets
- non-damage effects on opponent-controlled creatures
