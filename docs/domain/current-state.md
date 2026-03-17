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
- drawing cards (auto-draw when entering `Draw`)
- resolving explicit draw effects during main phases
- resolving explicit draw effects that draw multiple cards one by one
- ending the game when a player must draw from an empty library
- ending the game when a player reaches 0 life
- playing lands
- tapping lands for mana
- casting spells that require mana
- casting creature spells that enter the battlefield with power and toughness
- resolving instants and sorceries to graveyard
- summoning sickness for creatures (removed for the active player's battlefield at turn start)
- declaring attackers in `DeclareAttackers`
- declaring blockers in `DeclareBlockers`
- blocking currently supports at most one blocker per attacking creature
- opening priority windows across `Upkeep`, `Draw`, `FirstMain`, `BeginningOfCombat`, `EndOfCombat`, `SecondMain`, and `EndStep`
- opening a priority window when entering `BeginningOfCombat`
- opening priority windows after attackers and blockers are declared
- reopening priority after combat damage resolves while the game remains active in `EndOfCombat`
- allowing instant responses and active-player self-stacking in the currently supported stack windows
- resolving combat damage
- applying unblocked combat damage to players through shared life-change semantics
- destroying creatures automatically when marked combat damage is lethal
- destroying creatures with 0 toughness automatically after creature-spell resolution
- clearing marked damage from surviving creatures when the turn ends
- discarding down to the maximum hand size before the turn can advance out of `EndStep`
- tracking player life totals
- advancing turns
- full phase progression using State pattern (Setup, Untap, Upkeep, Draw, FirstMain, BeginningOfCombat, DeclareAttackers, DeclareBlockers, CombatDamage, EndOfCombat, SecondMain, EndStep)

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
- spell casting with mana cost
- transient mana pools cleared when the game advances to the next phase or turn
- creature cards with power and toughness
- creature spells entering the battlefield through `CastSpell`
- creature damage tracking during combat
- single-blocker combat assignments for each attacking creature
- player life changes from combat damage
- automatic destruction of creatures with lethal marked damage
- automatic destruction of creatures with 0 toughness after creature-spell resolution
- shared state-based action review after relevant gameplay actions for the currently supported SBA subset
- cleanup-based removal of marked damage from surviving creatures
- explicit cleanup discard to maximum hand size during `EndStep`
- summoning sickness for creatures (removed for the active player's creatures at turn start)
- turn and phase progression
- explicit draw effects as a simplified non-stack entrypoint, including multi-card draw
- terminal game state when a player loses by empty-library draw or zero life
- casting spells onto an aggregate-owned stack zone
- public priority passing for the currently open minimal stack windows
- the casting player retains priority immediately after a spell is put on the stack
- the responding player may cast a second instant while retaining priority on an existing stack
- entering `Upkeep` opens an empty priority window for the active player
- the active player may cast and resolve an instant during that upkeep priority window
- the active player may cast a second instant in `Upkeep` before passing priority after the first
- the non-active player may cast and resolve an instant in `Upkeep` after the active player passes
- the non-active player may cast a second instant in `Upkeep` before passing priority after the first response
- entering `Draw` opens an empty priority window for the active player after the automatic turn draw
- the active player may cast and resolve an instant during that draw-step priority window
- the active player may cast a second instant in `Draw` before passing priority after the first
- the non-active player may cast and resolve an instant in `Draw` after the active player passes
- the non-active player may cast a second instant in `Draw` before passing priority after the first response
- entering `FirstMain` or `SecondMain` opens an empty priority window for the active player
- the active player may cast and resolve an instant during that first-main priority window
- the active player may cast a second instant in `FirstMain` before passing priority after the first
- the non-active player may cast and resolve an instant in `FirstMain` after the active player passes
- the non-active player may cast a second instant in `FirstMain` before passing priority after the first response
- the active player may cast and resolve an instant during that second-main priority window
- the active player may cast a second instant in `SecondMain` before passing priority after the first
- the non-active player may cast and resolve an instant in `SecondMain` after the active player passes
- the non-active player may cast a second instant in `SecondMain` before passing priority after the first response
- entering `EndStep` opens an empty priority window for the active player before cleanup can finish the turn
- the non-active player may cast and resolve an instant in `EndStep` after the active player passes
- the non-active player may cast a second instant in `EndStep` before passing priority after the first response
- the active player may cast and resolve an instant during that end-step priority window
- the active player may cast a second instant in `EndStep` before passing priority after the first
- instant-speed spell responses for the current priority holder
- resolving the top stack object after two consecutive passes
- entering `BeginningOfCombat` opens an empty priority window for the active player
- closing that empty beginning-of-combat window advances the game into `DeclareAttackers`
- an empty `DeclareAttackers` step may advance into `DeclareBlockers`
- an empty `DeclareBlockers` step may advance into `CombatDamage`
- an empty `CombatDamage` step may advance into `EndOfCombat`
- closing the empty `EndOfCombat` window advances the game into `SecondMain`
- the non-active player may cast and resolve an instant at the beginning of `Combat` after the active player passes
- the non-active player may cast a second instant at the beginning of `Combat` before passing priority after the first response
- the active player may cast and resolve an instant at the beginning of `Combat`
- the active player may cast a second instant at the beginning of `Combat` before passing priority after the first
- combat actions reopen priority after attackers and blockers are declared, moving the game into `DeclareBlockers` and `CombatDamage`
- the explicit combat corridor now reopens priority coherently as combat progresses from `DeclareAttackers` to `DeclareBlockers`, from `DeclareBlockers` to `CombatDamage`, and from resolved combat damage to `EndOfCombat`
- the active player may cast and resolve an instant after attackers are declared
- the active player may cast a second instant after attackers are declared before passing priority after the first
- the non-active player may cast and resolve an instant after attackers are declared once the active player passes
- the non-active player may cast a second instant in `DeclareBlockers` once the active player passes after attackers are declared
- the active player may cast and resolve an instant after blockers are declared
- the active player may cast a second instant after blockers are declared before passing priority after the first
- the non-active player may cast and resolve an instant after blockers are declared once the active player passes
- the non-active player may cast a second instant in `CombatDamage` once the active player passes after blockers are declared
- combat damage resolution moves the game into `EndOfCombat` and reopens priority for the active player while the game remains active
- the non-active player may cast and resolve an instant after combat damage once the active player passes
- the active player may cast and resolve an instant after combat damage resolves
- the active player may cast a second instant after combat damage resolves before passing priority after the first

The system intentionally excludes complex gameplay mechanics at this stage.

---

# Known Constraints

The current implementation includes several deliberate simplifications.

These constraints allow the system to evolve safely through vertical slices.

Current constraints include:

- matches support exactly two players
- opening hand size is fixed to 7 cards
- only a subset of zones are modeled (no exile)
- spell responses during open priority windows are currently limited to instants
- priority windows are currently opened by spell casting, by entering `Upkeep`, `Draw`, `FirstMain`, `BeginningOfCombat`, `SecondMain`, or `EndStep`, after attackers or blockers are declared, and after combat damage resolves if the game remains active
- outside stack-aware operations, general turn advancement still requires the priority window to be closed
- broader priority windows for non-main-phase turn flow beyond `Upkeep`, `Draw`, `EndStep`, and the current combat windows are not modeled yet
- combat now uses explicit subphases, but still omits many richer combat mechanics and triggered timing details
- no triggered abilities
- limited card behavior modeling
- permanent spells resolve from the stack into the battlefield in the current simplified stack model
- mana production is simplified to active-player main phases and generic mana only
- combat blocking is simplified to at most one blocker per attacker

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
- explicit game-end events with reasons for terminal empty-library draw and zero life
- shared life-change semantics reused by explicit life adjustment and combat damage
- shared review of currently supported state-based actions after relevant gameplay actions
- aggregate-owned stack zone and priority state with minimal public stack behavior
- an event bus for event distribution
- projections derived from gameplay events
- State pattern for phase transitions
- helper methods for event persistence and publishing
- a Gherkin acceptance layer, with executable coverage for stack foundation, spell casting through the stack, instant responses, active-player self-stacking across the currently supported priority windows, turn-flow priority windows, combat priority windows, combat damage, creature destruction, cleanup damage removal, cleanup hand-size discard, explicit multi-card draw effects, empty-library draw loss, and zero-life loss via `cucumber-rs`

This architecture supports:

- replayability
- observability
- deterministic state transitions
- State pattern for phase behavior encapsulation

---

# Next Modeling Decision

The next gameplay expansion requires choosing which domain capability to introduce next.

Possible directions include:

- broader stack and priority behavior on top of the current minimal implementation
- broader state-based actions beyond lethal creature damage and zero-toughness creature death
- broader game-loss and game-end conditions beyond empty-library draw and zero life
- richer cleanup and end-of-turn semantics

The next slice should continue expanding gameplay behavior incrementally.

---

# Guiding Principle

The system should evolve through **small, deterministic vertical slices**.

Each slice should:

- introduce one new gameplay capability
- extend the domain model minimally
- emit explicit domain events
- preserve existing invariants
