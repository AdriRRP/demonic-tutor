# Slice Name

Support Sacrifice As Activation Cost

## Goal

Allow supported activated abilities to sacrifice their source or another supported permanent as part of the activation cost.

## Why This Slice Exists Now

Sacrifice costs unlock many archetypes and make both creatures and artifacts much more usable. They also exercise an important rules distinction: costs happen before resolution.

## Supported Behavior

- allow a supported activated ability to sacrifice its source as a cost
- allow a narrow supported subset to sacrifice another permanent as a cost when explicitly modeled
- move the sacrificed permanent immediately to graveyard before the ability resolves

## Invariants / Legality Rules

- the sacrifice is part of activation cost payment, not the resolution effect
- if the sacrifice cannot be paid, the ability cannot be activated
- the sacrificed permanent is gone before players can respond to the ability

## Out of Scope

- sacrificing from zones other than battlefield
- complex filter clauses like “sacrifice another artifact or creature”
- recursive sacrifice loops requiring trigger ordering decisions

## Domain Impact

### Aggregate Impact

- extend activation-cost payment semantics with battlefield-to-graveyard sacrifice

## Ownership Check

This belongs to the `Game` aggregate because cost payment and zone movement as activation costs are gameplay legality rules.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`
- this slice doc

## Test Impact

- sacrifice source as activation cost
- reject activation when no legal sacrifice exists
- sacrificed permanent is already gone before resolution

## Rules Reference

- 602
- 701.17

## Rules Support Statement

This slice adds a narrow explicit sacrifice-cost corridor. It does not imply broad sacrifice-deck support or complex sacrifice-choice grammars.

