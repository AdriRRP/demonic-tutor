# Limited Set Deck Construction Baseline

This document defines the current deckbuilding baseline for the first curated limited environment.

It is a product and content contract, not a claim that every rule below is already enforced automatically by the aggregate.

Where enforcement does exist, the code is the source of truth.
Where enforcement does not yet exist, these rules describe the assumptions the first real UI, fixture decks, and golden matchups should follow.

---

## Scope

This baseline is for:

- two-player best-of-one play
- the first curated limited card pool
- preconstructed or builder-produced decks that will be loaded into the current engine

It is not yet a general Magic deck-construction system.

---

## Current Environment Rules

### Match Shape

- exactly two players
- one main deck per player
- no sideboard
- no between-game swaps
- no multiplayer-specific deck rules

### Main Deck

- target main-deck size: `40` cards
- the current playable environment should treat `40` as the canonical deck size for authored limited decks
- opening hands still use the normal supported `7`-card opening hand

### Card Pool

- every authored card in the deck must fit the current curated authoring contract
- that means:
  - it must fit the current `SupportedLimitedSetCardProfile` catalog
  - it must fit the published [limited-set capability matrix](limited-set-capability-matrix.md)
- cards outside that profile contract are not part of the current limited environment

### Copies

- duplicate policy for the first curated environment:
  - the environment may include repeated copies of the same `CardDefinitionId`
  - the aggregate does not currently enforce a copy limit
- because copy legality is not enforced in code yet, curated decklists must keep any intended copy policy in content/tooling space

### Library Ordering

- the aggregate consumes authored `PlayerLibrary` input as an explicit ordered library
- deck randomization and shuffle policy are outside the aggregate today
- any caller that wants shuffled opening libraries must provide them already ordered as desired before `deal_opening_hands`

### Unsupported Deckbuilding Concepts

The first curated environment intentionally excludes:

- sideboards
- wishboards
- companion-style extra-deck semantics
- commander-style deck identity rules
- color-identity legality
- singleton / Highlander constraints
- draft or sealed pack-generation logic inside the aggregate

---

## Enforcement Status

### Already Enforced

- two-player matches only
- opening-hand flow
- authored cards outside the curated-set profile catalog are rejected when `PlayerLibrary` input is loaded

### Not Yet Enforced By The Aggregate

- exact `40`-card deck size
- copy-count legality
- shuffle/randomization policy
- any future sideboard or format-specific legality

These remain caller, tooling, or fixture responsibilities for now.

---

## Operational Rule

For the first curated limited set:

- if a deck violates one of the enforced rules, the engine should reject it
- if a deck violates one of the non-enforced baseline rules, content, fixtures, and the UI should still treat that deck as outside the intended environment
