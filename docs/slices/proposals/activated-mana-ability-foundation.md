# Slice Proposal — Activated Mana Ability Foundation

## Goal

Introduce an explicit activated mana-ability concept for supported permanents.

## Why This Slice Exists Now

Land tapping already behaves like a mana ability, but the model still treats it as a dedicated action. A minimal activated mana-ability abstraction is the next honest step before broader activated abilities.

## Supported Behavior

- a supported permanent may expose an activated mana ability
- activating that ability produces mana directly into the controller's pool

## Invariants / Legality Rules

- supported mana abilities remain stack-free
- activating the ability still obeys the current legality corridor for the acting player and window

## Out of Scope

- non-mana activated abilities
- loyalty abilities
- activated abilities with costs beyond tapping

## Domain Impact

- introduce a minimal activated-ability profile for mana production only
- keep the abstraction closed and aggregate-owned

## Ownership Check

Ability activation legality and resource mutation belong to the `Game` aggregate.

## Documentation Impact

- current-state
- aggregate-game if runtime responsibilities materially change
- implemented slice doc

## Test Impact

- unit tests for land-as-mana-ability parity
- executable BDD only if the public action surface changes

## Rules Reference

- 602
- 605

## Rules Support Statement

This slice introduces a minimal activated mana-ability abstraction only. It does not imply general activated-ability support.
