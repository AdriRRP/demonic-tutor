# Slice Proposal — Mana Model V1 Sync

## Goal

Synchronize canonical documentation after the minimal five-color and mixed-cost mana model lands.

## Why This Slice Exists Now

The mana model is growing from generic-only into a small but real colored subset. The truth needs to become easier to read than a long series of isolated slice notes.

## Supported Behavior

- current-state wording reflects the full supported mana subset
- glossary terms for mana colors, generic cost, and colored requirement stay aligned
- slice docs stop implying older generic-only limitations

## Invariants / Legality Rules

- documentation must not imply support for unsupported symbols or cost families
- the docs should describe the exact minimal subset, not full Magic mana

## Out of Scope

- runtime behavior changes
- new mana symbols or colors

## Domain Impact

- no runtime change
- documentation consolidation only

## Ownership Check

This belongs to canonical documentation because it synchronizes owned truth rather than changing aggregate behavior.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/DOMAIN_GLOSSARY.md`
- relevant implemented mana slice docs
- `features/README.md` if executable coverage expands

## Test Impact

- no new runtime tests
- optional check that related features remain correctly indexed

## Rules Reference

- 106
- 107.4
- 202

## Rules Support Statement

This slice is documentation-only. It compresses the supported mana subset into a stable, honest capability description.
