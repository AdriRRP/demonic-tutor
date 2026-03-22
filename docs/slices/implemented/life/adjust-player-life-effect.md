# Slice Name

AdjustPlayerLifeEffect

---

## Goal

Provide a minimal explicit life-effect entrypoint that lets a caster choose which player gains or loses life.

---

## Why This Slice Exists Now

This slice follows targeted explicit draw effects because:

1. the public domain surface already exposes effect-style commands outside stack objects
2. life totals are already modeled and observable through `LifeChanged`
3. zero-life loss already exists and becomes more useful when the target player is explicit
4. the change improves ubiquitous language without forcing timing rules for all life effects yet

---

## Supported Behavior

- replace `AdjustLifeCommand` with `AdjustPlayerLifeEffectCommand`
- require both `caster_id` and `target_player_id` to reference players in the game
- apply positive deltas as life gain and negative deltas as life loss to the target player
- emit `LifeChanged` for the target player
- reuse supported state-based-action review after the life change
- emit `GameEnded(ZeroLife)` if the targeted player reaches 0 life

---

## Invariants / Legality Rules

- life effects use saturating arithmetic at 0
- the target player is the one whose life total changes
- a missing caster or target makes the command invalid
- completed life change is not rolled back if state-based review then ends the game

---

## Out of Scope

- spell objects that carry life effects on the stack
- timing restrictions beyond the current aggregate action window semantics
- prevention or replacement effects
- poison or other loss conditions

---

## Domain Impact

### Commands

- rename `AdjustLifeCommand` to `AdjustPlayerLifeEffectCommand`

### Aggregate / Rules

- rename the public aggregate entrypoint to `adjust_player_life_effect`
- keep the shared `LifeChanged` / `GameEnded` semantics

### Events

- no new event types

---

## Test Impact

- unit coverage for targeting another player
- unit coverage for targeted life gain
- BDD coverage for explicit targeted life effects

---

## Rules Reference

- 118.1
- 118.2
- 104.3b
- 704.5a

---

## Rules Support Statement

DemonicTutor supports explicit targeted life effects as a simplified non-stack effect entrypoint. A caster identifies the target player directly in the command, the target player's life total changes immediately, and existing zero-life loss semantics still apply.
