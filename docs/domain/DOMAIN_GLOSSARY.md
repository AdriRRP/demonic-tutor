# Domain Glossary — DemonicTutor

This glossary defines the ubiquitous language used in the DemonicTutor domain.

Its purpose is to ensure that the same terms always mean the same concepts across:

- domain model
- documentation
- code
- discussions

Only domain concepts are included here.

---

# Core Concepts

## Game

A running play session between players.

A game maintains state such as players, zones, turn progression and legal actions.

The game enforces gameplay invariants and produces domain events describing state transitions.

---

## Player

A participant in a game.

A player owns cards in various zones and may perform actions when allowed by the game state.

---

## Deck

A predefined list of cards used to initialize a player's library.

Decks are defined outside gameplay and are not modified by the game.

---

# Cards

## CardDefinition

The conceptual identity of a card.

A card definition describes what a card *is* independently of any specific game.

Examples include card name, type and characteristics.

---

## CardInstance

A concrete copy of a card inside a specific game session.

Multiple card instances may reference the same card definition.

Instances track runtime state such as location or tapped status.

---

# Zones

## Zone

A logical area of the game capable of containing card instances.

Zones define where cards exist during gameplay.

---

## Library

A player's draw pile.

Cards are drawn from the library into the hand.

---

## Hand

A zone containing cards currently available to a player.

Cards in hand may potentially be played or cast.

---

## Battlefield

A zone containing permanents currently in play.

Cards entering the battlefield become part of the active game state.

---

## Graveyard

A zone containing cards that have been used, destroyed or discarded.

---

## Maximum Hand Size

The maximum number of cards a player may keep in hand when the turn ends.

In the current DemonicTutor model, this maximum is 7 and cleanup discard is handled explicitly during `EndStep`.

---

## Discard

To move a card from a player's hand to their graveyard.

In the current runtime model, discard is supported only as explicit cleanup behavior when the active player is above the maximum hand size at end of turn.

---

## Exile

A zone containing cards removed from normal gameplay circulation.

---

## Stack

A conceptual zone where spells and abilities wait to resolve.

Full support for stack behavior may be introduced in future slices.

---

# Card Types

## Creature

A card type representing a permanent that can attack and block.

Creatures have power and toughness characteristics.

When a creature enters the battlefield, it has summoning sickness and cannot attack or use tap abilities until the next turn.

---

## Land

A card type representing a permanent that produces mana.

Currently the only card type that can be played without mana cost.

---

## Permanent

A card or token that exists on the battlefield. Permanents include lands, creatures, enchantments, artifacts, and planeswalkers. They remain in play until removed by an effect.

---

## Spell

A card on the stack.

In the current runtime model, spell cards are cast through `CastSpell` onto an explicit stack. Permanent spells resolve from the stack to the battlefield, while instants and sorceries resolve from the stack to the graveyard. The current minimal stack slice also supports instant-speed spell responses while a priority window is open.

---

# Creature-Specific Terms

## Power

The amount of damage a creature deals in combat.

---

## Toughness

The amount of damage required to destroy a creature.

---

## Summoning Sickness

A state applied to creatures that have just entered the battlefield.

A creature with summoning sickness cannot attack or use abilities with tap in their cost.

Summoning sickness is removed at the beginning of the creature's controller's next turn.

---

## Damage

A value assigned to creatures during combat or from sources with damage effects.

Damage is marked on a creature and persists until end of turn or regeneration.

---

## Dies

A creature dies when it is put into its owner's graveyard from the battlefield.

In DemonicTutor, `CreatureDied` is the domain event used when a creature leaves the battlefield for its owner's graveyard as an automatic gameplay consequence, including lethal marked damage and zero toughness.

---

# Gameplay Structure

## Turn

A numbered unit of progression in the game.

Turns structure the order in which players act.

---

## Phase

A subdivision of a turn.

Phases organize gameplay into distinct stages.

---

## Priority

The right of a player to take an action at a specific moment.

Priority determines which player may act and when.

Full priority rules may be introduced incrementally.

---

# Actions and Facts

## Command

A request expressing intent to perform an action in the domain.

Commands ask the model to attempt an operation.

They may succeed or fail depending on the current state.

---

## Event

A domain fact representing something that has already happened.

Events describe state transitions and are immutable.

---

## Game End

A terminal match state in which the game has produced a winner, a loser, and an end reason.

While the game is in a terminal state, normal gameplay actions are no longer legal.

---

## Lose the Game

To reach a terminal state as the losing player of the match.

In the current DemonicTutor model, this happens when a player must draw from an empty library.
In the current DemonicTutor model, this also happens when a player's life total reaches 0.

---

# Domain Integrity

## Invariant

A rule that must always hold for the domain model to remain valid.

Invariants protect the correctness of the game state.
