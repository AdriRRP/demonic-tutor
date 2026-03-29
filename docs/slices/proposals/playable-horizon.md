# Playable Horizon To A Limited Set

This document records the now-closed proposal horizon that ran from `v0.7.0` to the first honestly playable limited subset.

It is intentionally explicit about both:

- when UI work should begin
- when the project may truthfully claim "you can play a game with a limited curated set"

It does not claim full Magic support.

## Target Horizon

The proposed horizon is:

- `0` releases
- `0` waves
- `0` slices

The current horizon is closed for the `v0.8.0` release cut.

## Planning Gates

### UI Start Gate

The UI-start gate was reached in `0.7.0`.

That means the live horizon is no longer about unlocking the first honest client contract.
It is now about expanding that client against a richer, explicitly bounded curated card pool.

### Limited-Set Playable Gate

The first honest "limited set playable" milestone was reached in `0.8.0`.

That release now has:

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

### `0.8.0`

Outcome:

- frozen curated playable subset
- real end-to-end golden matchups
- public runtime contract hardened for a first real playable client
