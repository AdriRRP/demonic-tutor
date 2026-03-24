# Implemented Slice — Vigilance Creatures Do Not Tap To Attack

## Summary

Support `Vigilance` so a creature can attack without becoming tapped.

## Supported Behavior

- a creature with `Vigilance` may be declared as an attacker without tapping
- non-vigilance attackers still tap when declared

## Invariants

- `Vigilance` does not change who can attack; it changes only the tap outcome of declaring attackers
- this slice does not add temporary vigilance-granting effects or broader activated-ability interactions

## Implementation Notes

- attack declaration keeps the existing attacking state transition
- the tap-on-attack transition is now conditional on the creature lacking `Vigilance`

## Tests

- unit coverage for vigilance and non-vigilance attack outcomes
- executable BDD coverage in `features/combat/vigilance_creatures_do_not_tap_to_attack.feature`
