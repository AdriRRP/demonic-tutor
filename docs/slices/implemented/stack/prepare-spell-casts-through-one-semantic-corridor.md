# Implemented Slice — Prepare Spell Casts Through One Semantic Corridor

## Summary

Compact the supported spell-casting hot path so mana payment, hand extraction, and stack snapshot creation pass through one explicit preparation corridor.

## Supported Behavior

- casting legality remains unchanged
- target validation remains unchanged
- mana payment remains unchanged
- a successful cast still moves a spell from hand to stack through the same observable event flow

## Invariants

- illegal casts still fail before mutating hand or stack
- insufficient mana still rejects the cast
- this slice does not expand Magic rules support

## Implementation Notes

- the casting corridor now uses one local semantic preparation step for:
  - mana payment
  - hand removal
  - spell snapshot creation
- this reduces branching in `cast_spell` without widening public aggregate surface unnecessarily

## Tests

- full stack, mana, targeting, and BDD regression coverage remains green
