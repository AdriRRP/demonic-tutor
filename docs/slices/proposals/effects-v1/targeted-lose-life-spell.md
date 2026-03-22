# Slice Proposal — Targeted Lose Life Spell

## Goal

Support a targeted spell that causes a player to lose life.

## Why This Slice Exists Now

`Lose life` is semantically distinct from damage and should not be smuggled in through the damage corridor. Adding it explicitly keeps life semantics honest as the spell subset grows.

## Supported Behavior

- a supported spell may target a legal player
- on resolution, the target loses the explicit amount of life
- zero-life game loss remains shared with the current life corridor

## Invariants / Legality Rules

- the spell requires one legal player target
- life loss reuses shared zero-life end-of-game review

## Out of Scope

- damage prevention
- life-loss replacement effects
- non-targeted life loss

## Domain Impact

- extend supported spell-resolution profiles with explicit life-loss semantics
- reuse current game-loss review for zero life

## Ownership Check

This remains aggregate-owned stack resolution and life-total legality.

## Documentation Impact

- current-state
- implemented slice doc

## Test Impact

- unit coverage for positive resolution and game-loss path
- executable BDD for one positive corridor

## Rules Reference

- 114
- 118
- 104.3b
- 704.5a

## Rules Support Statement

This slice adds targeted life loss as a distinct effect from damage. It does not imply prevention, replacement, or broader non-damage life semantics.
