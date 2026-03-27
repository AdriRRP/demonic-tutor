# Slice: Derive Public Target Candidates From Canonical Legality Queries

Status: proposed

## Summary

Move spell-target and ability-target candidate generation out of the public gameplay surface and into read-only aggregate queries that reuse canonical target-legality evaluation.

## Scope

- add read-only aggregate queries for legal spell targets and legal activated-ability targets
- make the public gameplay surface map those canonical results into UI-facing choice candidates
- ensure supported `Hexproof` and other current legality refinements are reflected in public target prompts

## Out of scope

- widening Magic target rules support
- changing supported targeting profiles
- adding multiplayer target-selection semantics

## Why now

The public contract already claims to surface legal actions and choice requests. That contract should not expose targets that the aggregate will reject moments later.
