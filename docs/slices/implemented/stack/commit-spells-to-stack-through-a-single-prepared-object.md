# Implemented Slice — Commit Spells To Stack Through A Single Prepared Object

## Summary

Tighten the casting corridor so the internal preparation phase produces one prepared stack spell object instead of leaving the final stack-object assembly split across later steps.

## Supported Behavior

- timing and target validation remain unchanged
- a successful cast still puts the same spell, controller, and target on the stack
- outward `SpellPutOnStack` events remain stable

## Invariants

- the prepared hand-cast corridor remains atomic with respect to mana payment and card extraction
- stack insertion still happens exactly once per accepted cast
- this slice does not expand supported Magic rules

## Implementation Notes

- casting now converts the prepared hand-cast result into a single prepared stack spell carrier before stack insertion
- `cast_spell` stays more linear and no longer recomposes the spell payload across several ad hoc locals

## Tests

- full casting, targeting, response-window, and BDD regression coverage remains green
