# Slice Name

MulliganWithSetup

---

## Goal

Implement proper Magic mulligan sequence with Setup phase at game start, allowing players to take mulligans in turn order until all players keep their hands.

## Historical Note

This foundation slice introduced `Setup` and the first browser-visible mulligan corridor, but its one-mulligan simplification is no longer the full live truth. Later slices widened the current model to repeated London-style mulligans with explicit bottom-card selection before keeping.

---

## Why This Slice Exists Now

The current implementation removed the Setup phase, but Magic requires a pre-game mulligan phase. Without proper mulligan support, players cannot restart poor hands, which is essential for playtesting.

---

## Supported Behavior

### Setup Phase
- Phase exists once at game start (before first turn)
- Starting player is determined at game start
- Only valid phase for mulligan decisions
- Transition to Untap after all players keep hands

### Mulligan Sequence
1. Starting player decides: take mulligan or keep hand
2. If mulligan: shuffle hand back into library, draw new hand (7 cards)
3. After mulligan, that same player keeps the new hand because the current slice supports only one mulligan per player
4. Continue to the next player in turn order
5. After all players keep, proceed to Untap phase

> **Simplification**: Current implementation allows only one mulligan per player (not London Mulligan with multiple mulligans per round)

### Phase Sequence
- Setup (mulligan phase - once at game start)
- Untap
- Upkeep (keep/mulligan decision for subsequent turns - simplified)
- Draw
- FirstMain
- Combat
- SecondMain
- EndStep

### Upkeep Phase (simplified)
- In this implementation, Upkeep is a placeholder for potential future "scry 1" functionality
- Currently allows transition from Untap to Draw
- No player action required in Upkeep

---

## Invariants / Legality Rules

- Each player can only take one mulligan (simplified, not London Mulligan)
- Mulligan draws 7 cards from library
- Minimum hand size is 1 card
- After Setup, transition to Untap is automatic when all keep
- Mulligan is only valid during Setup phase

---

## Out of Scope

- Scry 1 after mulligan (future)
- Partial Paris Mulligan rules
- Priority during mulligan
- General-purpose keep/mulligan commands in the shared public client contract
- Multiple mulligan types beyond London

---

## Domain Impact

### Phase Enum
```rust
pub enum Phase {
    Setup,      // mulligan phase - unique per game
    Untap,
    Upkeep,     // placeholder for future scry/keep decision
    Draw,
    FirstMain,
    Combat,
    SecondMain,
    EndStep,
}
```

### Player State
- `mulligan_taken_this_round: bool` - track if player took mulligan in current round

### Turn Progression
- Setup → Untap (after all players keep)
- Untap → Upkeep → Draw → FirstMain → Combat → SecondMain → EndStep → Untap (next player)

### Commands
- MulliganCommand: takes a player ID, processes mulligan for that player

---

## Ownership Check

This behavior belongs to the `Game` aggregate because:
- manages turn and phase progression
- enforces phase-specific rules for mulligan
- tracks player state during mulligan sequence

---

## Documentation Impact

- `docs/domain/current-state.md` - update capabilities
- `docs/domain/aggregate-game.md` - add Setup and Upkeep phases
- `docs/rules/rules-map.md` - confirm rule 103.4 references

---

## Test Impact

- mulligan succeeds in Setup phase
- mulligan fails outside Setup phase
- one mulligan maximum is enforced per player
- starting player decides first and the next player decides after that opening hand is locked
- game proceeds to Untap after all keep

---

## Rules Reference

- 103.1 — Beginning the game
- 103.2 — Starting player
- 103.3 — Mulligan
- 103.4 — Starting hand
- 500-514 — Turn structure

---

## Rules Support Statement

This slice introduces Setup phase at game start with simplified mulligan support. Players can take one mulligan during Setup, shuffling their hand back into the library and drawing 7 new cards. After all players keep their hands, the game proceeds to the first turn's Untap phase. Multiple mulligans per round, scry 1, priority during mulligan, and full London Mulligan rules remain out of scope.

---

## Open Questions

1. Should Upkeep phase have any gameplay function in future? (scry mechanic)
2. Do we need to track total mulligans taken for hand size calculation?

---

## Review Checklist

- [x] Is the slice minimal?
- [x] Does it introduce one coherent behavior?
- [x] Are the legality rules explicit?
- [x] Is out-of-scope behavior stated clearly?
- [x] Does it avoid implying unsupported rules?
- [x] Is ownership clear?
- [x] Does it preserve bounded context and aggregate boundaries?
- [x] Is the slice easy to review and test?
