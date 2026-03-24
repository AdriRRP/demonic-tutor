# Implemented Slice — Push Target Legality Predicates Into Target Rules

## Summary

Move more legality semantics from the central targeting evaluator into the explicit target-rule types themselves.

## Supported Behavior

- current player, creature, and graveyard-card target semantics remain unchanged
- cast-time and resolution-time legality remain shared
- current target kinds remain unchanged

## Invariants

- legality remains explicit and deterministic
- unsupported target families remain unsupported
- this slice does not expand Magic rules support

## Implementation Notes

- `SingleTargetRule` now exposes direct legality predicates for:
  - player targets
  - creature targets
  - graveyard-card targets
- `spell_effects` now focuses on context assembly, target lookup, and legality result mapping

## Tests

- full targeting, stack, and effect-resolution regression coverage remains green
