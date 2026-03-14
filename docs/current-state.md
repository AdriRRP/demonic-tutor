# Current State — DemonicTutor

## Implemented Slices (11/11)

1. StartGame
2. DrawOpeningHands
3. PlayLand
4. AdvanceTurn
5. DrawCard
6. Mulligan
7. Infrastructure (EventBus, EventStore, GameLogProjection)
8. Player Life
9. Turn Number
10. Turn Phases
11. Tap Lands for Mana

## Current Aggregate: Game

The `Game` aggregate handles:
- game lifecycle
- player management (exactly 2)
- player life totals (starts at 20)
- mana pool (starts at 0)
- zone transitions (library → hand → battlefield)
- turn progression (turn number, active player)
- phase progression (Setup → Beginning → Main → Ending)
- action legality

## Constraints (Temporary)

- 2 players only
- Opening hand: 7 cards
- Phase: Setup → Beginning → Main → Ending → Main (next player)
- No stack, priority, spell casting
- In-memory event store (future: persistence)
- In-memory event bus (future: distributed)

## Quality Gates

- `cargo fmt --check`
- `cargo test`
- `cargo clippy [strict flags] -D warnings`
- Panic-free production code (`src/`)

## Next Decision Point

Choose next focus: Cast Non-Land Spells / Pay Mana Cost / Combat
