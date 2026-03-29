# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.8.0] - 2026-03-29

### Added

- **First honest curated limited shell**: the repository now freezes `playable_subset_version = v1`, publishes one canonical capability matrix plus deck-construction baseline for the first curated environment, and rejects curated card loads whose authored profiles exceed that supported subset
- **Curated golden matchup closure**: the current playable horizon now includes white-blue tempo, black-red value/sacrifice, and green-white counters golden matchups that exercise the intended first best-of-one product shell
- **Seeded public session bootstrapping**: the public application layer now supports deterministic seeded setup, rematch, replay log access, and a first real client-facing session bootstrap contract for the frozen subset

### Changed

- **Public runtime semantics were hardened aggressively**: observable zone changes now converge on one canonical language, cleanup/combat/resource corridors expose visible moves more truthfully, and public command envelopes stay aligned with the persisted replay stream
- **Read-side and replay hot paths were tightened**: priority snapshots, target caches, public event-log projection, and related read paths now avoid several redundant scans, clones, and cache misses that showed up in the final embed-grade audit passes
- **Release-horizon truth is now explicit**: the `0.8.0` planning backlog is closed, proposal wave directories are gone from the live backlog, and the repository now treats the first curated limited shell as a shipped historical milestone rather than an open proposal

### Documentation

- Synchronized canonical docs, runtime explanations, implemented slice history, README-level summaries, and release-horizon notes with the final `0.8.0` subset
- Added implemented slice records for the final public-surface hot-path cleanup and graveyard-target hardening work that landed during release closure

### Quality

- Multiple hypercritical elite/embed-grade audit follow-ups were folded into the release before tagging
- Strict repository validation remains clean through `./scripts/check-all.sh`

## [0.7.0] - 2026-03-28

### Added

- **Public gameplay contract for UI work**: the engine now exposes a stable public game snapshot, canonical legal-action queries, visible choice requests, and deterministic public command envelopes for frontend clients
- **Broader explicit choice corridors**: the current subset now supports `choose one`, `you may`, `loot`, `rummage`, `scry 1`, and `surveil 1` through surfaced pending decisions instead of hidden resolution shortcuts
- **Attachment and static battlefield baseline**: the current subset now supports `Enchant creature` Auras, attached stat bonuses, pacifism-style attachment restrictions, and one explicit controller anthem profile
- **Board-control utility for limited-style play**: the engine now supports `tap target creature`, `untap target creature`, `target creature can't block this turn`, and bounded defender-like attack restrictions
- **Broader battlefield snowball/value patterns**: the current subset now supports keyworded tokens, multi-token creation, spell recursion to hand, one-shot cast-from-graveyard profiles with exile on resolution, and an explicit end-step spell-recursion trigger

### Changed

- **Owner/controller semantics hardened**: cards now preserve persistent ownership across the currently supported temporary battlefield-control transfers, so death, bounce, exile, and other zone exits route back to the owner's zones truthfully
- **Public legality became canonical**: target candidates and blocker options are now derived from aggregate-owned read-only legality queries instead of speculative probes or duplicated application rules
- **Stack event semantics were hardened**: pass-priority, optional choices, loot/rummage, scry, and surveil now emit a more truthful observable event order, including visible zone changes for supported deferred-resolution corridors
- **Runtime internals were modularized further**: crowded runtime, player, public-surface, and spell-resolution hotspots were split into more focused modules to reduce cognitive load without widening aggregate boundaries
- **Attachment and SBA internals were tightened**: Aura attach/detach and orphan cleanup now rely on the aggregate's indexed location lookup paths more consistently, with stricter invariant handling

### Documentation

- Synchronized canonical docs and slice history around the final `0.7.0` subset and its UI-start gate
- Curated the live proposal backlog so only `0.8.0` work remains under `docs/slices/proposals/`
- Kept runtime, aggregate, and release documentation aligned with the current event, ownership, and public-query semantics

### Quality

- Multiple elite-audit follow-up fixes were folded into the release before tagging
- Strict repository validation remains clean through `./scripts/check-all.sh`

## [0.6.0] - 2026-03-25

### Added

- **Wave 1 stack interaction baseline**: the current subset now supports `counter target spell`, `return target permanent to hand`, `destroy target artifact or enchantment`, and targeted chosen-card discard
- **Wave 2 triggered-ability baseline**: supported permanents now exercise explicit `enter-the-battlefield`, `dies`, `beginning of upkeep`, and `beginning of end step` triggers through the stack
- **Wave 3 activated-ability usability**: the current engine now supports generalized non-mana tap abilities, mana-costed activations, sacrifice-as-cost activations, and the first explicit loyalty-ability corridor for supported planeswalkers
- **Wave 4 combat usability**: combat now supports multiple blockers per attacker, ordered damage assignment, `Deathtouch`, and `Double strike` in the current subset
- **Wave 7 counters and tokens baseline**: explicit vanilla creature-token creation, `+1/+1` counters, and explicit counter-placement effects are now supported
- **Wave 8 graveyard recursion baseline**: the subset now supports return-from-graveyard-to-hand, reanimation from the caster's own graveyard, explicit self/target mill, and explicit cast-from-own-graveyard profiles
- **Keyword resilience pass**: added current-subset support for `Menace`, `Lifelink`, creature `Hexproof`, and creature `Indestructible`

### Changed

- **Combat correctness tightened**: blocked attackers remain blocked across later combat-damage passes, and `Deathtouch + Trample` now uses one nonzero damage as lethal assignment before excess reaches the defending player
- **Triggered timing is broader and more truthful**: supported upkeep and end-step triggers now scan all relevant battlefields, not only the active player's
- **Planeswalker activation semantics tightened**: supported loyalty abilities are now limited to one activation per planeswalker each turn
- **Mill semantics corrected**: explicit mill effects now move as many cards as possible when the library contains fewer than `N`
- **Reanimation scope narrowed honestly**: reanimation now truthfully targets only creature cards in the caster's own graveyard until owner/controller separation is modeled correctly

### Documentation

- Synchronized canonical domain docs, implemented slices, and rules map with the final supported `0.6.0` subset
- Added and curated implemented slice docs for the `0.6.0` functional waves and their follow-up correctness fixes
- Kept the repository's architecture and release notes aligned with the actual supported gameplay surface

### Quality

- Multiple elite-audit follow-up fixes were folded into the release before tagging
- Strict repository validation remains clean through `./scripts/check-all.sh`

## [0.5.0] - 2026-03-25

### Added

- **First supported non-mana activated ability corridor**: supported non-mana activations now use the same aggregate-owned stack and priority corridor as other stack interactions
- **Keyword and combat effect coverage expanded**: the current subset now exercises `Haste`, `Vigilance`, `Trample`, `First strike`, and temporary pump effects that can change combat outcomes
- **Broader targeted effect subset**: the supported spell-effect corridor now includes `destroy target creature`, `exile target creature`, `exile target card from graveyard`, explicit life gain/loss effects, and actor-relative plus combat-relative target restrictions
- **Broader open-priority casting subset**: the current explicit `Flash`-like support now exercises supported creatures plus selected noncreature permanents across the modeled open-priority windows
- **Human-first runtime architecture guide**: added `docs/architecture/runtime-abstractions.md` with Feynman-style explanations and Mermaid diagrams for the core runtime abstractions

### Changed

- **Runtime identity model consolidated**: the core now prefers numeric-core ids, owner indices, and player card handles internally while materializing readable public ids at true boundaries
- **Player-owned runtime storage consolidated**: players now operate through a dense handle-first arena with boundary-only card-id lookups instead of treating public card ids as the canonical runtime path
- **Stack runtime compacted further**: stack objects now use thinner spell payloads, internal target references, internal source references for activated abilities, and less duplicated resolution metadata
- **Zone storage and lookup hot paths tightened**: ordered zones now combine reusable slots, visible indexing, and sparse-position compaction to keep reads and removals efficient without suffix-wide rewrites
- **Aggregate support structures tightened**: the aggregate-level location index now updates transactionally during known card moves instead of relying on player snapshot refreshes
- **Turn flow, combat, targeting, and resolution hot paths** now operate much more consistently on indices, handles, and compact refs instead of repeatedly cloning or resolving public ids

### Documentation

- Synchronized canonical architecture and domain docs with the final handle-first, compact-runtime model
- Curated proposal waves and implemented slices so the remaining backlog reflects only live work
- Refined Mermaid diagrams to a more conservative GitHub-friendly subset

### Quality

- Completed the `engine-audit` closure waves through `v11`, including the final runtime hot-path cleanup pass
- Strict repository validation remains clean through `./scripts/check-all.sh`

## [0.4.0] - 2026-03-22

### Added

- **Combat-context stack support expanded**: `Flash` creatures, artifacts, and enchantments now operate across the currently supported upkeep, stack-response, and combat priority windows
- **Actor-relative and combat-relative targeting expanded**: the stack now supports explicit target legality for opponent players, controlled creatures, attacking creatures, blocking creatures, and their current actor-relative combat variants
- **Mana model v1 completed**: the runtime now exercises five basic land colors, mixed costs such as `1G`, repeated same-color costs such as `GG`, generic payment from remaining colored mana, and explicit rejection when the required color symbol is missing
- **Executable mana coverage expanded**: Cucumber features now cover mixed and repeated colored spell payment plus mixed-cost rejection corridors

### Changed

- **Card storage model**: player-owned `Library`, `Hand`, `Battlefield`, `Graveyard`, and `Exile` now use id-backed storage with a unified owned-card store, while immutable card definitions are shared across instances
- **Stack runtime model**: stack objects now use cheaper internal numeric ids and carry spell snapshots instead of full `CardInstance` values
- **Targeting and combat hot paths**: battlefield lookup, single-blocker combat damage, mana payment, and supported state-based actions were simplified and tightened around the currently supported domain
- **Repository navigation and slices**: slice history is grouped by capability, completed proposal waves have been curated into implemented slices, and documentation navigation now uses portable relative links instead of workspace-specific absolute paths

### Documentation

- Added ADR `0015` to capture shared card-face storage and semantic zone-access guidance
- Refreshed development guidance with an explicit compact feature workflow for humans and agents
- Synchronized canonical docs, glossary, features index, slice backlog, and agent context with the current mana, targeting, stack, and storage model

### Quality

- Global consistency pass completed across code, docs, slice history, and agent context
- Strict repository validation remains clean through `./scripts/check-all.sh`

## [0.3.0] - 2026-03-18

### Added

- **Minimal stack and priority gameplay**: spells are now cast onto a real aggregate-owned stack, `PassPriority` is public, and `StackTopResolved` marks top-of-stack resolution after two consecutive passes
- **Priority windows across turn flow**: explicit empty priority windows now open in `Upkeep`, `Draw`, `FirstMain`, `BeginningOfCombat`, `EndOfCombat`, `SecondMain`, and `EndStep`
- **Priority windows across combat flow**: attackers, blockers, and combat damage now reopen priority coherently as combat progresses
- **Instant-speed interaction across supported windows**: the active player can cast instants, the non-active player can respond after the first pass, and the current priority holder can self-stack a second instant in the supported windows
- **Explicit combat subphases**: the turn model now uses `BeginningOfCombat`, `DeclareAttackers`, `DeclareBlockers`, `CombatDamage`, and `EndOfCombat`
- **Sorcery-speed spell support in main phases**: sorceries, creatures, artifacts, enchantments, and planeswalkers now resolve through the stack in empty `FirstMain` and `SecondMain` windows for the active player
- **Minimal explicit targeted instant subset**: supported instant spells can target a player or creature, validate that target at cast time, and preserve it on the stack object until resolution
- **Targeted gameplay effects outside the stack**: explicit draw effects can now target any player, and explicit life effects now distinguish `caster` and `target`

### Changed

- **Combat semantics**: combat now advances through explicit subphases instead of a single monolithic `Combat` phase
- **Timing semantics**: sorcery-speed legality is centralized and now consistently requires the active player's empty main-phase window
- **Targeted damage resolution**: targeted instant damage to players reuses shared life-change semantics, while targeted creature damage marks damage first and relies on shared SBA review for lethal destruction
- **Domain language layout**: play commands and events are now split internally by sublanguage while preserving the `play` bounded context and aggregate ownership
- **Repository truthfulness**: stack design docs, slice docs, current-state docs, public summaries, and agent guidance now describe the current stack/targeting model more honestly and mark older design notes as historical when appropriate

### Quality

- Executable BDD coverage expanded substantially across stack, combat, timing, targeting, draw, life, and game-end semantics
- Strict repository validation remains clean through `./scripts/check-all.sh`
- Canonical docs, agent context, and skills were synchronized with the current stack/priority and targeting model before release

## [0.2.0] - 2026-03-17

### Added

- **CombatDamage**: Resolve combat damage between attackers and blockers with marked creature damage
- **DeclareBlockers**: Declare blockers for attacking creatures
- **Full phase model**: `Setup -> Untap -> Upkeep -> Draw -> FirstMain -> Combat -> SecondMain -> EndStep`
- **Composite turn events**: `TurnProgressed` replaces technical turn delta events
- **Draw origin tracking**: `CardDrawn` now records whether the draw came from a turn step or explicit action
- **Runtime semantic tests**: regression coverage for combat damage, untap ownership, spell resolution, and zone invariants
- **Shared test support**: reusable helpers for common game setup and phase advancement flows
- **Repository curation skills**: reusable agent workflows for repository closing and release preparation

### Changed

- **Turn progression**: auto-untap happens only for the active player and automatic draw happens in the Draw phase
- **Spell casting semantics**: creatures are now cast through `CastSpell`, and permanent spells enter the battlefield while instants and sorceries resolve to the graveyard in the simplified model
- **Bounded context layout**: gameplay code now lives explicitly under `domain::play`
- **Game aggregate internals**: split into `model`, `rules`, and `invariants` for clearer ownership
- **Application layer**: command processing uses explicit service and aggregate methods instead of a generic command trait
- **Infrastructure layout**: event bus/store and projections now use more explicit module structure
- **Event payloads**: `SpellCast` now records card type, mana cost paid, and outcome for better replayability
- **Memory footprint**: identifiers now share storage with `Arc<str>` and card runtime state is more compact internally

### Quality

- Strict clippy warnings resolved
- Canonical docs, ADRs, slices, agent context, and skills synchronized with implementation
- Historical slices and ADRs marked explicitly when superseded

---

## [0.1.0] - 2026-03-14

### Added

- **Game Aggregate**: Core domain model for managing a Magic: The Gathering playtest session
- **Phase System**: `Phase::Setup` and `Phase::Main` to track game state progression
- **Player Management**: Support for exactly two players with unique identification
- **Zone System**: Library, Hand, and Battlefield zones for card management
- **StartGame Command**: Initialize a new game with two players
- **DealOpeningHands Command**: Deal 7-card opening hands to all players
- **PlayLand Command**: Play lands from hand to battlefield
- **AdvanceTurn Command**: Advance to next player's turn
- **DrawCard Command**: Draw cards from library to hand
- **Mulligan Command**: Return hand to library, shuffle, and draw new 7-card hand
- **EventStore Trait**: Abstraction for persisting domain events
- **EventBus Trait**: Abstraction for publishing domain events to subscribers
- **InMemoryEventStore**: In-memory implementation of EventStore
- **InMemoryEventBus**: In-memory implementation of EventBus
- **GameLogProjection**: Projection that accumulates human-readable event logs
- **Generic GameService**: Application service parameterized by EventStore and EventBus

### Changed

- **Setup Flow**: Opening hands are dealt during `Phase::Setup`, allowing mulligan before game begins
- **Phase Transition**: `Phase::Main` is now reached via `AdvanceTurn` command instead of automatic transition
- **GameService**: Now generic over EventStore and EventBus traits, persists and publishes events after each command

### Documentation

- Domain glossary defining ubiquitous language
- Vertical slice specifications for each feature
- Architectural decision records (ADRs)
- Agent entrypoint documentation for AI assistants
- Context map showing bounded contexts
- Aggregate documentation for Game

### Testing

- 35 integration tests covering all implemented slices
- Test coverage for:
  - StartGame validation (duplicate players, player count)
  - DealOpeningHands (card movement, event emission, error cases)
  - PlayLand (turn validation, land limits, zone transitions)
  - AdvanceTurn (player rotation, land reset)
  - DrawCard (phase validation, library management)
  - Mulligan (setup phase validation, one-time use)
  - Infrastructure (EventStore, EventBus, GameLogProjection)

### Quality

- Strict clippy linting enforced (`-D warnings`)
- Panic-free domain code
- No `unwrap_used` or `expect_used` in production code
- Consistent code formatting with `cargo fmt`
