# Slice: Trim Redundant Public Surface Card Scans

Status: implemented

## Summary

The public gameplay surface now derives more of its castable, activatable, and cleanup state directly from visible card iterators instead of materializing ids and then looking the same cards up again.

## What this slice adds

- one canonical `hand_cards()` iterator on `Player` alongside the existing visible-zone iterators
- castable-card projection built directly from visible hand and graveyard cards
- activatable-card projection built directly from visible battlefield cards
- cleanup-discard surface construction that reuses the same hand-id list for both the legal action and the choice request

## Why this matters

- the public read-side now does less redundant work in one of the hottest UI snapshot paths
- visible-zone access stays aligned with the harder fail-fast invariant model instead of drifting back toward ad hoc id round-trips
- the public surface gets a little cheaper without widening aggregate boundaries or inventing new abstractions

## Boundaries kept explicit

- this slice does not change legal-action semantics
- this slice does not widen the public contract
- this slice is a runtime/read-side cleanup, not a gameplay capability expansion
