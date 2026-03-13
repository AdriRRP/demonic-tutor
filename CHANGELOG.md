# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0] - 2026-03-13

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

### Changed

- **Setup Flow**: Opening hands are dealt during `Phase::Setup`, allowing mulligan before game begins
- **Phase Transition**: `Phase::Main` is now reached via `AdvanceTurn` command instead of automatic transition

### Documentation

- Domain glossary defining ubiquitous language
- Vertical slice specifications for each feature
- Architectural decision records (ADRs)
- Agent entrypoint documentation for AI assistants
- Context map showing bounded contexts

### Testing

- 29 integration tests covering all implemented slices
- Test coverage for:
  - StartGame validation (duplicate players, player count)
  - DealOpeningHands (card movement, event emission, error cases)
  - PlayLand (turn validation, land limits, zone transitions)
  - AdvanceTurn (player rotation, land reset)
  - DrawCard (phase validation, library management)
  - Mulligan (setup phase validation, one-time use)

### Quality

- Strict clippy linting enforced (`-D warnings`)
- Panic-free domain code
- No `unwrap_used` or `expect_used` in production code
- Consistent code formatting with `cargo fmt`
