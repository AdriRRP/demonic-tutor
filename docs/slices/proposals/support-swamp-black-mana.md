# Slice Proposal — Support Swamp Black Mana

## Goal

Extend the current minimal colored mana model so `Swamp` produces `Black`.

## Why This Slice Exists Now

`Black` is the next low-risk color addition on the path to a full five-color minimal mana baseline. It reuses the same abstractions as the existing colored mana subset.

## Supported Behavior

- a `Swamp` card face may exist in setup
- tapping a `Swamp` adds one `Black` mana to the acting player's mana pool
- events and tests expose the produced `Black` color

## Invariants / Legality Rules

- the land remains a normal stack-free mana ability
- tapping legality remains unchanged from the current land-tap corridor
- mana-pool totals still reflect total and per-color holdings

## Out of Scope

- black-specific spell effects
- multi-color lands
- life-payment costs

## Domain Impact

- extend land setup and face constructors with `Black`
- extend mana-pool color accounting with `Black`

## Ownership Check

This is owned by the existing play domain model for lands and mana because it only extends already-owned runtime state and legality.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/DOMAIN_GLOSSARY.md`
- the implemented slice doc for this capability

## Test Impact

- unit tests for `Swamp -> Black`
- BDD only if coupled to a black-cost spell corridor

## Rules Reference

- 106
- 107.4
- 305
- 605

## Rules Support Statement

This slice adds `Black` to the current minimal colored mana subset. It does not yet implement broader black card behavior.
