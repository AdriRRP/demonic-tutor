# Slice Proposal — Support Plains White Mana

## Goal

Extend the current minimal colored mana model so `Plains` produces `White`.

## Why This Slice Exists Now

The runtime already supports `Forest -> Green` and `Mountain -> Red`. Adding `White` continues the same model without changing casting semantics and prepares the five-color baseline needed for a stable engine.

## Supported Behavior

- a `Plains` card face may exist in setup
- tapping a `Plains` adds one `White` mana to the acting player's mana pool
- the corresponding mana-added events and tests reflect the produced color

## Invariants / Legality Rules

- a land produces exactly the mana color declared on its face
- tapping a `Plains` remains stack-free
- the acting player must still be allowed to tap the land in the current window

## Out of Scope

- white spell effects
- dual lands or multi-mana production
- colorless mana as a separate symbol

## Domain Impact

- extend land setup helpers and card-definition constructors with `White`
- extend mana-pool accounting to carry `White`
- keep generic mana and existing colors unchanged

## Ownership Check

This slice belongs to the `Game` aggregate and play-owned card-face model because mana production, legality, and mana-pool state are gameplay concerns already owned there.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/DOMAIN_GLOSSARY.md`
- the implemented slice doc for this capability

## Test Impact

- unit tests for tapping a `Plains`
- executable BDD only if the slice also exposes a gameplay corridor that spends `White`

## Rules Reference

- 106
- 107.4
- 305
- 605

## Rules Support Statement

This slice extends the current minimal colored mana subset by adding `White` as another produced mana color. It does not yet imply full color-symbol coverage or richer land behavior.
