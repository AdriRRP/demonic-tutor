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
- resolving explicit draw effects during main phases onto any player
- resolving explicit draw effects that draw multiple cards one by one onto the chosen player
- ending the game when a player must draw from an empty library
- ending the game when a player reaches 0 life
- playing lands
- tapping lands for mana
- tapping lands for mana in the currently exercised open priority windows while the acting player holds priority
- casting spells that require mana
- casting creature spells that enter the battlefield with power and toughness
- resolving instants and sorceries to graveyard
- supporting a minimal targeted instant subset against players or creatures through the current stack windows
- moving cards explicitly to exile from battlefield or graveyard
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
- resolving explicit targeted life effects
- advancing turns
- full phase progression using State pattern (Setup, Untap, Upkeep, Draw, FirstMain, BeginningOfCombat, DeclareAttackers, DeclareBlockers, CombatDamage, EndOfCombat, SecondMain, EndStep)
- keyword abilities: Flying and Reach affect combat blocking legality

These capabilities correspond to the slices currently implemented in the system.

---

# Current Domain Scope

The system currently models a **minimal subset of Magic gameplay** focused on playtesting.

The domain currently includes:

- game sessions
- players
- card instances with immutable face data and mutable runtime state
- shared immutable card definitions referenced by runtime card instances
- basic zones (library, hand, battlefield, graveyard, exile)
- player-owned zone carriers keyed by card id, with ordered library/hand/graveyard/exile views and intentionally unordered battlefield removal semantics
- mana production from lands
- minimal colored mana support for `Forest -> Green`, `Mountain -> Red`, `Plains -> White`, `Island -> Blue`, and single-color instant costs
- spell casting with mana cost
- transient mana pools cleared when the game advances to the next phase or turn
- the current transient mana model is now exercised explicitly from `Upkeep` into `Draw`
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
- exile zone as a player-owned zone where cards can be moved from battlefield or graveyard
- explicit exile effects that move a chosen card from battlefield or graveyard into its owner's exile zone
- summoning sickness for creatures (removed for the active player's creatures at turn start)
- turn and phase progression
- explicit draw effects as a simplified non-stack entrypoint, including multi-card targeted draw
- terminal game state when a player loses by empty-library draw or zero life
- casting spells onto an aggregate-owned stack zone
- public priority passing for the currently open minimal stack windows
- the casting player retains priority immediately after a spell is put on the stack
- currently supported open priority windows exist in `Upkeep`, `Draw`, `FirstMain`, `BeginningOfCombat`, post-attackers, post-blockers, `EndOfCombat`, `SecondMain`, and `EndStep`
- in each currently supported instant-speed window, the active player may cast an instant and self-stack a second instant before passing priority
- in each currently supported instant-speed window, the non-active player may respond with an instant after the first pass and may self-stack a second instant before passing priority
- the current response corridor also supports producing generic mana from a land, without using the stack, and immediately spending it to cast a paid instant response on the same open stack
- sorcery-speed spells are supported for the active player in empty `FirstMain` and `SecondMain` windows for the currently modeled spell-card subset: creature, sorcery, artifact, enchantment, and planeswalker
- the current supported spell-card subset also allows explicit card-face casting rules that open non-instant spells to open priority windows, providing minimal `Flash`-like support for the currently exercised subset
- the current minimal `Flash`-like support is currently exercised by supported creatures in `Upkeep`, `BeginningOfCombat`, on an existing stack response window, and after attackers, blockers, or combat damage, and by supported artifact and enchantment spells on an existing stack response window, in `BeginningOfCombat`, and after blockers or combat damage
- the current supported spell-card subset also allows explicit turn-relative open-priority casting rules for the currently supported noncreature permanent subset
- the current supported noncreature permanent subset for that rule is currently exercised by `Artifact` and `Enchantment` in `Upkeep`, `BeginningOfCombat`, post-attackers, post-blockers, and post-combat-damage
- the current turn-relative open-priority casting subset is explicitly rejected as a response during the opponent's turn for the currently exercised artifact and enchantment cases
- the current priority holder may cast and resolve a targeted instant at a player or creature in the currently supported targeted-spell subset whenever that holder can legally cast an instant in the current window
- supported targeted instants currently require exactly one explicit player or creature target when cast
- the current targeted-spell subset now supports contextual target restrictions such as `opponent of the acting player/controller` and `creature controlled by the acting player/controller`
- the current non-combat targeted-spell subset explicitly exercises `opponent player` and `creature you control` restrictions in `FirstMain`
- the current targeted-spell subset now also supports explicit combat-relative target restrictions such as `attacking creature`, `blocking creature`, `attacking creature you control`, `blocking creature you control`, `blocking creature an opponent controls`, and `attacking creature an opponent controls`
- the current combat-relative targeted-spell subset is currently exercised in the post-attackers and post-blockers windows, including lethal and nonlethal damage against attacking, blocking, controlled-attacking, controlled-blocking, opponent-controlled attacking, and opponent-controlled blocking creatures
- supported targeted instant damage to a player emits `LifeChanged` on resolution
- supported targeted instant damage to a creature marks damage and then relies on shared SBA review for lethal destruction
- supported targeted instants currently do not apply their effect if their only legal creature target is gone on resolution
- legal-target evaluation for the current targeted-spell subset is shared between cast-time validation and resolution-time revalidation, using explicit cast and resolution contexts
- supported spell targeting, casting rules, and resolution are currently carried as explicit card-face profiles rather than inferred from card-definition strings during casting or resolution
- card definitions are currently created through card-type-aware constructors so supported spell cards receive casting semantics when the face is built
- stack-borne spells now carry explicit spell snapshots and resolution metadata instead of reusing the full moved card runtime
- resolving the top stack object after two consecutive passes
- the explicit combat corridor progresses through `BeginningOfCombat`, `DeclareAttackers`, `DeclareBlockers`, `CombatDamage`, and `EndOfCombat`
- empty combat windows close forward coherently from `BeginningOfCombat` into `DeclareAttackers`, from `DeclareAttackers` into `DeclareBlockers`, from `DeclareBlockers` into `CombatDamage`, and from `EndOfCombat` into `SecondMain`
- combat actions reopen priority after attackers and blockers are declared, and combat damage resolution moves the game into `EndOfCombat` with a reopened priority window while the game remains active

The system intentionally excludes complex gameplay mechanics at this stage.

---

# Known Constraints

The current implementation includes several deliberate simplifications.

These constraints allow the system to evolve safely through vertical slices.

Current constraints include:

- matches support exactly two players
- opening hand size is fixed to 7 cards
- only a subset of zones are modeled (library, hand, battlefield, graveyard, exile)
- spell responses during open priority windows currently support instants plus the explicitly modeled `OpenPriorityWindow` subset
- the current targeted-spell subset is intentionally tiny and driven by explicit card-face legal-target rules and resolution profiles
- the current targeted-spell subset currently supports only a small explicit damage-instant subset, but that subset already includes actor-relative and combat-relative target restrictions
- sorcery-speed spells are currently supported only for the active player in `FirstMain` or `SecondMain` while the stack is empty
- priority windows are currently opened by spell casting, by entering `Upkeep`, `Draw`, `FirstMain`, `BeginningOfCombat`, `SecondMain`, or `EndStep`, after attackers or blockers are declared, and after combat damage resolves if the game remains active
- outside stack-aware operations, general turn advancement still requires the priority window to be closed
- broader priority windows for non-main-phase turn flow beyond `Upkeep`, `Draw`, `EndStep`, and the current combat windows are not modeled yet
- combat now uses explicit subphases, but still omits many richer combat mechanics and triggered timing details
- no triggered abilities
- limited card behavior modeling
- permanent spells resolve from the stack into the battlefield in the current simplified stack model
- mana production is simplified to generic mana plus a minimal colored subset (`White`, `Blue`, `Green`, `Red`) and is currently exercised in main phases plus the currently supported open priority windows while the acting player holds priority
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
- type-safe library initialization data with a unified card shape and optional creature profile
- domain events describing state transitions
- composite turn progression events and draw events with explicit origin
- explicit game-end events with reasons for terminal empty-library draw and zero life
- shared life-change semantics reused by explicit targeted life effects and combat damage
- shared review of currently supported state-based actions after relevant gameplay actions
- aggregate-owned stack zone and priority state with minimal public stack behavior
- semantic zone and player accessors shielding most core rules and shared tests from raw zone storage details
- player-owned card stores behind semantic zone views for library, hand, battlefield, graveyard, and exile
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
