# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

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
