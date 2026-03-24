# Implemented Slice — Stop Reconstructing Cards Across Stack Resolution

## Summary

Reduce stack-resolution churn by keeping resolved spells as `SpellPayload` until their final destination decides whether a full `CardInstance` must be materialized.

## Supported Behavior

- stack resolution still produces the same spell outcomes and events
- extracting a resolving spell no longer rebuilds `CardInstance` eagerly
- card materialization now happens only in the destination corridor that truly needs it

## Invariants

- observable spell outcomes remain unchanged
- supported permanent spells still enter the battlefield with the same runtime semantics
- this slice does not expand supported Magic rules

## Implementation Notes

- `ResolvedSpellObject` now carries `SpellPayload`
- stack extraction remains lightweight until destination routing is known
- `CardInstance` reconstruction moved out of extract-time and into the final zone destination step

## Tests

- full repository validation remains green after the lighter resolution corridor refactor
