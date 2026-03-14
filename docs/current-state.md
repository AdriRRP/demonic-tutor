# Current State — DemonicTutor

## Implemented Slices (9/11)

1. StartGame
2. DrawOpeningHands
3. PlayLand
4. AdvanceTurn
5. DrawCard
6. Mulligan
7. Infrastructure (EventBus, EventStore, GameLogProjection)
8. Player Life
9. Turn Number

## Current Aggregate: Game

The `Game` aggregate handles:
- game lifecycle
- player management (exactly 2)
- player life totals (starts at 20)
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

Choose next focus: Turn Number / Turn Phases / Mana / Combat
