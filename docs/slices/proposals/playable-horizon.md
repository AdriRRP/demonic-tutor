# Playable Horizon To A Limited Set

This document defines the current proposal horizon from `v0.6.0` to an honestly playable limited subset.

It is intentionally explicit about both:

- when UI work should begin
- when the project may truthfully claim "you can play a game with a limited curated set"

It does not claim full Magic support.

## Target Horizon

The proposed horizon is:

- `2` releases
- `10` waves
- `36` slices

The releases are:

- `0.7.0` — stabilize the public gameplay contract and ship the first broad honest gameplay subset for UI work
- `0.8.0` — finish the curated limited-set contract and harden the first actually playable product shell

## Planning Gates

### UI Start Gate

UI work should begin after `0.7.0`.

At that point the project should have:

- a stable public game snapshot
- a stable legal-action surface
- explicit prompt/choice projections
- enough common card patterns that the UI is not designed around a toy subset

This gate is now considered reached on `main`.

That means the next planning horizon is no longer "what must exist before any UI work starts", but rather:

- what should land before the UI has a richer limited-like card pool to exercise
- what should still remain explicit and rejected while the first client is being built

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

### `0.7.0`

Goal:

- make the engine stable enough for UI work
- unlock the first broad enough gameplay subset that the UI can be built against the real engine contract

Waves:

- `6`
- `22` slices

### `0.8.0`

Goal:

- finish the missing snowball/combat-trigger patterns
- freeze the curated playable subset
- prove real end-to-end matchups
- harden the public contract for a first real playable client

Waves:

- `4`
- `14` slices
