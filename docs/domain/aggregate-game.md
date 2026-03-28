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
- stack zone state
- aggregate card location index
- optional priority state
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

- hold runtime-owned gameplay state and player identity
- manage personal card zones
- own the canonical runtime storage for that player's card instances
- track life total
- track per-turn state
- expose state required for gameplay operations

Players are entities contained within the aggregate.

Deck-oriented setup identifiers are translated at the aggregate boundary and are not stored as part of the player's runtime gameplay state.

---

## Library

Represents a player's draw pile.

Current implementation:

- ordered collection backed by player-owned runtime storage and internal card handles

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

- ordered visible view over player-owned card handles backed by indexed ordered-zone storage

The current runtime preserves the order in which cards appear in hand.
Rules and tests should prefer semantic player and zone queries over direct dependence on the hand's backing storage.

---

## Battlefield

Represents permanents currently in play.

Responsibilities:

- receive permanents played or cast
- expose battlefield state

Current implementation:

- collection of player-owned card handles backed by a player-owned card arena

The battlefield currently models only a minimal subset of permanent state.
The runtime does not currently promise stable battlefield ordering.

---

## Graveyard

Represents cards that have resolved or otherwise left active play.

Responsibilities:

- receive instants and sorceries after simplified resolution
- preserve card history once they leave the active battlefield model

Current implementation:

- ordered visible view over player-owned card handles backed by indexed ordered-zone storage

The current runtime preserves graveyard insertion order.
Rules and tests should prefer semantic player and zone queries over direct dependence on graveyard storage details.

---

## Exile

Represents cards removed from normal gameplay circulation.

Responsibilities:

- receive cards exiled from battlefield or graveyard
- expose exiled cards for gameplay inspection

Current implementation:

- ordered visible view over player-owned card handles backed by indexed ordered-zone storage

The current runtime preserves exile insertion order.
Rules and tests should prefer semantic player and zone queries over direct dependence on exile storage details.

---

## CardInstance

Represents a concrete instance of a card inside a match.

The current runtime model separates:

- immutable card-face data (`definition id`, `type`, `mana cost`, optional spell-casting permissions`)
- immutable supported targeting and resolution profiles for the card's currently modeled behavior
- mutable gameplay state (`tapped`, combat flags, creature runtime state, optional attached target id`)

Immutable card-face metadata is currently shared across instances rather than copied by value per zone entry.
Internal runtime identity now prefers numeric-core ids, player indices, and player-owned card handles, while readable public ids are materialized at the aggregate boundary.
Card instances also retain persistent owner identity even when the supported runtime temporarily places them on another player's battlefield to represent current control.

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
- blocking target (for blocking creatures)
- ordered blockers assigned against the attacker (for attacking creatures)
- combat-damage assignment following that ordered blocker group in the supported subset
- damage marked on the creature
- attached target card id for the current supported attachment subset

Responsibilities:

- uniquely identify cards within a match
- reference card definitions
- track minimal runtime state required for gameplay

The current model includes:

- power and toughness for creature cards
- a closed set of creature keyword abilities for the currently supported combat subset
- summoning sickness tracking and automatic removal at turn start
- declare attackers and blockers in combat phase
- marked combat damage on creatures
- automatic destruction of creatures with lethal marked damage
- automatic destruction of creatures with 0 toughness through the shared review of supported state-based actions
- minimal stack-aware spell casting and spell resolution
- the stack may currently hold supported spells plus the current supported non-mana activated ability object family
- supported planeswalkers may now carry initial loyalty and explicit loyalty-ability profiles in the same aggregate-owned activation corridor
- the stack and supported activated abilities prefer internal owner/handle references while public ids are materialized only for outward-facing events, errors, and inspection
- explicit supported spell-effect profiles carried by card-face data rather than inferred from card-definition strings during resolution
- explicit legal-target rules for the current supported targeted-spell subset, currently covering a small closed set of players, creatures, permanents, graveyard cards, and stack objects
- spell-target and spell-resolution metadata can be projected into dedicated stack-borne spell snapshots instead of reusing full permanent runtime
- non-mana activated abilities may ride the same aggregate-owned stack while mana abilities remain outside it, and the current activation-cost subset now includes tapping, explicit mana payment, and `sacrifice this source`
- casting player retaining priority immediately after putting a spell on the stack
- instant responses by the current priority holder in the currently supported windows
- opening an empty priority window when entering `Upkeep`
- opening an empty priority window when entering `Draw` after the automatic turn draw
- opening empty priority windows when entering `FirstMain` and `SecondMain`
- opening an empty priority window when entering `EndStep`
- opening an empty priority window when entering `BeginningOfCombat`
- moving combat state through `DeclareAttackers`, `DeclareBlockers`, `CombatDamage`, and `EndOfCombat`
- reopening priority after attackers and blockers are declared in combat
- reopening priority after combat damage resolves when the game remains active in `EndOfCombat`
- active-player self-stacking of multiple instants in the currently supported stack windows
- rejection of turn advancement while a priority window is open
- explicit exile from battlefield or graveyard into a player-owned exile zone

Card instances can be checked for whether they represent permanents (cards that can exist on the battlefield) using the `CardType::is_permanent()` method.

The current model intentionally omits:

- rules text
- most counter families beyond the current explicit `+1/+1` counter subset
- non-instant spell responses while a priority window is open

These may be introduced incrementally in future slices.

The current triggered-ability support is intentionally tiny and explicit:

- enter-the-battlefield triggers on supported permanents
- dies triggers on supported creatures
- beginning-of-upkeep triggers on supported battlefield permanents across all controllers
- beginning-of-end-step triggers on supported battlefield permanents across all controllers
- the current supported triggered effects are still explicit profiles, currently limited to life gain, optional life gain, and one bounded end-step spell-recursion effect

This is not yet a generic trigger engine for arbitrary delayed, intervening-if, or multi-object ordering cases.

---

# Aggregate Invariants

The `Game` aggregate currently guarantees:

- exactly two players exist in a match
- players are uniquely identified
- card instances belong to exactly one player
- cards cannot be drawn if not available
- the game ends if a required draw cannot happen because the relevant library is empty
- the game ends if a player's life total reaches 0
- creatures with 0 toughness die when the runtime performs its current zero-toughness check
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
- legal blocking constraints for the currently supported `Flying` and `Reach` keyword subset
- creature spell validation including power/toughness presence before battlefield entry
- turn progression rules
- phase progression rules
- active-player-only automatic turn updates
- terminal game tracking for empty-library draw and zero-life loss
- ownership of stack and priority state
- minimal priority-window legality for currently supported stack interactions
- explicit spell-target validation for the currently supported targeted-spell subset
- zero-toughness creature death after current creature-spell resolution checks
- lethal-damage creature destruction after combat damage resolution
- correct event emission

The aggregate must remain:

- deterministic
- infrastructure-free
- explicit in its state transitions

---

# Responsibilities Outside the Aggregate

Several concerns intentionally live outside the aggregate.

## Deck Input Contract

The aggregate consumes deck-oriented setup input but does not own a separate deck model.

Deck metadata such as `DeckId` currently exists only in setup-oriented input. Once the match starts, the aggregate retains only play-owned runtime state.

Responsible outside the aggregate:

- deck construction
- deck legality
- deck persistence

For the first curated limited environment, the current baseline for those responsibilities is documented in:

- [limited-set-deck-construction-baseline.md](limited-set-deck-construction-baseline.md)

---

## Future Play Expansion

Future slices inside the `play` context are expected to introduce:

- broader stack and priority behavior
- priority passing
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

## Projections / Analytics Read Side

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
- combat now uses explicit subphases, but remains intentionally simplified compared with full rules-complete combat-step modeling
- stack interactions are intentionally limited to the current minimal spell-and-priority slices
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
├── invariants.rs   # pure aggregate legality checks
├── helpers.rs      # state-modifying lookups and internal helpers
├── model/
│   ├── mod.rs
│   ├── player.rs         # aggregate-owned entity internals
│   ├── priority.rs
│   ├── stack.rs
│   └── terminal_state.rs # game outcome state
└── rules/
    ├── mod.rs
    ├── lifecycle.rs        # start game, opening hands, mulligan
    ├── game_effects.rs     # direct life and game-end helpers reused by rules
    ├── resource_actions.rs # lands, mana, spells, creatures, life
    ├── state_based_actions.rs # shared review of supported state-based actions
    ├── combat/              # declaration, blocking legality, damage, progression
    ├── stack_priority/     # casting, passing, resolution, spell effects
    └── turn_flow/          # phases, turn progression, draw effects, cleanup
```

This organization keeps the aggregate cohesive while avoiding monolithic files.
