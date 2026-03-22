# Slice Name

CompactManaPaymentCorridor

---

## Goal

Collapse duplicated mana availability and payment work into one compact corridor built around the indexed mana representation.

---

## Why This Slice Exists Now

`ManaPool` already used indexed color storage, but spell casting still did a separate total-mana precheck before the real payment attempt.

This slice exists to:

1. let one payment corridor answer both legality and mutation
2. remove duplicated branching in spell casting
3. keep failed payment non-mutating in one place

---

## Supported Behavior

- mana payment now uses one explicit indexed color iteration strategy
- `cast_spell` relies on the payment corridor instead of a separate availability precheck
- current generic and colored mana semantics remain unchanged
- failed payment still leaves the pool unchanged

---

## Invariants / Legality Rules

- colored requirements must still be satisfied exactly
- colored mana may still pay generic costs in the current supported model
- payment failure must not partially mutate the mana pool

---

## Out of Scope

- new mana colors or cost shapes
- mana burn
- alternative costs

---

## Domain Impact

### Aggregate Impact

- mana legality and payment are now decided through one compact pool-owned corridor

### Entity / Value Object Impact

- `ManaColor` exposes a stable ordered set for indexed payment iteration

### Commands

- no new public command required

### Events

- no event payload changes

### Errors

- no new public error required

---

## Documentation Impact

- this slice document

---

## Test Impact

- full unit and BDD regression coverage remains green
- successful and failed payment still preserve current gameplay semantics

---

## Rules Reference

- 106
- 601.2f

---

## Rules Support Statement

This slice optimizes the current supported mana-payment subset only. It does not broaden mana rules support.
