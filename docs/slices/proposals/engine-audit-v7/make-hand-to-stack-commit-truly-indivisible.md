# Proposal Slice — Make Hand To Stack Commit Truly Indivisible

## Summary

Tighten the internal spell-casting commit so a cast either leaves the card fully in hand or fully prepared for stack insertion, with no intermediate desynchronization between hand and arena state.

## Motivation

- remove the last partial-mutation risk from the casting commit corridor
- make the casting core robust against internal desynchronization
- strengthen the aggregate before pushing identity and ownership refactors further

## Target Shape

- the internal hand-to-stack preparation path performs one indivisible ownership take
- mana payment is only finalized once card extraction cannot partially fail
- internal invariant errors do not leave hand and arena in divergent states

## Invariants

- accepted casts still remove exactly one spell from hand
- rejected casts still leave the card in hand and mana unchanged
- this slice does not expand supported Magic rules
