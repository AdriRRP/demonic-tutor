# Slice Name

`ExposeReadonlyLegalActionQueriesForPublicContract`

## Goal

Introduce explicit read-only legality queries owned by the gameplay domain so the public gameplay contract can derive legal actions without cloning the full `Game` and speculatively executing commands.

## Why This Slice Exists Now

`0.7.0` is the first release where the UI can start against a stable public contract. That contract should not discover actions by simulating full mutations per candidate card, because that couples UI polling cost to aggregate clone cost and keeps legality discovery implicit.

## Supported Behavior

- expose read-only legality queries for the currently supported public-action subset
- allow the public gameplay projection to ask whether a land is playable, a mana source is tappable, a spell is castable, an ability is activatable, and a creature can attack
- preserve the current partial-success semantics where a card may be legal but still require explicit target or choice input
- remove speculative `Game` cloning from the public legal-action projection path

## Invariants / Legality Rules

- the `Game` aggregate remains the authority on legality
- legality queries must not mutate runtime state
- legality queries must not widen rules support beyond what the canonical commands already enforce
- missing target or missing explicit choice may still count as “legally actionable with further input”

## Out of Scope

- changing the command model
- adding new gameplay actions
- introducing UI-specific legality concepts into the domain language
- caching or incremental memoization of legal actions

## Domain Impact

### Aggregate Impact
- `Game` gains read-only legality-query entrypoints for the currently supported action surface

### Commands
- no new commands

### Events
- no new events

## Ownership Check

This belongs to the `Game` aggregate because gameplay legality is aggregate-owned truth. The public contract may project that truth, but should not rediscover it by simulating mutations.

## Documentation Impact

- this slice document
- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`

## Test Impact

- public legal actions stay unchanged for representative board states
- public legal actions no longer require aggregate cloning in the projection corridor
- spell and ability candidates still surface when they only lack target or explicit choice input

## Rules Reference

- 117 — timing and priority in the currently supported subset
- 601.2 — spell casting legality, simplified to the supported engine subset
- 602.2 — activated ability legality, simplified to the supported engine subset
- 508.1 — declare attackers legality, simplified to the supported engine subset

## Rules Support Statement

This slice does not add new Magic mechanics. It makes the existing legality model explicit and queryable for the current supported subset.

## Open Questions

- none
