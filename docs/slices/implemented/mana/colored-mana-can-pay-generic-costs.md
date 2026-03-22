# Slice — Colored Mana Can Pay Generic Costs

## Goal

Allow colored mana in the pool to satisfy the generic portion of a spell cost.

## Supported behavior

- after satisfying required colored symbols, remaining colored mana may be used to pay generic cost
- a spell with only generic cost may be paid using colored mana already in the pool

## Current scope

This slice proves the rule through a generic-cost artifact paid with colored mana already in the pool.

It does not add colorless-symbol support or broader mana-choice optimization.

## Rules reference

- 106
- 107.4b
- 601.2f

## Rules support statement

DemonicTutor now explicitly supports colored mana paying generic costs inside the current minimal mana model.
