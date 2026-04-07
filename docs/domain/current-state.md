# Current State — DemonicTutor

This document provides a snapshot of the current capabilities of the system.

It summarizes what parts of the domain are implemented and what areas remain intentionally incomplete.

Detailed slice documentation lives in:

docs/slices/

Curated-set authoring truth also lives in:

- [limited-set-capability-matrix.md](limited-set-capability-matrix.md)
- [limited-set-deck-construction-baseline.md](limited-set-deck-construction-baseline.md)
- [playable-subset-v1.md](playable-subset-v1.md)

---

# Implemented Gameplay Capabilities

The current implementation supports a minimal playable flow for deck playtesting.

Implemented capabilities include:

- starting a two-player game
- projecting a stable public game snapshot for clients, including phase, priority, stack, battlefield, graveyard, exile, and hand counts
- projecting decomposed mana-cost profiles for visible public cards so browser clients can render mana symbols without image assets
- surfacing a public legal-action menu derived from the current supported actor and game state
- deriving that public legal-action menu from read-only aggregate legality queries instead of speculative command probes
- surfacing explicit concede actions for active games in the public legal-action menu
- deriving current supported target-selection candidates and blocker options from canonical aggregate legality queries instead of application-local rule approximations
- surfacing public choice requests for target selection, explicit hand-card choice, bounded modal spell choice, bounded optional secondary-target spell choice, binary optional-effect decisions, and cleanup discard
- returning a deterministic public command envelope with emitted events, updated snapshot, legal actions, and visible choice requests
- exposing a deterministic persisted public event log with explicit sequence numbers for replay and animation clients
- exposing deterministic public prompt ordering for the current supported simultaneous trigger batches, choice requests, and target candidate lists
- exposing deterministic seeded game setup and rematch helpers for public session bootstrapping
- exposing `playable_subset_version = v1` in the public game view so clients can pin against the first frozen playable contract
- projecting the attached creature id for the current supported Aura subset in the public battlefield snapshot
- dealing opening hands
- mulligan support (London Mulligan - simplified)
- drawing cards (auto-draw when entering `Draw`)
- resolving explicit draw effects during main phases onto any player
- resolving explicit draw effects that draw multiple cards one by one onto the chosen player
- ending the game when a player must draw from an empty library
- ending the game when a player reaches 0 life
- ending the game when one player concedes
- playing lands
- tapping lands for mana
- tapping lands for mana in the currently exercised open priority windows while the acting player holds priority
- activating the current supported non-mana tap-ability subset through the stack in open priority windows
- activating the current supported mana-costed non-mana ability subset when the controller can pay the explicit activation cost
- activating the current supported sacrifice-cost ability subset when the source can be sacrificed as part of the activation cost
- activating the first explicit planeswalker loyalty ability subset in the active player's main phase, limited to one loyalty activation per supported planeswalker each turn
- casting spells that require mana
- casting creature spells that enter the battlefield with power and toughness
- resolving instants and sorceries to graveyard
- supporting a minimal targeted instant subset against players or creatures through the current stack windows
- moving cards explicitly to exile from battlefield or graveyard
- summoning sickness for creatures (removed for the active player's battlefield at turn start)
- declaring attackers in `DeclareAttackers`
- declaring blockers in `DeclareBlockers`
- blocking now supports multiple blockers per attacking creature in declared order
- opening priority windows across `Upkeep`, `Draw`, `FirstMain`, `BeginningOfCombat`, `EndOfCombat`, `SecondMain`, and `EndStep`
- opening a priority window when entering `BeginningOfCombat`
- opening priority windows after attackers and blockers are declared
- reopening priority after combat damage resolves while the game remains active in `EndOfCombat`
- allowing instant responses and active-player self-stacking in the currently supported stack windows
- resolving combat damage
- assigning attacker combat damage across multiple blockers in declared order
- keeping a blocked attacker blocked across the later supported combat-damage pass even if its blockers died earlier in combat
- applying unblocked combat damage to players through shared life-change semantics
- destroying creatures automatically when marked combat damage is lethal
- destroying creatures with 0 toughness automatically after creature-spell resolution
- clearing marked damage from surviving creatures when the turn ends
- discarding down to the maximum hand size before the turn can advance out of `EndStep`
- tracking player life totals
- resolving explicit targeted life effects
- resolving the first explicit `tap target creature` spell subset through the shared targeting and stack corridor
- resolving the first explicit `untap target creature` spell subset through the shared targeting and stack corridor
- resolving the first explicit `target creature can't block this turn` spell subset through the shared targeting and combat-legality corridor
- resolving the first explicit bounded `distribute two +1/+1 counters among up to two target creatures` spell subset
- resolving the first explicit `attacks` and `deals combat damage to a player` triggered-ability subset through the shared combat and stack corridors
- deriving an explicit limited-set card-profile catalog from authored `LibraryCard` definitions for the first curated environment
- rejecting curated-set library loads whose authored `LibraryCard` definitions exceed that supported profile catalog
- publishing one canonical curated-set card capability matrix for set design
- publishing one canonical deck-construction baseline for the first curated limited environment
- executing a third curated golden matchup for green-white counters through distributed counters, token buildup, anthem scaling, and combat growth
- executing a second curated golden matchup for black-red value play through discard, sacrifice-cost activation, removal, and creature recursion
- executing the first curated golden matchup for white-blue tempo through flyers, bounce, and combat tricks
- resolving the first explicit keyworded creature-token creation subset for one supported token with one supported keyword
- resolving the first explicit multi-token creation subset for one supported effect that creates multiple identical vanilla creature tokens
- advancing turns
- full phase progression using State pattern (Setup, Untap, Upkeep, Draw, FirstMain, BeginningOfCombat, DeclareAttackers, DeclareBlockers, CombatDamage, EndOfCombat, SecondMain, EndStep)
- keyword abilities: Flying and Reach affect combat blocking legality, Haste bypasses summoning-sickness attack restriction, Vigilance avoids tapping on attack, Menace requires at least two blockers, Trample assigns excess damage after forward lethal assignment through declared blockers, First strike splits combat damage into an earlier and later supported pass, Double strike deals damage in both supported combat-damage passes, Lifelink gains life equal to combat damage dealt in the supported subset, Hexproof rejects opposing targeted spells against the supported creature subset, Indestructible survives the current lethal-damage and destroy corridors, Defender rejects attack declaration for the supported static subset, and Deathtouch makes nonzero combat damage lethal for the current SBA subset
- the supported `Deathtouch + Trample` interaction now uses 1 nonzero damage as lethal assignment before excess reaches the defending player

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
- runtime identity inside the aggregate now prefers numeric-core ids, player indices, and player-owned card handles over public string ids
- public `CardInstanceId` and `PlayerId` values are now treated primarily as readable boundary identities for commands, events, and tests
- visible public card projections now include decomposed mana-cost profiles for battlefield, graveyard, and exile cards
- mana production from lands
- explicit activated mana-ability profiles for the currently supported mana-producing permanents
- the current supported non-mana tap-ability corridor using the same priority and stack model as other stack interactions, including no-target `Tap: you gain life` and targeted `Tap: target player gains life`
- the current supported non-mana activation subset also includes explicit mana costs paid atomically before stack insertion
- the current supported non-mana activation subset also includes explicit `sacrifice this source` costs paid before the ability resolves
- the current supported planeswalker subset now includes explicit initial loyalty plus profile-based loyalty abilities whose loyalty change is paid on activation
- minimal colored mana support for `Forest -> Green`, `Mountain -> Red`, `Plains -> White`, `Island -> Blue`, `Swamp -> Black`, single-color instant costs, a first mixed `generic + colored` spell cost corridor, repeated same-color costs such as `GG`, colored mana satisfying generic requirements after colored symbols are reserved, and explicit rejection when a required colored symbol is missing
- spell casting with mana cost
- transient mana pools cleared when the game advances to the next phase or turn
- the current transient mana model is now exercised explicitly from `Upkeep` into `Draw`
- creature cards with power and toughness
- creature spells entering the battlefield through `CastSpell`
- creature damage tracking during combat
- ordered blocker groups per attacking creature
- player life changes from combat damage
- automatic destruction of creatures with lethal marked damage
- automatic destruction of creatures with 0 toughness after creature-spell resolution
- shared state-based action review after relevant gameplay actions for the currently supported SBA subset
- the current SBA subset remains limited to `0 toughness`, lethal marked damage, and `0 life`, and that same subset now also covers the supported pump, trample, and first-strike corridors
- cleanup-based removal of marked damage from surviving creatures
- explicit cleanup discard to maximum hand size during `EndStep`
- exile zone as a player-owned zone where cards can be moved from battlefield or graveyard
- explicit exile effects that move a chosen card from battlefield or graveyard into its owner's exile zone
- summoning sickness for creatures (removed for the active player's creatures at turn start)
- turn and phase progression
- explicit draw effects as a simplified non-stack entrypoint, including multi-card targeted draw
- terminal game state when a player loses by empty-library draw or zero life, and a drawn terminal state when both players reach zero life simultaneously
- casting spells onto an aggregate-owned stack zone
- public priority passing for the currently open minimal stack windows
- the casting player retains priority immediately after a spell is put on the stack
- currently supported open priority windows exist in `Upkeep`, `Draw`, `FirstMain`, `BeginningOfCombat`, post-attackers, post-blockers, `EndOfCombat`, `SecondMain`, and `EndStep`
- in each currently supported instant-speed window, the active player may cast an instant and self-stack a second instant before passing priority
- in each currently supported instant-speed window, the non-active player may respond with an instant after the first pass and may self-stack a second instant before passing priority
- the current response corridor also supports producing generic mana from a land, without using the stack, and immediately spending it to cast a paid instant response on the same open stack
- the current stack model now supports both spells and the current supported non-mana activated ability object family
- sorcery-speed spells are supported for the active player in empty `FirstMain` and `SecondMain` windows for the currently modeled spell-card subset: creature, sorcery, artifact, enchantment, and planeswalker
- the current supported spell-card subset also allows explicit card-face casting rules that open non-instant spells to open priority windows, providing minimal `Flash`-like support for the currently exercised subset
- the current minimal `Flash`-like support is currently exercised by supported creatures in `Upkeep`, `BeginningOfCombat`, on an existing stack response window, and after attackers, blockers, or combat damage, and by the current supported noncreature subset (`Artifact`, `Enchantment`, `Planeswalker`) on an existing stack response window, in `BeginningOfCombat`, and after blockers or combat damage
- the current supported spell-card subset also allows explicit turn-relative open-priority casting rules for the currently supported noncreature permanent subset
- the current supported noncreature permanent subset for that rule is currently exercised by `Artifact` and `Enchantment` in `Upkeep`, `BeginningOfCombat`, post-attackers, post-blockers, and post-combat-damage
- the current turn-relative open-priority casting subset is explicitly rejected as a response during the opponent's turn for the currently exercised artifact and enchantment cases
- the current priority holder may cast and resolve a targeted instant at a player or creature in the currently supported targeted-spell subset whenever that holder can legally cast an instant in the current window
- supported targeted instants currently require exactly one explicit player or creature target when cast
- the current targeted-spell subset now supports contextual target restrictions such as `opponent of the acting player/controller` and `creature controlled by the acting player/controller`
- the current non-combat targeted-spell subset explicitly exercises `any player`, including self-targeting, plus `opponent player`, `creature you control`, and `creature an opponent controls` restrictions in `FirstMain`, including cast-time rejection and successful resolution for the currently modeled opponent-controlled-creature damage corridor
- the current non-combat target matrix exercised in `FirstMain` is:
  - `AnyPlayer`: cast and resolve at opponent, cast and resolve at self
  - `OpponentOfActor`: cast and resolve at opponent, reject self
  - `CreatureControlledByActor`: cast and resolve at controlled creature, reject opponent-controlled creature
  - `CreatureControlledByOpponent`: cast and resolve at opponent-controlled creature, reject controlled creature
- the current targeted-spell subset now also supports explicit combat-relative target restrictions such as `attacking creature`, `blocking creature`, `attacking creature you control`, `blocking creature you control`, `blocking creature an opponent controls`, and `attacking creature an opponent controls`
- the current combat-relative targeted-spell subset is currently exercised in the post-attackers and post-blockers windows, including lethal and nonlethal damage against attacking, blocking, controlled-attacking, controlled-blocking, opponent-controlled attacking, and opponent-controlled blocking creatures
- the current spell-effect subset also supports first direct `destroy target creature`, `exile target creature`, `exile target card from graveyard`, and minimal `+N/+N until end of turn` corridors outside combat in `FirstMain`
- the current graveyard-recursion subset supports returning a target creature card to hand from any graveyard and reanimating a target creature card from the caster's own graveyard onto the resolving spell controller's battlefield
- the current graveyard-recursion subset also supports returning a target instant or sorcery card from a graveyard to its owner's hand
- the current graveyard-casting subset also supports one explicit profile that is exiled after resolving when cast from its owner's graveyard
- cards now preserve persistent owner identity even if they temporarily live on another player's battlefield, so death, bounce, and exile still route them back to the owner's zones
- supported Aura detach cleanup now uses explicit aggregate location lookup on the main zone-move corridors when that shared index is available
- the current mill subset mills up to `N` cards, stopping early if the target library runs out
- the current targeted-spell subset now rejects opposing targets with supported creature `Hexproof` during cast validation and resolution revalidation
- the current `destroy target creature` subset now leaves supported indestructible creatures on the battlefield
- the current spell-effect subset also supports chosen-card discard from a targeted player's hand through an explicit target-plus-choice command corridor
- the current spell-effect subset now also supports the first explicit `tap target creature` resolution profile against creatures on the battlefield
- the current temporary pump subset is also exercised in combat by casting a pump spell after blockers to change the same turn's combat outcome
- the current player-target spell subset now also supports explicit `gain life` and explicit `lose life` as distinct effects from damage while reusing the shared life-change corridor
- the current player-target spell subset now also supports the first explicit `choose one` corridor with a selected mode stored on stack and resolved deterministically
- the current trigger subset now also supports the first explicit `you may` corridor with a pending yes/no choice surfaced at resolution time
- the current trigger subset now also supports one explicit beginning-of-end-step recursion profile that returns the first supported instant or sorcery card from its controller's graveyard to hand
- the current trigger subset now also supports one explicit `Attacks` profile and one explicit `DealsCombatDamageToPlayer` profile that both reuse the shared triggered-ability stack corridor
- the current authoring boundary now classifies supported curated-set cards through one explicit limited-set profile catalog keyed by base card family plus the currently allowed authored subprofiles
- pending stack-time player choices are now modeled through one closed aggregate concept instead of parallel pending fields
- aggregate location lookups and compact battlefield refs now name the current player-arena index explicitly instead of calling that carrier position an owner index
- public and legality read paths for visible zones now prefer semantic card iterators (`hand_cards`, `battlefield_cards`, `graveyard_cards`, `exile_cards`) over raw zone-handle walks so visible-zone desynchronization fails explicitly instead of shrinking the observed state silently
- the current spell-effect subset now also supports explicit `loot` and `rummage`, surfaced as pending hand-card choice prompts during resolution
- the current library-manipulation subset now also supports explicit `scry 1`, surfaced as a controller-scoped pending top-card choice during resolution
- the current library-and-graveyard value subset now also supports explicit `surveil 1`, surfaced as a controller-scoped pending top-card choice that can keep the card on top or move it to graveyard during resolution
- the current counter-placement subset now also supports one bounded distribution profile that places two `+1/+1` counters among up to two chosen creature targets
- the current attachment subset now supports `Enchant creature` Auras that target while cast, enter attached if the target stays legal on resolution, and are put into graveyard by SBA if they become unattached
- the current attachment subset now also supports one explicit attached `+N/+N` Aura bonus profile while the Aura remains attached
- the current attachment subset now also supports one explicit pacifism-style Aura profile that prevents the enchanted creature from attacking and blocking while attached
- the current non-attachment static subset now also supports one explicit controller-scoped anthem profile that gives controlled creatures `+1/+1` while the permanent remains on the battlefield
- supported targeted instant damage to a player emits `LifeChanged` on resolution
- supported targeted instant damage to a creature marks damage and then relies on shared SBA review for lethal destruction
- supported indestructible creatures survive the current SBA lethal-damage review while still dying to zero toughness
- supported targeted instants currently do not apply their effect if their only legal creature target is gone on resolution
- legal-target evaluation for the current targeted-spell subset is shared between cast-time validation and resolution-time revalidation, using explicit cast and resolution contexts
- supported spell targeting, casting rules, and resolution are currently carried as explicit card-face profiles rather than inferred from card-definition strings during casting or resolution
- spell targeting and spell-resolution profiles now live in a focused submodule instead of continuing to grow one central rules hotspot
- spell resolution effects now also live in focused internal submodules instead of continuing to grow one monolithic resolver file
- card definitions are currently created through card-type-aware constructors so supported spell cards receive casting semantics when the face is built
- stack-borne spells now carry explicit spell snapshots and resolution metadata instead of reusing the full moved card runtime
- supported activated abilities on the stack now also prefer internal source references, materializing public card ids only when leaving the runtime core
- the aggregate card-location index remains the canonical lookup for supported zone moves, while the current SBA review regenerates a fresh index from live player state on each iteration
- ordered visible zones now combine reusable slots, visible indexing, and sparse-position compaction so position lookup and ordered removal both avoid the earlier linear hot-path costs
- resolving the top stack object after two consecutive passes
- the explicit combat corridor progresses through `BeginningOfCombat`, `DeclareAttackers`, `DeclareBlockers`, `CombatDamage`, and `EndOfCombat`
- multi-blocked attackers now use declared blocker order when assigning combat damage in the supported subset
- combat damage now uses temporary keyed accumulation and blocker lookup instead of repeated linear scans through already collected participants
- attackers that were blocked remain blocked for the later supported combat-damage pass even if no blocker survives into that pass
- empty combat windows close forward coherently from `BeginningOfCombat` into `DeclareAttackers`, from `DeclareAttackers` into `DeclareBlockers`, from `DeclareBlockers` into `CombatDamage`, and from `EndOfCombat` into `SecondMain`
- combat actions reopen priority after attackers and blockers are declared, and combat damage resolution moves the game into `EndOfCombat` with a reopened priority window while the game remains active

The system intentionally excludes complex gameplay mechanics at this stage.

---

# Stable Capability Matrices

These matrices compress the stable supported subset without implying broader Magic support.

## Mana

- sources:
  - `Forest -> Green`
  - `Mountain -> Red`
  - `Plains -> White`
  - `Island -> Blue`
  - `Swamp -> Black`
- payment:
  - generic costs are supported
  - single colored costs are supported
  - mixed costs like `1G` are supported
  - repeated colored costs like `GG` are supported
  - colored mana may pay generic requirements
  - missing required color still rejects the cast
- activation:
  - the current land-tap corridor is a supported mana ability
  - supported mana abilities remain stack-free
  - supported non-mana activated abilities use the stack

## Casting And Stack

- spell-card subset:
  - `Creature`
  - `Instant`
  - `Sorcery`
  - `Artifact`
  - `Enchantment`
  - `Planeswalker`
- sorcery-speed windows:
  - active player only
  - `FirstMain`
  - `SecondMain`
  - stack empty
- open-priority casting windows:
  - `Upkeep`
  - `Draw`
  - `FirstMain`
  - `BeginningOfCombat`
  - post-attackers
  - post-blockers
  - `EndOfCombat`
  - `SecondMain`
  - `EndStep`
- explicit `Flash`-like support:
  - supported creatures
  - supported `Artifact`
  - supported `Enchantment`
  - supported `Planeswalker`
- stack objects:
  - spells
  - supported non-mana activated tap abilities with explicit profile-based targeting when modeled
  - the first supported Aura corridor carried by an `Enchantment` spell plus explicit creature attachment semantics

## Targeting

- player target rules:
  - `AnyPlayer`
  - `OpponentOfActor`
- creature target rules:
  - `AnyCreatureOnBattlefield`
  - `CreatureControlledByActor`
  - `CreatureControlledByOpponent`
  - `AttackingCreature`
  - `BlockingCreature`
  - `CreatureControlledByActorAndAttacking`
  - `CreatureControlledByActorAndBlocking`
  - `BlockingCreatureControlledByOpponent`
  - `AttackingCreatureControlledByOpponent`
- graveyard targets:
  - explicit card target in graveyard for the supported exile corridor
- legality:
  - casting and resolution share the same explicit legality semantics
  - unsupported target families are not implied

## Combat

- core flow:
  - `BeginningOfCombat`
  - `DeclareAttackers`
  - `DeclareBlockers`
  - `CombatDamage`
  - `EndOfCombat`
- supported invariants:
  - one blocker still blocks at most one attacker in the current subset
  - multi-blocked attackers assign damage forward through blockers in declared order
  - an attacker that was blocked stays blocked across the later supported combat-damage pass even if all blockers died in the earlier pass
  - `Deathtouch + Trample` uses 1 nonzero damage as lethal assignment per blocker in the current subset
  - combat damage reopens priority into `EndOfCombat` if the game remains active
  - shared SBA review covers `0 toughness`, lethal marked damage, and `0 life`
- supported combat keywords:
  - `Haste`
  - `Vigilance`
  - `Menace`
  - `Trample`
  - `Lifelink`
  - `First strike`
  - `Double strike`
  - `Deathtouch`
- current combat-relative corridors:
  - attacking-creature targeting
  - blocking-creature targeting
  - temporary pump can change combat outcomes
  - combat damage to players uses shared life-change semantics

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
- minimal explicit triggered abilities only for the currently modeled subset:
  - enter-the-battlefield life-gain triggers from supported permanents
  - dies life-gain triggers from supported creatures that move from battlefield to graveyard
  - beginning-of-upkeep triggers from supported battlefield permanents across all controllers
  - beginning-of-end-step triggers from supported battlefield permanents across all controllers
- limited card behavior modeling
- permanent spells resolve from the stack into the battlefield in the current simplified stack model
- mana production is simplified to generic mana plus a minimal colored subset (`White`, `Blue`, `Black`, `Green`, `Red`) and is currently exercised in main phases plus the currently supported open priority windows while the acting player holds priority
- the current land-tap corridor is backed by an explicit activated mana-ability profile and remains stack-free, while the currently supported non-mana activated-ability subset is intentionally limited to one untargeted life-gain corridor
- combat blocking is simplified to at most one blocker per attacker

These constraints are expected to evolve in future slices.

---

# Infrastructure State

The system currently runs entirely in-memory.

Infrastructure components include:

- in-memory event store
- in-memory event bus
- projections for gameplay logs
- crate validation that now includes `cargo check --target wasm32-unknown-unknown`

Persistent infrastructure may be introduced in future iterations.

The current portability boundary is:

- the crate validates compilation for `wasm32-unknown-unknown`
- the domain core now routes its shared ownership and hash collections through one alloc-friendly portability module
- application and infrastructure still depend on `std`
- the project is not `no_std`-ready

---

# Architectural Status

The project currently includes:

- a core `Game` aggregate with centralized player access
- command-driven gameplay operations
- play-owned library initialization data for opening hands
- type-safe library initialization data with a unified card shape and optional creature profile
- domain events describing state transitions
- composite turn progression events and draw events with explicit origin
- explicit game-end events with reasons for terminal empty-library draw, zero life, and simultaneous zero life
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
- broader state-based actions beyond the current explicit subset of `0 toughness`, lethal creature damage, and `0 life`
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
