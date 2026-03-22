# Slice Proposal — Contextual Noncombat Target Matrix Sync

## Goal

Compress the supported non-combat target rules into one clear canonical matrix.

## Why This Slice Exists Now

The non-combat targeting subset is growing into a family: `opponent player`, `any player`, `creature you control`, and `creature an opponent controls`. A matrix is easier to keep honest than a long list of scattered slice notes.

## Supported Behavior

- documentation explicitly summarizes the supported non-combat target rules
- the matrix states which positive and negative corridors are exercised today

## Invariants / Legality Rules

- docs must not imply support for unsupported restrictions such as `any target`, `graveyard player`, or multiplayer-relative rules

## Out of Scope

- runtime behavior changes
- combat-relative targeting documentation beyond cross-reference

## Domain Impact

- no runtime change
- documentation-only consolidation

## Ownership Check

This belongs to canonical documentation.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/DOMAIN_GLOSSARY.md`
- relevant implemented target slice docs

## Test Impact

- no runtime tests

## Rules Reference

- 114
- 601.2c

## Rules Support Statement

This slice is documentation-only. It keeps the supported non-combat target subset explicit and reviewable.
