# Slice: Derive Public Target Candidates From Canonical Legality Queries

Status: implemented

## Summary

The public gameplay surface now derives spell-target and ability-target candidates from canonical read-only aggregate queries instead of rebuilding target legality locally.

## What changed

- `Game` exposes read-only queries for legal spell targets and legal activated-ability targets
- those queries reuse the same target-legality corridor that already enforces the supported runtime rules
- the public gameplay surface now maps canonical `SpellTarget` candidates into UI-facing choice candidates

## Why it matters

- keeps supported `Hexproof` and other target refinements reflected in public choice requests
- removes a second target-legality implementation from the application layer
- keeps UI projection truthful without pulling UI concepts into the domain model
