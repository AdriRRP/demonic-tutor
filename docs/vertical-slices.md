# Vertical Slices — DemonicTutor

## Purpose

Vertical slices are the primary evolution mechanism. Each slice delivers one coherent, testable, observable behavior.

## Implemented Slices

| # | Slice | File |
|---|-------|------|
| 1 | StartGame | `docs/slices/start-game.md` |
| 2 | DrawOpeningHands | `docs/slices/draw-opening-hands.md` |
| 3 | PlayLand | `docs/slices/play-land.md` |
| 4 | AdvanceTurn | `docs/slices/advance-turn.md` |
| 5 | DrawCard | `docs/slices/draw-card.md` |

## Future Slices

Future slices will expand:
- combat phase
- spell casting
- card abilities
- turn phases beyond Main
- multiplayer support

See `docs/slices/` for detailed slice specifications.

## Policy

- Slices must be implemented in order
- Each slice introduces only the minimum rule subset required
- No slice implies broader rules support than explicitly implemented
