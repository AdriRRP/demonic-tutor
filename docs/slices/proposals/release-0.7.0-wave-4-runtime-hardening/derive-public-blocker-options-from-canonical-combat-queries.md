# Slice: Derive Public Blocker Options From Canonical Combat Queries

Status: proposed

## Summary

Move blocker-option generation into a read-only combat query owned by the aggregate so the public contract stops reimplementing a simplified blocking legality.

## Scope

- add a read-only aggregate query for blocker options during `DeclareBlockers`
- make the public gameplay surface consume that query instead of rechecking combat rules locally
- ensure current restrictions such as `cannot_block` stay reflected in the UI prompt

## Out of scope

- widening supported combat rules
- introducing a generic combat simulation engine
- changing the command shape for `DeclareBlockers`

## Why now

The current public contract can advertise blocker assignments that the aggregate rejects. That drift gets worse as combat support grows.
