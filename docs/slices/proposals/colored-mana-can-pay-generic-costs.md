# Slice Proposal — Colored Mana Can Pay Generic Costs

## Goal

Allow colored mana in the pool to satisfy the generic portion of a spell cost.

## Why This Slice Exists Now

Mixed costs become misleading if colored mana can only satisfy colored symbols. This slice makes the payment model semantically honest for the already-supported generic and colored subset.

## Supported Behavior

- after satisfying required colored symbols, remaining colored mana may be used to pay generic cost
- a spell with only generic cost may be paid using colored mana already in the pool

## Invariants / Legality Rules

- colored requirements still reserve the matching colored mana first
- generic payment may consume any remaining mana regardless of color
- payment remains deterministic

## Out of Scope

- colorless-specific symbols
- mana-choice optimization for complex multicolor pools
- floating-mana UI concerns

## Domain Impact

- refine mana-payment ordering and spending logic
- keep the pool model small and aggregate-owned

## Ownership Check

This is part of spell-payment legality and therefore belongs to the aggregate-owned casting corridor.

## Documentation Impact

- `docs/domain/current-state.md`
- the implemented slice doc for this capability

## Test Impact

- unit tests for generic-only casts paid by colored mana
- mixed-cost tests that leave extra colored mana available for the generic remainder

## Rules Reference

- 106
- 107.4b
- 601.2f

## Rules Support Statement

This slice makes the current minimal mana model semantically truer by allowing colored mana to pay generic costs. It does not add colorless-symbol support.
