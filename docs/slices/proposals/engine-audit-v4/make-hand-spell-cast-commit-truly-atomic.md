# Proposal Slice — Make Hand Spell Cast Commit Truly Atomic

## Summary

Tighten the player-owned hand-to-stack commit so mana payment and card extraction become one semantically atomic transition.

## Motivation

- remove the remaining mismatch between the semantic promise of `prepare_hand_spell_cast` and its mutation order
- prevent partial state updates if an internal desynchronization ever slips through
- leave the casting corridor safer before more storage optimizations arrive

## Target Shape

- hand spell preparation prevalidates a stable card handle or equivalent ownership token
- mana is only spent once card extraction can no longer fail unexpectedly
- the API still returns a prepared spell payload for the stack corridor

## Invariants

- failed spell preparation must not mutate the mana pool
- failed spell preparation must not mutate hand ownership
- this slice does not expand supported Magic rules
