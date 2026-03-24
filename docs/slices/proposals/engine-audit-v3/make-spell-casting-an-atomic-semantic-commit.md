# Proposal Slice — Make Spell Casting An Atomic Semantic Commit

## Summary

Collapse the current multi-phase casting corridor into one semantic commit point that validates, pays, removes from hand, and prepares the stack payload atomically.

## Motivation

- reduce lookup duplication in the hot cast path
- make the central invariant easier to review and harder to break
- keep ownership transfer semantics concentrated in one place

## Target Shape

- an internal cast-preparation object or corridor owns the whole commit
- mana payment and hand removal become one semantic operation
- stack insertion consumes the already-prepared cast payload

## Invariants

- timing and target validation stay unchanged
- illegal casts still leave hand and mana untouched
- this slice does not expand supported Magic rules
