# Slice Implemented - Support Creature Aura Casting And Attachment

## Outcome

The engine now supports the first bounded Aura corridor.

## What Landed

- an explicit `Enchant creature` attachment profile for the current subset
- `Enchantment` spells that target exactly one creature while being cast
- Aura resolution that enters the permanent attached only if the target is still legal
- Aura fallback to graveyard if the target is illegal on resolution
- state-based cleanup that moves the supported Aura to graveyard if it becomes unattached
- public battlefield projection of the attached creature id for clients

## Notes

- this slice does not yet grant stat bonuses or combat restrictions from the Aura
- it is intentionally the attachment baseline that later Aura slices build on

