# Vertical Slices — DemonicTutor

## Purpose

This document defines the first narrow end-to-end behaviors that should guide the early evolution of DemonicTutor.

Its purpose is to keep development incremental, testable and aligned with project constraints.

A vertical slice is a minimal end-to-end implementation of one coherent and observable behavior.

The system should evolve through small slices rather than broad speculative scaffolding.

---

# Slice 1 — StartGame

## Goal

Initialize a new game session from valid player and deck inputs.

## Why it matters

This slice establishes the first executable path across the system.

It forces the project to define:

* core identifiers
* game creation flow
* initial aggregate lifecycle
* first command
* first event
* first application orchestration

It is the best possible starting point because it creates a running domain object without requiring complex gameplay rules.

## Expected behavior

The system should be able to:

* receive a request to start a game
* create a new game session
* associate players with decks
* produce a `GameStarted` domain event
* persist that event
* publish that event
* expose a basic read model confirming that the game exists

## Domain concepts

* Game
* GameId
* PlayerId
* DeckId
* StartGameCommand
* GameStarted

## Out of scope

This slice does not yet require:

* card drawing
* mulligan handling
* turn progression
* phase progression
* action legality beyond basic creation
* card-specific rules

---

# Slice 2 — DrawOpeningHand

## Goal

Allow a started game to assign opening hands to players.

## Why it matters

This slice introduces controlled state evolution after game creation.

It also introduces the first meaningful zone transition from library to hand.

This helps validate:

* zone modeling
* deterministic setup behavior
* event sequencing after game start

## Expected behavior

The system should be able to:

* draw an opening hand for each player
* place card instances into player hands
* emit one or more domain events describing the draw
* persist and publish those events
* expose a projection showing opening hand size or draw summary

## Domain concepts

* Library
* Hand
* CardInstance
* CardDefinitionId
* CardInstanceId
* OpeningHandDealt
* DealOpeningHandsCommand

## Out of scope

This slice does not yet require:

* mulligan decisions
* priority
* stack handling
* card text execution
* full turn structure

---

# Slice 3 — PlayLand

## Goal

Allow a player to play a land during a legal point of the turn under simplified rules.

## Why it matters

This slice introduces the first clearly player-driven gameplay action with legality checks.

It begins the transition from setup behavior to actual game behavior.

It forces the model to represent:

* turn ownership
* at least one phase
* hand-to-battlefield movement
* simple legality validation

## Expected behavior

The system should be able to:

* receive a command to play a land
* verify that the action is legal in the current state
* move the selected card instance from hand to battlefield
* emit a domain event representing the action
* reject the command if the action is illegal

## Domain concepts

* Phase
* Battlefield
* LandPlayed
* CardType

## Out of scope

This slice does not yet require:

* stack usage for spells
* triggered abilities
* replacement effects
* complete phase model
* multiple land-rule edge cases unless explicitly needed

---

# Slice 4 — AdvanceTurn

## Goal

Advance the game to the next player's turn using a minimal turn model.

## Why it matters

This slice enables turn progression between the two players.

It resets per-turn state and enables continued gameplay.

## Expected behavior

The system should be able to:

* accept an `AdvanceTurnCommand`
* change the active player
* reset the phase to `Phase::Main`
* reset land-play counters
* emit a `TurnAdvanced` event

## Domain concepts

* Turn advancement
* Active player tracking

## Out of scope

This slice does not yet require:

* full turn structure
* draw step
* upkeep
* combat
* priority
* stack
* automatic triggers
* multiplayer turn-order generalization

---

# Slice 5 — DrawCard

## Goal

Allow the active player to draw exactly one card from their library into their hand.

## Why it matters

This slice introduces the first explicit card-drawing action.

It validates phase restrictions and library availability.

## Expected behavior

The system should be able to:

* accept a `DrawCardCommand`
* verify the player is active
* verify the phase allows drawing
* move one card from library to hand
* emit a `CardDrawn` event
* reject if library is empty

## Domain concepts

* Card drawing
* Phase validation

## Out of scope

This slice does not yet require:

* automatic draw step
* drawing multiple cards
* decking / losing from empty library
* replacement effects
* priority
* stack

---

# Ordering Policy

The slices must be implemented in order.

The project must not skip ahead to richer gameplay before the previous slice is coherent and tested.

---

# Modeling Policy

Each slice should introduce only the minimum rule subset required for its behavior.

No slice should imply broader rules support than what is explicitly implemented.

---

# Testing Policy

Each slice should eventually include:

* domain tests
* application-level tests
* at least one observable scenario if useful

---

# Agent Guidance

When proposing code or architecture, the agent must respect the current active slice.

It must not introduce concepts required only for later slices unless they are strictly necessary for correctness.
