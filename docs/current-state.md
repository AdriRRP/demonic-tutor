# Current State — DemonicTutor

## Implemented Slices (7/7)

1. StartGame
2. DrawOpeningHands
3. PlayLand
4. AdvanceTurn
5. DrawCard
6. Mulligan
7. Infrastructure (EventBus, EventStore, GameLogProjection)

## Current Aggregate: Game

The `Game` aggregate handles:
- game lifecycle
- player management (exactly 2)
- zone transitions (library → hand → battlefield)
- turn progression
- action legality

## Constraints (Temporary)

- 2 players only
- Opening hand: 7 cards
- Phase: Setup → Main
- No stack, priority, spell casting
- In-memory event store (future: persistence)
- In-memory event bus (future: distributed)

## Quality Gates

- `cargo fmt --check`
- `cargo test`
- `cargo clippy [strict flags] -D warnings`
- Panic-free production code (`src/`)

## Next Decision Point

Choose next focus: new slice / internal refactor / infrastructure extraction
