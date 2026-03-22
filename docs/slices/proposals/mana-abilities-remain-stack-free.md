# Slice Proposal — Mana Abilities Remain Stack Free

## Goal

Preserve stack-free semantics when the model grows from dedicated land taps into explicit activated mana abilities.

## Why This Slice Exists Now

The repo already proved stack-free land mana production. A future activated mana-ability abstraction must not accidentally regress that property.

## Supported Behavior

- supported mana abilities do not add stack objects
- activating a supported mana ability leaves current stack contents intact
- the acting player retains priority after activation

## Invariants / Legality Rules

- only supported mana abilities bypass the stack
- non-mana activations continue to use the normal priority and stack corridor

## Out of Scope

- triggered mana abilities
- multiplayer priority interactions

## Domain Impact

- may harden or centralize mana-ability classification

## Ownership Check

This is aggregate-owned stack and activation legality.

## Documentation Impact

- current-state
- implemented slice doc

## Test Impact

- regression coverage comparing mana vs non-mana ability activation behavior

## Rules Reference

- 605.1
- 605.3
- 405

## Rules Support Statement

This slice preserves a core semantic boundary: supported mana abilities remain stack-free even after abilities become a first-class part of the model.
