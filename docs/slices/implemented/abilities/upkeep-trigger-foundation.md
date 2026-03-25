# Slice Name

Upkeep Trigger Foundation

## Goal

Support the first triggered abilities that trigger at the beginning of a player's upkeep.

## Why This Slice Exists Now

The engine already has a real `Upkeep` step and priority windows there. Beginning-of-upkeep triggers are common, high-value, and a natural next step once stack-borne triggers exist.

## Supported Behavior

- detect the beginning of upkeep for the active player
- enqueue supported upkeep triggers from supported battlefield permanents across all controllers
- put those triggers on the stack before ordinary upkeep priority actions continue

## Invariants / Legality Rules

- beginning-of-upkeep triggers occur once when the game enters `Upkeep`
- supported triggers are controlled by permanents on the battlefield unless a slice says otherwise
- triggers must exist on the stack before players take ordinary upkeep actions in that window

## Out of Scope

- delayed upkeep triggers created by other spells
- cumulative upkeep
- multiple upkeep trigger ordering choices by the same player
- generic delayed upkeep triggers created by prior effects

## Domain Impact

### Aggregate Impact

- extend turn-step entry behavior to enqueue supported upkeep triggers

### Entity / Value Object Impact

- add upkeep trigger profiles where needed

## Ownership Check

This belongs to the `Game` aggregate because step entry and trigger timing are aggregate-owned turn-flow semantics.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/rules/rules-map.md`
- this slice doc

## Test Impact

- supported upkeep trigger appears before ordinary upkeep play
- no duplicate trigger while staying in the same upkeep
- trigger resolves through the stack corridor

## Rules Reference

- 503
- 603
- 117

## Rules Support Statement

This slice adds a minimal beginning-of-upkeep trigger family only. It does not imply full triggered-step support across every phase or delayed triggered abilities.
