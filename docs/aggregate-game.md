# Game Aggregate — DemonicTutor

## Purpose

This document describes the current structure, responsibilities and evolution of the `Game` aggregate in DemonicTutor.

It serves as a reference for:

* understanding the current domain model
* documenting which responsibilities belong to the aggregate
* clarifying what has been implemented in each slice
* preventing accidental overreach when adding new behavior

---

# Aggregate Overview

`Game` is the central aggregate of the **play** bounded context.

It represents a single running playtest session between players.

The aggregate is responsible for:

* maintaining the authoritative state of the match
* enforcing gameplay invariants
* applying commands
* emitting domain events that describe state transitions

External systems must interact with the game **only through commands**.

---

# Current Aggregate Structure

At the current stage (after Slice 13), the aggregate structure is conceptually:

Game
├── id
├── active_player
├── phase
├── turn_number
└── players
    ├── id
    ├── deck_id
    ├── library
    ├── hand
    ├── battlefield
    ├── life
    ├── mana
    ├── lands_played_this_turn
    └── mulligan_used

### Game

Represents a running match.

Responsibilities:

* create a new match
* coordinate operations that affect multiple players
* validate player references
* emit events describing match state changes

The aggregate root ensures that all operations maintain valid game state.

---

### Player

Represents a participant in the match.

Each player owns their personal game zones.

Responsibilities:

* hold references to deck identity
* manage personal card zones (library, hand, battlefield)
* track per-turn state (lands played, mana)
* track life total
* expose state needed for gameplay actions

Players are not aggregates themselves; they are **entities contained within `Game`**.

---

### Library

Represents a player's draw pile.

Current implementation:

* simple ordered collection of `CardInstance`
* supports drawing cards via `draw(n)`

Responsibilities:

* provide cards when drawn
* maintain card ordering
* enforce draw availability

Not yet responsible for:

* shuffle rules
* deck legality
* format validation

---

### Hand

Represents the cards currently held by a player.

Current implementation:

* simple collection of `CardInstance`

Responsibilities:

* receive cards drawn from library
* expose cards available to the player

---

### Battlefield

Represents the permanents currently in play.

Current implementation:

* simple collection of `CardInstance` (lands)

Responsibilities:

* receive cards played from hand
* expose battlefield contents

---

### CardInstance

Represents a concrete instance of a card inside a match.

Fields:

* `CardInstanceId`
* `CardDefinitionId`
* `CardType` (Land, Creature, Instant, Sorcery, Enchantment, Artifact, Planeswalker)
* `tapped` (bool)
* `mana_cost` (u32)

Responsibilities:

* uniquely identify cards within a match
* reference a card definition
* distinguish card types
* track tapped state
* track mana cost

Not yet responsible for:

* rules text
* abilities
* power/toughness
* damage
* complex card state

---

# Slice Evolution

Each slice is documented in detail in `docs/slices/`.

## Slice 1 — StartGame

Initialize a valid game with exactly two players.

**Commands**: `StartGameCommand`  
**Events**: `GameStarted`

---

## Slice 2 — DrawOpeningHands

Assign opening hands to all players.

**Commands**: `DealOpeningHandsCommand`  
**Events**: `OpeningHandDealt`

---

## Slice 3 — PlayLand

Play a land from hand to battlefield.

**Commands**: `PlayLandCommand`  
**Events**: `LandPlayed`

---

## Slice 4 — AdvanceTurn

Advance to the next player's turn.

**Commands**: `AdvanceTurnCommand`  
**Events**: `TurnAdvanced`

---

## Slice 5 — DrawCard

Draw a card from library to hand.

**Commands**: `DrawCardCommand`  
**Events**: `CardDrawn`

---

## Slice 6 — Mulligan

Perform a mulligan by returning hand to library, shuffling, and drawing a new 7-card hand.

**Commands**: `MulliganCommand`  
**Events**: `MulliganTaken`

---

## Slice 7 — Infrastructure

Add event store, event bus, and projections.

**Components**: `EventStore`, `EventBus`, `GameLogProjection`

---

## Slice 8 — Player Life

Add player life tracking.

**Commands**: `SetLifeCommand`  
**Events**: `LifeChanged`

---

## Slice 9 — Turn Number

Track turn count.

**Events**: `TurnNumberChanged`

---

## Slice 10 — Turn Phases

Add proper phase structure (Setup, Beginning, Main, Ending).

**Events**: `PhaseChanged`

---

## Slice 11 — Tap Lands for Mana

Add mana production from lands.

**Commands**: `TapLandCommand`  
**Events**: `LandTapped`, `ManaAdded`

---

## Slice 12 — Cast Non-Land Spells

Enable casting non-land spells from hand to battlefield.

**Commands**: `CastSpellCommand`  
**Events**: `SpellCast`

---

## Slice 13 — Pay Mana Cost

Require mana payment for casting spells.

**Commands**: `CastSpellCommand` (now checks mana cost)  
**Errors**: `InsufficientMana`

---

# Invariants Maintained by the Aggregate

The `Game` aggregate currently guarantees:

* exactly two players exist in a match
* players are uniquely identified
* libraries contain card instances owned by that player
* cards cannot be drawn if not available
* opening hand assignment is atomic

---

# Responsibilities of the Aggregate

The aggregate root must enforce:

* player existence
* player uniqueness
* card movement correctness
* consistency between zones
* event emission for state changes

The aggregate **must not** handle:

* UI concerns
* persistence
* analytics
* external deck loading
* rule execution engines

---

# Responsibilities Outside the Aggregate

Some responsibilities belong to other contexts or future slices.

These include:

Deck context

* deck building
* deck legality
* deck storage

Rules engine

* card abilities
* stack resolution
* triggered effects

Infrastructure

* persistence
* event store
* event bus
* projections

Analytics

* match statistics
* gameplay telemetry

---

# Known Temporary Decisions

The current implementation includes several intentionally temporary constraints.

These decisions are documented in ADRs.

Current temporary rules include:

* matches support exactly two players
* opening hand size is fixed to 7
* deck contents are provided externally
* shuffle behavior is not yet configurable
* phase transitions: Setup → Beginning → Main → Ending → Main (next player)
* spells cast for free (no mana cost)
* no power/toughness on creatures

These constraints will likely evolve in later slices.

---

# Guidance for Future Changes

When extending the aggregate:

Prefer:

* adding behavior incrementally through slices
* keeping operations deterministic
* emitting explicit domain events

Avoid:

* introducing generic rule engines prematurely
* modeling unused game zones
* adding abstractions without active behavior
* leaking infrastructure concerns into the domain

