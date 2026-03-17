# Game Aggregate — DemonicTutor

## Purpose

This document describes the structure and responsibilities of the `Game` aggregate in DemonicTutor.

It serves as a reference for:

- understanding the current domain model
- clarifying which responsibilities belong to the aggregate
- documenting the conceptual state of the system
- preventing accidental domain overreach when extending gameplay

Detailed evolution of the system is documented in:

```
docs/slices/

```

---

# Aggregate Overview

`Game` is the **aggregate root** of the `play` bounded context.

It represents a single playtest session between players.

The aggregate is responsible for:

- maintaining the authoritative state of the match
- enforcing gameplay invariants
- applying commands
- emitting domain events describing state transitions

All external interactions with the match must occur **through commands handled by the aggregate**.

---

# Conceptual Aggregate State

At the current stage of the system, the aggregate conceptually maintains:

- game identity
- active player
- current phase
- turn number
- participating players
- optional terminal game outcome (`winner`, `loser`, `end reason`)

Each player maintains their own game zones and state within the aggregate.

---

# Internal Entities

The aggregate contains several internal entities.

These entities are **not aggregates themselves** and are fully controlled by `Game`.

---

## Player

Represents a participant in the match.

Responsibilities:

- hold references to the player's deck identity
- manage personal card zones
- track life total
- track per-turn state
- expose state required for gameplay operations

Players are entities contained within the aggregate.

---

## Library

Represents a player's draw pile.

Current implementation:

- ordered collection of `CardInstance`

Responsibilities:

- provide cards when drawn
- maintain card ordering
- enforce draw availability

Not yet responsible for:

- shuffle rules
- deck legality
- format validation

---

## Hand

Represents the cards currently held by a player.

Responsibilities:

- receive cards drawn from library
- expose playable cards

Current implementation:

- collection of `CardInstance`

---

## Battlefield

Represents permanents currently in play.

Responsibilities:

- receive permanents played or cast
- expose battlefield state

Current implementation:

- collection of `CardInstance`

The battlefield currently models only a minimal subset of permanent state.

---

## Graveyard

Represents cards that have resolved or otherwise left active play.

Responsibilities:

- receive instants and sorceries after simplified resolution
- preserve card history once they leave the active battlefield model

Current implementation:

- collection of `CardInstance`

---

## CardInstance

Represents a concrete instance of a card inside a match.

Fields include:

- `CardInstanceId`
- `CardDefinitionId`
- `CardType`
- tapped state
- mana cost
- power (for creatures)
- toughness (for creatures)
- has_summoning_sickness (for creatures)
- is_attacking (for creatures)
- is_blocking (for creatures)
- damage marked on the creature

Responsibilities:

- uniquely identify cards within a match
- reference card definitions
- track minimal runtime state required for gameplay

The current model includes:

- power and toughness for creature cards
- summoning sickness tracking and automatic removal at turn start
- declare attackers and blockers in combat phase
- marked combat damage on creatures
- automatic destruction of creatures with lethal marked damage

Card instances can be checked for whether they represent permanents (cards that can exist on the battlefield) using the `CardType::is_permanent()` method.

The current model intentionally omits:

- rules text
- triggered abilities
- counters
- stack interactions

These may be introduced incrementally in future slices.

---

# Aggregate Invariants

The `Game` aggregate currently guarantees:

- exactly two players exist in a match
- players are uniquely identified
- card instances belong to exactly one player
- cards cannot be drawn if not available
- the game ends if a required draw cannot happen because the relevant library is empty
- the game ends if a player's life total reaches 0
- card movements maintain zone consistency
- end-of-turn cleanup discard must reduce the active player's hand to the maximum before the turn can advance
- gameplay actions are rejected once the game is in a terminal state
- gameplay operations emit domain events

These invariants are enforced whenever commands are applied.

---

# Responsibilities of the Aggregate

The aggregate root must enforce:

- player existence
- player uniqueness
- valid card movement between zones
- creature spell validation including power/toughness presence before battlefield entry
- turn progression rules
- phase progression rules
- active-player-only automatic turn updates
- terminal game tracking for empty-library draw and zero-life loss
- lethal-damage creature destruction after combat damage resolution
- correct event emission

The aggregate must remain:

- deterministic
- infrastructure-free
- explicit in its state transitions

---

# Responsibilities Outside the Aggregate

Several concerns intentionally live outside the aggregate.

## Deck Context

Responsible for:

- deck construction
- deck legality
- deck persistence

---

## Rules Engine (future)

Responsible for:

- stack resolution
- card abilities
- triggered effects
- replacement effects

---

## Infrastructure

Responsible for:

- persistence
- event store
- event bus
- projections

---

## Analytics

Responsible for:

- match statistics
- gameplay telemetry
- replay analysis

---

# Known Temporary Constraints

The current implementation includes several deliberate simplifications.

These are documented in ADRs where appropriate.

Current constraints include:

- matches support exactly two players
- opening hand size is fixed to 7
- deck contents are provided externally
- shuffle behavior is minimal
- creature damage tracking in combat (without destruction)
- stack interactions are not modeled
- only a minimal subset of card behavior exists

These constraints will evolve as new slices are introduced.

---

# Guidance for Future Changes

When extending the aggregate:

Prefer:

- incremental modeling through vertical slices
- explicit command handling
- explicit domain events
- deterministic behavior

Avoid:

- introducing generic rule engines prematurely
- modeling unused zones or mechanics
- leaking infrastructure concerns into the domain
- speculative abstractions without active behavior

---

# Internal Implementation Guidance

The `Game` aggregate remains a single aggregate root. Its internal implementation may be organized into modules by domain capability to improve readability and maintainability.

This means:

- `Game` continues to be the **aggregate root**
- implementation may be divided into **internal modules** by behavior
- dividing code into modules does **not create new aggregates**
- modularization should follow **domain capabilities**, not generic utilities

Example structure (guideline, not requirement):

```
src/domain/play/game/
├── mod.rs
├── invariants.rs   # aggregate legality checks and internal lookups
├── model/
│   ├── mod.rs
│   └── player.rs   # aggregate-owned entity internals
└── rules/
    ├── mod.rs
    ├── lifecycle.rs        # start game, opening hands, mulligan
    ├── turn_flow.rs        # phases, draws, turn progression
    ├── resource_actions.rs # lands, mana, spells, creatures, life
    └── combat.rs
```

This organization keeps the aggregate cohesive while avoiding monolithic files.
