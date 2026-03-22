# Slice Proposal — Generalize Flash Support For Noncreature Spells

## Goal

Generalize the current explicit `Flash`-like support for the already-supported noncreature spell subset.

## Why This Slice Exists Now

The repo currently exercises `Artifact` and `Enchantment` with several `OpenPriorityWindow` corridors. The next coherent step is to describe and implement that support as one supported noncreature family instead of as isolated type-specific cases.

## Supported Behavior

- the current supported noncreature permanent subset may be opened to legal instant-speed windows through explicit casting rules on the card face
- the current tested windows remain supported

## Invariants / Legality Rules

- the support statement must remain explicit about which noncreature types are included
- the current stack legality still depends on explicit supported casting rules, not generic type inference

## Out of Scope

- all noncreature spells
- activated abilities
- triggered abilities

## Domain Impact

- may collapse duplicate card-face casting-rule setup
- should not widen the supported subset beyond the currently exercised types without explicit tests

## Ownership Check

Casting legality remains aggregate-owned and card-face driven.

## Documentation Impact

- current-state
- glossary if the supported casting-rule family gets a stable term
- implemented slice doc

## Test Impact

- likely mostly refactor-safe regression coverage across current noncreature flash cases

## Rules Reference

- 117
- 601
- 702.8

## Rules Support Statement

This slice makes the current noncreature flash-like subset more coherent. It does not imply universal `Flash` support across all spell families.
