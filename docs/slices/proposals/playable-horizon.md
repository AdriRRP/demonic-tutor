# Playable Horizon To A Limited Set

This document defines the current proposal horizon from `v0.7.0` to an honestly playable limited subset.

It is intentionally explicit about both:

- when UI work should begin
- when the project may truthfully claim "you can play a game with a limited curated set"

It does not claim full Magic support.

## Target Horizon

The proposed horizon is:

- `1` release
- `2` waves
- `6` slices

The releases are:

- `0.8.0` — finish the curated limited-set contract and harden the first actually playable product shell

## Planning Gates

### UI Start Gate

The UI-start gate was reached in `0.7.0`.

That means the live horizon is no longer about unlocking the first honest client contract.
It is now about expanding that client against a richer, explicitly bounded curated card pool.

### Limited-Set Playable Gate

The first honest "limited set playable" milestone should now be `0.8.0`.

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

### `0.8.0`

Goal:

- freeze the curated playable subset
- prove real end-to-end matchups
- harden the public contract for a first real playable client

Waves:

- `2`
- `6` slices
