# Domain Glossary — DemonicTutor

This glossary defines the initial ubiquitous language of the project.

Its purpose is to establish precise shared terms before deeper domain modeling begins.

## Game

A running play session with players, zones, turn progression, legal actions and event history.

A game is not just a screen state.
It is a domain concept with rules, transitions and invariants.

## Player

A participant in a game.

A player may own a deck, hold cards in hand, control permanents and perform legal actions when allowed by game state.

## Deck

A player-owned list of cards used as the source for the library and related game setup operations.

A deck is not the same thing as a running game state.

## CardDefinition

The conceptual identity of a card as defined by its characteristics and rules meaning.

This is the card as a known game object in abstraction.

## CardInstance

A concrete occurrence of a card inside a specific game session.

It is useful to distinguish a card definition from a specific copy of that card inside a game.

## Library

A zone representing the draw pile of a player.

## Hand

A zone containing cards currently available to a player for potential play or other actions.

## Battlefield

A zone containing permanents currently in play.

## Graveyard

A zone containing cards that have been used, destroyed, discarded or otherwise moved there according to rules and effects.

## Exile

A zone containing cards that have been moved outside normal battlefield and graveyard circulation.

## Stack

A zone or conceptual area where certain spells and abilities wait to resolve.

Full support may come later, but the concept is part of the language.

## Zone

A logical game area capable of containing card instances.

Initial relevant zones are:
- library
- hand
- battlefield
- graveyard
- exile
- stack

## Turn

A numbered unit of progression in the game.

A turn structures the flow of legal actions and game progression.

## Phase

A sub-division of a turn.

Not all phases need to be modeled immediately, but the concept must exist explicitly.

## Priority

The right of a player to take an action at a given moment.

Priority is part of action legality and should not be treated as a UI detail.

## Command

A request expressing player or system intent in domain terms.

A command asks the model to try to perform something.
It is not a fact.

## Event

A domain fact representing something that has already happened.

Events are useful for traceability, replayability and analytics.

## Event Store

A persistence mechanism for domain events.

Its purpose is to retain event history so that game sessions can be reconstructed or analyzed.

## Event Bus

An application-layer mechanism for distributing domain events to interested handlers.

The event bus is not part of the aggregate itself.

## Projection

A read-oriented model derived from domain events.

Projections are useful for statistics, summaries, timelines and other observable views.

## Replay

A reconstruction of a game from persisted events.

Replayability is one of the intended strengths of the system.

## Invariant

A rule that must always hold for the domain model to remain valid.

Invariants protect correctness.

## Vertical Slice

A minimal end-to-end implementation of one coherent, testable and user-visible behavior.

Vertical slices are the preferred way to evolve the system.

## Rules Support

The subset of official Magic rules currently represented by the implementation.

Rules support must always be explicit.
The project must not imply broader support than it actually implements.
