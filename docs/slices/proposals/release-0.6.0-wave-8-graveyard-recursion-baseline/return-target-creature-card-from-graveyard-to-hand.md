# Slice Proposal - Return Target Creature Card From Graveyard To Hand

## Goal

Support the first explicit recursion corridor by returning a target creature card from a graveyard to its owner's hand.

## Why This Slice

This is a very high-return graveyard rule that reuses existing zones and targeting without needing full battlefield reentry semantics.

## Scope

- target creature card in a graveyard
- move it to its owner's hand on resolution
- revalidate target legality if the card leaves the graveyard before resolution

## Out of Scope

- returning noncreature cards
- battlefield reanimation
- random or mass recursion

## Notes

- this is the safest first recursion slice
- it composes well with discard, destruction, and ETB casting loops
