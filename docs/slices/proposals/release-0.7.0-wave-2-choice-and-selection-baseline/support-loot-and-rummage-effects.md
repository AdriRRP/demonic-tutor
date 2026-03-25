# Slice Proposal - Support Loot And Rummage Effects

## Goal

Support explicit `draw then discard` and `discard then draw` effects through prompt-driven selection.

## Why

- these effects are common, high-return, and test both public prompts and zone movement semantics
- they expand playable blue/red-style card patterns sharply in a limited subset
- they reuse and strengthen the hand-choice corridor already started by discard-choice support

## In Scope

- one explicit `loot` profile:
  - draw a bounded number of cards
  - choose one card from hand to discard
- one explicit `rummage` profile:
  - choose one card from hand to discard
  - then draw a bounded number of cards
- public choice projection for the discard selection
- one or two executable cards/tests for each corridor

## Out of Scope

- discard-at-random
- multiple simultaneous discard choices
- replacement interactions with drawing/discarding
- madness, flashback, threshold, or graveyard-trigger expansions

## Acceptance

- supported loot draws before prompting the discard
- supported rummage discards before drawing
- the selected card moves to graveyard through canonical discard semantics
- the public contract exposes the pending hand-card choice at the correct point in the sequence

## Notes

- keep the corridor prompt-driven and deterministic; do not introduce generic stack pauses beyond the explicit pending choice
