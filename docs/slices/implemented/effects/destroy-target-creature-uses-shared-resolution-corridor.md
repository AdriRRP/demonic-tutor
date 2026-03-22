# Destroy Target Creature Uses Shared Resolution Corridor

## Status

Implemented

## Scope

- `destroy target creature` reuses the shared cast-time and resolution-time legal-target evaluation
- if the creature target is gone on resolution, the spell applies no effect
- the destroy effect stays inside the normal stack-resolution corridor rather than bypassing it

## Out Of Scope

- triggered death handling
- regeneration or indestructible
- broader destroy effect families
