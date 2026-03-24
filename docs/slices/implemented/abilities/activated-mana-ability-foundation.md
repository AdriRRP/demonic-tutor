# Slice — Activated Mana Ability Foundation

## Goal

Introduce an explicit activated mana-ability concept for the currently supported mana-producing permanents.

## Implemented Behavior

- supported permanents may expose an explicit `ActivatedManaAbilityProfile`
- lands currently use that profile for their tap-for-mana behavior
- activating the current supported mana ability remains stack-free and follows the existing legality corridor for `tap_land`

## Notes

- this slice does not introduce generic non-mana activated abilities
- the current public action surface remains `tap_land`; the new ability concept is aggregate-owned runtime structure
