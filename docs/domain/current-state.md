# Current State — DemonicTutor

This document provides a snapshot of the current capabilities of the system.

It summarizes what parts of the domain are implemented and what areas remain intentionally incomplete.

Detailed slice documentation lives in:

docs/slices/

---

# Implemented Gameplay Capabilities

The current implementation supports a minimal playable flow for deck playtesting.

Implemented capabilities include:

- starting a two-player game
- dealing opening hands
- mulligan support (London Mulligan - simplified)
- drawing cards (auto-draw in Draw phase)
- resolving explicit draw effects during main phases
- playing lands
- tapping lands for mana
- casting non-land spells that require mana
- casting creature spells that enter the battlefield with power and toughness
- resolving instants and sorceries to graveyard
- summoning sickness for creatures (removed for the active player's battlefield at turn start)
- declaring attackers in combat phase
- declaring blockers in combat phase
- resolving combat damage
- destroying creatures automatically when marked combat damage is lethal
- tracking player life totals
- advancing turns
- full phase progression using State pattern (Setup, Untap, Upkeep, Draw, FirstMain, Combat, SecondMain, EndStep)

These capabilities correspond to the slices currently implemented in the system.

---

# Current Domain Scope

The system currently models a **minimal subset of Magic gameplay** focused on playtesting.

The domain currently includes:

- game sessions
- players
- card instances
- basic zones (library, hand, battlefield, graveyard)
- mana production from lands
- non-land spell casting with mana cost
- transient mana pools cleared when the game advances to the next phase or turn
- creature cards with power and toughness
- creature spells entering the battlefield through `CastSpell`
- creature damage tracking during combat
- automatic destruction of creatures with lethal marked damage
- summoning sickness for creatures (removed for the active player's creatures at turn start)
- turn and phase progression
- explicit draw effects as a simplified non-stack entrypoint

The system intentionally excludes complex gameplay mechanics at this stage.

---

# Known Constraints

The current implementation includes several deliberate simplifications.

These constraints allow the system to evolve safely through vertical slices.

Current constraints include:

- matches support exactly two players
- opening hand size is fixed to 7 cards
- only a subset of zones are modeled (no exile or stack zone behavior)
- no stack resolution
- no priority system
- no triggered abilities
- limited card behavior modeling
- non-land permanents currently enter the battlefield through simplified spell resolution without stack handling
- mana production is simplified to active-player main phases and generic mana only

These constraints are expected to evolve in future slices.

---

# Infrastructure State

The system currently runs entirely in-memory.

Infrastructure components include:

- in-memory event store
- in-memory event bus
- projections for gameplay logs

Persistent infrastructure may be introduced in future iterations.

---

# Architectural Status

The project currently includes:

- a core `Game` aggregate with centralized player access
- command-driven gameplay operations
- play-owned library initialization data for opening hands
- type-safe library initialization data with distinct creature and non-creature variants
- domain events describing state transitions
- composite turn progression events and draw events with explicit origin
- an event bus for event distribution
- projections derived from gameplay events
- State pattern for phase transitions
- helper methods for event persistence and publishing
- a Gherkin feature pilot, with executable turn-progression acceptance coverage via `cucumber-rs`

This architecture supports:

- replayability
- observability
- deterministic state transitions
- State pattern for phase behavior encapsulation

---

# Next Modeling Decision

The next gameplay expansion requires choosing which domain capability to introduce next.

Possible directions include:

- cleanup-based damage removal
- stack and priority system
- broader state-based actions beyond lethal creature damage

The next slice should continue expanding gameplay behavior incrementally.

---

# Guiding Principle

The system should evolve through **small, deterministic vertical slices**.

Each slice should:

- introduce one new gameplay capability
- extend the domain model minimally
- emit explicit domain events
- preserve existing invariants
