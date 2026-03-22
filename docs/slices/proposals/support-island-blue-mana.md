# Slice Proposal — Support Island Blue Mana

## Goal

Extend the current minimal colored mana model so `Island` produces `Blue`.

## Why This Slice Exists Now

After `White`, `Blue` continues the same low-risk expansion pattern. The goal is to reach a stable five-color minimal mana base before modeling richer mixed costs or broader spell families.

## Supported Behavior

- an `Island` card face may exist in setup
- tapping an `Island` adds one `Blue` mana to the acting player's mana pool
- mana-production events and tests expose the produced `Blue` color

## Invariants / Legality Rules

- tapping an `Island` remains stack-free
- only the acting priority holder may tap the land in stack-aware windows
- the mana pool preserves existing colors and generic accounting

## Out of Scope

- blue-specific spell rules
- hybrid mana
- nonbasic lands

## Domain Impact

- extend land face setup with `Blue`
- extend mana-pool accounting with `Blue`
- keep the current single-color production model

## Ownership Check

Mana production remains aggregate-owned because it mutates legal game state and is validated against current priority and battlefield state.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/DOMAIN_GLOSSARY.md`
- the implemented slice doc for this capability

## Test Impact

- unit coverage for `Island -> Blue`
- optional BDD if paired with a blue-cost spell slice

## Rules Reference

- 106
- 107.4
- 305
- 605

## Rules Support Statement

This slice adds `Blue` to the same minimal colored mana model already used for `Green` and `Red`. It does not yet imply full color-symbol support.
