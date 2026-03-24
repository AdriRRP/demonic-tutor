# Slice Proposal — Prepare Spell Casts Through One Semantic Corridor

## Goal

Compact the current casting flow so spell preparation, legality checks, payment, and card extraction are coordinated through one clearer semantic corridor.

## Why This Slice Exists Now

`cast_spell` has already been improved, but it still splits lookup, validation, mana payment, and hand extraction across several manual phases.

This slice exists to:

1. reduce duplicated lookup and branching in the hot path
2. keep spell-casting orchestration reviewable as one domain transition
3. prepare future casting growth on top of a smaller semantic surface

## Supported Behavior

- casting legality remains unchanged
- supported target validation remains unchanged
- mana payment remains unchanged
- a successfully cast spell still moves from hand to stack through the same observable event flow

## Invariants / Legality Rules

- illegal casts must still fail before mutating the hand or stack
- insufficient mana must still reject the cast
- no broader timing or stack support is implied

## Out of Scope

- new casting permissions
- new spell types
- new target families

## Domain Impact

- the aggregate gets a tighter spell-preparation corridor for supported casts

## Documentation Impact

- this slice document

## Test Impact

- full stack, mana, and targeting regression coverage remains green

## Rules Reference

- none beyond current casting support

## Rules Support Statement

This slice is a casting-corridor refactor only. It does not expand Magic rules support.
