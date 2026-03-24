# Implemented Slice — Make Spell Casting An Atomic Semantic Commit

## Summary

Move the hand-to-stack commit of a spell into one player-owned semantic operation that pays mana, removes the card from hand, and prepares stack payload atomically.

## Supported Behavior

- validated spell casts now commit through one player-owned corridor
- mana payment and hand removal happen together
- stack insertion consumes an already-prepared spell payload

## Invariants

- illegal casts still leave hand and mana untouched
- timing and target legality stay unchanged
- this slice does not expand supported Magic rules

## Implementation Notes

- `Player::prepare_hand_spell_cast(...)` is now the semantic commit point
- the stack casting corridor translates commit failures into existing domain errors
- ownership transfer no longer depends on a free-standing helper path

## Tests

- full repository validation remains green after the casting corridor compaction
