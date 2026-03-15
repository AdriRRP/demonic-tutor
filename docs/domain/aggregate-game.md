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

- receive cards played or cast
- expose battlefield state

Current implementation:

- collection of `CardInstance`

The battlefield currently models only a minimal subset of permanent state.

---

## CardInstance

Represents a concrete instance of a card inside a match.

Fields include:

- `CardInstanceId`
- `CardDefinitionId`
- `CardType`
- tapped state
- mana cost

Responsibilities:

- uniquely identify cards within a match
- reference card definitions
- track minimal runtime state required for gameplay

The current model intentionally omits:

- rules text
- triggered abilities
- power/toughness
- damage tracking
- stack interactions

These may be introduced incrementally in future slices.

---

# Aggregate Invariants

The `Game` aggregate currently guarantees:

- exactly two players exist in a match
- players are uniquely identified
- card instances belong to exactly one player
- cards cannot be drawn if not available
- card movements maintain zone consistency
- gameplay operations emit domain events

These invariants are enforced whenever commands are applied.

---

# Responsibilities of the Aggregate

The aggregate root must enforce:

- player existence
- player uniqueness
- valid card movement between zones
- turn progression rules
- phase progression rules
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
- creature combat is not modeled
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
