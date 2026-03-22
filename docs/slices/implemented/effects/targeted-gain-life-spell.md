# Implemented Slice — Targeted Gain Life Spell

## Summary

Support a targeted spell that causes a player to gain life.

## Supported Behavior

- a supported spell may target a legal player
- on resolution, the target gains the explicit amount of life
- the spell reuses the shared life-change corridor and `LifeChanged` event

## Invariants

- the spell requires one legal player target
- this slice does not introduce prevention or replacement effects on life gain

## Tests

- unit coverage for positive cast and resolution
- executable BDD for a positive corridor in `FirstMain`
