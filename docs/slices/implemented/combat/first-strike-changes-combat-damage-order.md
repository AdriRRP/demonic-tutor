# Implemented Slice — First Strike Changes Combat Damage Order

## Summary

Support `First strike` so first-strike creatures deal combat damage in an earlier supported pass and creatures destroyed there do not deal normal combat damage later that combat.

## Supported Behavior

- creatures with `First strike` deal combat damage before creatures without it
- creatures destroyed by first-strike combat damage do not deal normal combat damage later that combat
- when no creature in combat has `First strike`, combat damage keeps the current single-pass behavior

## Invariants

- this remains the current one-blocker combat model
- the slice adds only the minimal two-pass combat-damage support required for `First strike`
- `Double strike` is still out of scope

## Implementation Notes

- the combat-damage corridor now performs an earlier pass only when at least one combat participant has `First strike`
- state-based actions are reviewed between the first-strike and normal passes
- combat state is cleared only after the supported damage passes finish

## Tests

- unit coverage for first-strike lethal damage preventing normal retaliation
- executable BDD coverage in `features/combat/first_strike_changes_combat_damage_order.feature`
