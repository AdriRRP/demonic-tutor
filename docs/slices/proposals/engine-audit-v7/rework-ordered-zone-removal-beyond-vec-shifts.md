# Proposal Slice — Rework Ordered Zone Removal Beyond Vec Shifts

## Summary

Replace the current `Vec::remove` plus suffix reindexing strategy in ordered player zones with a structure that preserves visible order without linear full-suffix rewrites on every removal.

## Motivation

- remove the current O(n) suffix rewrite cost from hand, graveyard, and exile removals
- keep visible positional access compatible with embedded-class throughput goals
- land on a more final ordered-zone abstraction instead of another intermediate stop

## Target Shape

- visible ordered-zone access remains direct and truthful
- preserved-order removals avoid full suffix shifts and complete positional rewrites
- the new structure keeps membership and positional lookup explicit

## Invariants

- hand, graveyard, and exile still preserve visible insertion order
- battlefield remains intentionally separate unless its semantics also change
- this slice does not expand supported Magic rules
