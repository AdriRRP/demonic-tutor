# Slice: Derive Public Blocker Options From Canonical Combat Queries

Status: implemented

## Summary

The public gameplay surface now derives blocker options from a read-only combat query owned by the aggregate instead of reimplementing a simplified blocker legality check.

## What changed

- `Game` exposes a read-only blocker-options query for the current supported `DeclareBlockers` corridor
- the query reflects the current supported combat restrictions, including blockers that currently cannot block
- the public gameplay surface now projects those canonical blocker options directly

## Why it matters

- stops the UI from advertising blocker assignments that the aggregate would reject
- keeps combat legality inside the gameplay domain instead of duplicating it in the application layer
- reduces future drift as the supported combat subset grows
