# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.2.0] - Unreleased

### Added

- **CombatDamage**: Resolve combat damage between attackers and blockers (without creature destruction)
- **DeclareBlockers**: Declare blockers for attacking creatures
- **Phase::Combat**: Proper combat phase with begin/combat/end steps
- **Phase::Upkeep**: Intermediate phase between Untap and Draw
- **Turn Number**: Track and increment turn number
- **Summoning Sickness**: Creatures cannot attack the turn they enter battlefield
- **CardInstance damage**: Track damage on creatures during combat

### Changed

- **Phase Model**: Full 8-phase turn structure (Setup → Untap → Upkeep → Draw → FirstMain → Combat → SecondMain → EndStep)
- **Turn Progression**: Auto-untap at start of turn, auto-draw in Draw phase
- **Game Aggregate**: Split into internal modules by domain capability

### Quality

- Strict clippy warnings resolved
- Documentation synchronized with implementation

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
