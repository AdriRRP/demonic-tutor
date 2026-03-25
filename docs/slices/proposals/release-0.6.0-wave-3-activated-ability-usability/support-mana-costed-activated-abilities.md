# Slice Name

Support Mana-Costed Activated Abilities

## Goal

Allow supported activated abilities to require explicit mana payment in addition to any tap cost.

## Why This Slice Exists Now

Many usable permanents only become meaningful once activated abilities can spend mana. The engine already has mana payment and stack activation corridors, so this is a natural convergence slice.

## Supported Behavior

- allow supported activated abilities to declare a mana cost
- reserve and spend mana during activation
- reject activation when the controller cannot pay

## Invariants / Legality Rules

- mana payment must succeed before the ability is put on the stack
- activation cost payment remains atomic with stack insertion
- mana abilities remain stack-free and separate from this slice

## Out of Scope

- hybrid or phyrexian activation costs
- mana refunds or partial payment
- variable activation costs

## Domain Impact

### Aggregate Impact

- reuse and extend mana payment for non-mana activated abilities

## Ownership Check

This belongs to the `Game` aggregate because cost payment and activation legality are core gameplay rules.

## Documentation Impact

- `docs/domain/current-state.md`
- this slice doc

## Test Impact

- activate a supported ability with sufficient mana
- reject activation without enough mana
- preserve atomicity if activation fails mid-cost

## Rules Reference

- 602
- 601.2f
- 605

## Rules Support Statement

This slice adds explicit mana-costed activation for a supported subset only. It does not imply generic arbitrary activation-cost parsing.

