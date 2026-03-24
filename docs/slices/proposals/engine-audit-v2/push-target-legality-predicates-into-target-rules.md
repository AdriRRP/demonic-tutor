# Slice Proposal — Push Target Legality Predicates Into Target Rules

## Goal

Move more legality semantics from the central targeting evaluator into the explicit target-rule types themselves.

## Why This Slice Exists Now

The current targeting evaluator still owns too much branching over player, creature, and graveyard-card rules. That works today, but it will scale worse as target families grow.

This slice exists to:

1. keep targeting growth aligned with explicit rule objects
2. reduce central procedural branching
3. make legality semantics easier to extend without reworking one large function

## Supported Behavior

- current target kinds remain unchanged
- cast-time and resolution-time legality remain shared
- current player, creature, and graveyard-card target semantics remain unchanged

## Invariants / Legality Rules

- legality must still be deterministic and explicit
- unsupported target families remain unsupported
- no broader Magic targeting support is implied

## Out of Scope

- new target kinds
- triggered retargeting
- multilayer continuous effects

## Domain Impact

- target-rule value objects carry more of their own legality semantics

## Documentation Impact

- this slice document

## Test Impact

- targeting, stack, and effect-resolution regression coverage remains green

## Rules Reference

- none beyond current targeting support

## Rules Support Statement

This slice is a targeting-architecture refactor only. It does not expand Magic rules support.
