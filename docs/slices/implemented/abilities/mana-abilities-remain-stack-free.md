# Slice — Mana Abilities Remain Stack Free

## Outcome

Supported mana abilities remain semantically distinct from non-mana activations: they do not create stack objects and they leave the current stack contents intact.

## Supported Behavior

- tapping a supported mana-producing permanent does not put an object on the stack
- the current stack contents remain unchanged after mana production
- the acting player retains priority after producing mana in an open priority window
- non-mana activated abilities continue to use the stack corridor

## Notes

- the currently exercised mana-ability subset is still the land-tap corridor
- this slice preserves the semantic boundary after introducing explicit activated-ability profiles

## Executable Coverage

- unit coverage for mana production in supported open windows
- BDD coverage in `features/stack/tap_land_for_mana_does_not_use_the_stack.feature`
