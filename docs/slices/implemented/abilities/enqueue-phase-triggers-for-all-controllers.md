# Slice Name

Enqueue Phase Triggers For All Controllers

## Goal

Make supported upkeep and end-step battlefield triggers observe all relevant permanents, not only the active player's.

## Why This Slice Exists Now

The trigger foundation already modeled `BeginningOfUpkeep` and `BeginningOfEndStep`, but the first implementation only scanned one player's battlefield on phase entry. That silently skipped valid triggers controlled by the opponent during the active player's turn.

## Supported Behavior

- when `Upkeep` begins, enqueue supported battlefield upkeep triggers from all controllers
- when `EndStep` begins, enqueue supported battlefield end-step triggers from all controllers
- preserve controller ownership of each trigger while using the current active-player-first ordering

## Invariants / Legality Rules

- phase-entry triggers are discovered before ordinary priority actions in that step
- a trigger is controlled by the controller of its source permanent
- the current subset still only covers supported battlefield trigger profiles

## Out of Scope

- delayed triggers created by arbitrary prior effects
- full multiplayer trigger ordering beyond the current turn-order model
- intervening-if clauses or optional trigger choices beyond the supported subset

## Domain Impact

### Aggregate Impact

- widen phase-entry trigger discovery from single-controller scan to aggregate-wide battlefield scan

## Ownership Check

This belongs to the `Game` aggregate because step entry, battlefield observation, and trigger stack insertion are aggregate-owned turn-flow semantics.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`
- `docs/rules/rules-map.md`
- `docs/slices/implemented/abilities/upkeep-trigger-foundation.md`
- `docs/slices/implemented/abilities/end-step-trigger-foundation.md`
- this slice doc

## Test Impact

- beginning-of-upkeep enqueues supported triggers from both players' battlefields
- beginning-of-end-step enqueues supported triggers from both players' battlefields

## Rules Reference

- 101.4
- 503
- 513
- 603

## Rules Support Statement

This slice widens the supported phase-trigger subset to all relevant battlefield permanents in the current two-player engine. It still does not imply a generic delayed-trigger engine.
