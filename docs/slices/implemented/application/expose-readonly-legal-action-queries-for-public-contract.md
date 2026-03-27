# Slice: Expose Readonly Legal Action Queries For Public Contract

Status: implemented

## Summary

The public gameplay contract now derives legal actions from read-only aggregate queries instead of cloning `Game` and probe-executing commands per candidate.

## What changed

- `Game` exposes read-only legality queries for land play, mana tapping, spell candidacy, activated-ability candidacy, and attacker candidacy
- the public gameplay surface now uses those queries to build legal actions
- UI-facing projection remains outside the domain model, but it no longer depends on speculative aggregate mutation

## Why it matters

- removes `Game::clone()` from the public read path
- keeps UI concerns out of domain mutation logic
- makes the public contract scale better as the action surface grows
