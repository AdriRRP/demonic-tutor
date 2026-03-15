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
- drawing cards
- playing lands
- tapping lands for mana
- casting spells that require mana
- tracking player life totals
- advancing turns
- basic phase progression

These capabilities correspond to the slices currently implemented in the system.

---

# Current Domain Scope

The system currently models a **minimal subset of Magic gameplay** focused on playtesting.

The domain currently includes:

- game sessions
- players
- card instances
- basic zones (library, hand, battlefield)
- mana production from lands
- spell casting with mana cost
- turn and phase progression

The system intentionally excludes complex gameplay mechanics at this stage.

---

# Known Constraints

The current implementation includes several deliberate simplifications.

These constraints allow the system to evolve safely through vertical slices.

Current constraints include:

- matches support exactly two players
- opening hand size is fixed to 7 cards
- only a subset of zones are modeled
- no stack resolution
- no priority system
- no combat system
- limited card behavior modeling

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

- a core `Game` aggregate
- command-driven gameplay operations
- domain events describing state transitions
- an event bus for event distribution
- projections derived from gameplay events

This architecture supports:

- replayability
- observability
- deterministic state transitions

---

# Next Modeling Decision

The next gameplay expansion requires choosing which domain capability to introduce next.

Possible directions include:

- creature power/toughness modeling
- combat system
- declare attackers step
- stack and priority system

The next slice should continue expanding gameplay behavior incrementally.

---

# Guiding Principle

The system should evolve through **small, deterministic vertical slices**.

Each slice should:

- introduce one new gameplay capability
- extend the domain model minimally
- emit explicit domain events
- preserve existing invariants
