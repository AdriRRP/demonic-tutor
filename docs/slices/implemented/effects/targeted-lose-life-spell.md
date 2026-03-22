# Implemented Slice — Targeted Lose Life Spell

## Summary

Support a targeted spell that causes a player to lose life.

## Supported Behavior

- a supported spell may target a legal player
- on resolution, the target loses the explicit amount of life
- zero-life game loss continues to reuse the shared life and SBA corridor

## Invariants

- the spell requires one legal player target
- this slice models life loss as distinct from damage
- this slice does not introduce prevention or replacement effects

## Tests

- unit coverage for positive resolution and lethal zero-life path
- executable BDD for a positive corridor in `FirstMain`
