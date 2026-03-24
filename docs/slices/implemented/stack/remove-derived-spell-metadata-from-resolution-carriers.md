# Implemented Slice — Remove Derived Spell Metadata From Resolution Carriers

## Summary

Stop duplicating card type and supported spell rules in resolved-spell carriers when that metadata is already derivable from the payload being resolved.

## Motivation

- reduce duplicated semantic state in a hot resolution corridor
- keep one canonical source of truth for resolved spell metadata
- simplify the next payload-thinning steps

## Delivered Shape

- `ResolvedSpellObject` now carries the `payload` plus only the metadata that is not derivable from it
- `card_type` and `supported_spell_rules` are read from `payload` at the usage point inside resolution
- extraction stays explicit without widening the carrier

## Invariants

- spell resolution behavior remains unchanged
- events and effects still observe the same supported subset
- this slice does not expand supported Magic rules
