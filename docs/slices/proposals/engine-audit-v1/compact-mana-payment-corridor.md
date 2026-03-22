# Slice Proposal — Compact Mana Payment Corridor

## Goal

Collapse duplicated mana availability and payment work into one compact corridor built around the indexed mana representation.

## Why This Slice Exists Now

`ManaPool` already uses indexed color storage, but cost payment still repeats manual color iteration and `cast_spell` still performs a total-mana precheck before the real payment attempt. The next step is to let one payment corridor answer both legality and mutation.

## Supported Behavior

- mana payment uses one explicit indexed color iteration strategy
- spell casting relies on the payment corridor instead of a separate total-mana precheck
- current generic and colored mana semantics remain unchanged

## Invariants / Legality Rules

- colored requirements must still be satisfied exactly
- colored mana may still pay generic costs in the current model
- failed payment must leave the pool unchanged

## Out of Scope

- new mana colors or cost shapes
- mana burn
- alternative costs

## Domain Impact

- tighten `ManaPool` arithmetic around one legality-and-spend corridor
- reduce duplicate branching in spell casting and mana payment

## Ownership Check

Mana legality and payment belong to the aggregate-owned gameplay model.

## Documentation Impact

- `docs/domain/current-state.md` only if the supported mana wording changes
- this proposal file

## Test Impact

- unit tests for successful and failed payment preserving the pool correctly
- regression tests for current colored and mixed-cost support

## Rules Reference

- 106
- 601.2f

## Rules Support Statement

This slice optimizes the current supported mana-payment subset only. It does not broaden mana rules support.
