# Implemented Slice — Make Hand To Stack Commit Truly Indivisible

## Summary

Tighten the internal spell-casting commit so a cast either leaves the card fully in hand or fully prepared for stack insertion, without leaving hand and arena state partially desynchronized.

## Supported Behavior

- accepted casts still remove exactly one spell from hand and prepare it for stack insertion
- rejected casts still leave mana unchanged
- internal desynchronization between hand and arena no longer leaves the arena permanently missing the card during a rejected cast

## Invariants

- the casting commit finalizes mana only after card extraction can complete without partial loss
- a failed extraction path rolls back arena ownership before returning an error
- this slice does not expand supported Magic rules

## Implementation Notes

- `PlayerCardArena` now supports an internal begin/commit/rollback removal flow
- `prepare_hand_spell_cast` uses that flow to guarantee rollback if hand extraction fails after the arena take begins
- the module now includes a focused regression test for the desynchronized hand/arena edge case

## Tests

- full `./scripts/check-all.sh` regression remains green
- a focused unit test proves the rollback on hand/arena desynchronization
