# Slice — PlayCreature

## Status

Superseded by [`cast-spell.md`](./cast-spell.md).

## Summary

This slice originally introduced a dedicated `PlayCreatureCommand` and a creature-specific event.

That command no longer exists. The current model uses `CastSpellCommand` for creature spells, which matches the domain more accurately:

- creatures are spells while being cast
- creature spells resolve to the battlefield
- creature validation still requires power and toughness
- summoning sickness is still applied through the creature runtime model

This document remains only as historical context for the repository's incremental evolution.
