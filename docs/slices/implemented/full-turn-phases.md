# Slice Name

FullTurnPhases

> **Note**: This slice has been superseded by `mulligan-with-setup.md` which adds Setup and Upkeep phases.

---

## Goal

Expand the turn model to include all standard Magic phases: Setup, Untap, Upkeep, Draw, First Main, Combat, Second Main, and End Step. This replaces the current simplified phase model with the proper Magic turn structure.

---

## Why This Slice Exists Now

The current simplified model (Setup → Main → Combat → Ending) was too limited. The introduction of `mulligan-with-setup.md` re-introduced the `Setup` and `Upkeep` phases. This slice ensures the full turn structure is accurately reflected.

1. Combat damage is now implemented, requiring proper phase structure
2. Draw should happen at start of turn, not as explicit command
3. Two main phases (pre-combat and post-combat) are needed for proper game flow
4. End step is needed for end-of-turn effects (future)

---

## Supported Behavior

- expand `Phase` enum to: Untap, Draw, FirstMain, Combat, SecondMain, EndStep
- auto-draw at start of turn (Draw phase) - replaces explicit DrawCardCommand for normal turn draw
- untap all permanents at start of turn (Untap phase) - automatic
- phase progression via AdvanceTurnCommand:
  - Untap → Draw
  - Draw → FirstMain
  - FirstMain → Combat
  - Combat → SecondMain
  - SecondMain → EndStep
  - EndStep → (next player's Untap)
- maintain existing functionality: lands, spells, combat in correct phases

---

## Invariants / Legality Rules

- untap: automatic at turn start, no player action needed
- draw: automatic at start of turn (one card), fails if library empty
- first main: play lands, cast spells
- combat: declare attackers, blockers, resolve damage
- second main: play lands (from other phase), cast spells
- end step: cleanup, end of turn effects (future)
- lands can only be played once per turn (across both main phases)
- creatures can only attack once per turn (in combat phase)

---

## Out of Scope

- priority system
- stack
- triggered abilities
- upkeep step (rule 502)
- begin combat step / end combat step
- cleanup step
- extra turns
- skipping phases
- phase-specific effects

---

## Domain Impact

### Phase Enum
```rust
pub enum Phase {
    Untap,
    Draw,
    FirstMain,
    Combat,
    SecondMain,
    EndStep,
}
```

### Turn Progression Changes
- new phase sequence
- automatic untap at start
- automatic draw at Draw phase

### Commands
- existing commands work in appropriate phases

### Events
- add PhaseUntap, PhaseDraw events (optional, or reuse PhaseChanged)

---

## Ownership Check

This behavior belongs to the `Game` aggregate because:

- it manages turn and phase progression
- it enforces phase-specific rules
- it produces phase change events

---

## Documentation Impact

- `docs/domain/current-state.md` - update capabilities
- `docs/domain/aggregate-game.md` - extend phase model
- `docs/rules/rules-map.md` - add full turn phase rules

---

## Test Impact

- turn progresses through all phases in order
- auto-draw happens in Draw phase
- lands/spells work in correct main phases
- combat works in Combat phase
- lands can be played once across both main phases

---

## Rules Reference

- 500.1 — Turn
- 501.1 — Beginning phase
- 502.1 — Untap step
- 502.2 — Automatic untap
- 503.1 — Upkeep step (not implemented)
- 504.1 — Draw step
- 504.2 — Automatic draw
- 505.1 — Main phase
- 506.1 — Combat phase
- 506.4 — End of combat step
- 507.1 — Ending phase
- 507.2 — End step

---

## Rules Support Statement

This slice introduces the full Magic turn structure with six phases. It replaces the simplified phase model with: Untap → Draw → FirstMain → Combat → SecondMain → EndStep. Automatic actions (untap, draw) happen in their respective phases. Priority, stack, and triggered abilities remain out of scope.

---

## Open Questions

1. Should we keep explicit DrawCardCommand as alternative to auto-draw?
2. Do we need separate BeginCombat/EndCombat steps?
3. Should damage clear at end of turn? (rule 703.4n)

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
