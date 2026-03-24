# Implemented Slice — Expand State Based Actions For New Effects

## Summary

Confirm that the current explicit SBA subset remains sufficient after the newly implemented spell effects and combat keywords.

## Supported Behavior

- the currently supported SBA subset remains:
  - creature death for `0 toughness`
  - creature death for lethal marked damage
  - game loss for `0 life`
- that same subset now explicitly remains the shared correctness corridor for:
  - targeted damage effects
  - temporary pump effects
  - `Trample`
  - `First strike`
- `First strike` reuses the same SBA subset between its earlier and later supported damage passes

## Invariants

- no broader SBA are implied
- the supported checks remain explicit, ordered, and deterministic
- newly introduced gameplay semantics continue to rely on the shared SBA corridor instead of ad hoc destruction rules

## Implementation Notes

- no new SBA category was required by the implemented effect and combat slices
- the meaningful closure here is semantic: the supported subset is now explicitly documented as still sufficient after the new gameplay work

## Tests

- existing unit and BDD coverage for targeted damage, pump, trample, and first strike continue to pass through the same shared SBA corridor
