# Playable Horizon To A Limited Set

This document defines the current proposal horizon from `v0.6.0` to an honestly playable limited subset.

It is intentionally explicit about both:

- when UI work should begin
- when the project may truthfully claim "you can play a game with a limited curated set"

It does not claim full Magic support.

## Target Horizon

The proposed horizon is:

- `3` releases
- `9` waves
- `36` slices

The releases are:

- `0.7.0` — stabilize the public gameplay contract and add choice-heavy card patterns
- `0.8.0` — broaden board texture and common limited-card interactions
- `0.9.0` — lock a curated limited-set contract and prove it with golden end-to-end matchups

## Planning Gates

### UI Start Gate

UI work should begin after `0.7.0`.

At that point the project should have:

- a stable public game snapshot
- a stable legal-action surface
- explicit prompt/choice projections
- enough common card patterns that the UI is not designed around a toy subset

### Limited-Set Playable Gate

The first honest "limited set playable" milestone should be `0.9.0`.

At that point the project should have:

- a curated card pool whose supported patterns are explicit
- golden decks and golden gameplay scenarios covering the intended archetypes
- a public runtime contract suitable for an actual UI
- rejection of cards that exceed the curated subset

## Constrained Set Assumptions

This horizon assumes the first playable set deliberately avoids the following families:

- control-changing effects
- copy effects
- replacement and prevention effects
- library search and shuffle-heavy tutor packages
- multiplayer semantics
- broad layers/timestamp/dependency coverage beyond the explicit static slices below

If the curated set later needs those families, the roadmap must expand.

## Release Shape

### `0.7.0`

Goal:

- make the engine stable enough for UI work
- unlock common choice-driven and attachment-driven cards

Waves:

- `3`
- `12` slices

### `0.8.0`

Goal:

- broaden the gameplay texture of a curated limited environment
- add the most common missing board-control, token, counter, trigger, and graveyard patterns

Waves:

- `3`
- `12` slices

### `0.9.0`

Goal:

- freeze the curated playable subset
- prove real end-to-end matchups
- harden the public contract for UI work

Waves:

- `3`
- `12` slices
