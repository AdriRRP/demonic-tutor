# Slice Name

Deathtouch Trample Lethal Assignment

## Goal

Treat one nonzero damage as lethal when a creature with both `Deathtouch` and `Trample` assigns combat damage through blockers.

## Why This Slice Exists Now

The engine already supports `Deathtouch`, `Trample`, and ordered blocker assignment. Without this interaction, a deathtouch trampler still overassigns damage to blockers and underassigns damage to the defending player.

## Supported Behavior

- when an attacking creature has both `Deathtouch` and `Trample`, it only needs to assign 1 nonzero damage to each blocker before assigning excess damage to the defending player in the current subset
- ordered blocker assignment still applies before damage can move on to later blockers or the defending player

## Invariants / Legality Rules

- deathtouch changes the lethal-assignment threshold for combat damage assignment
- trample still requires forward assignment through blockers in order
- the marked-damage amount remains the actual assigned damage, not the blocker's full toughness

## Out of Scope

- damage prevention and replacement effects
- indestructible and other broader damage-modification semantics not yet modeled

## Domain Impact

### Aggregate Impact

- refine the aggregate-owned combat damage-assignment corridor for the supported `Deathtouch + Trample` combination

## Ownership Check

This belongs to the `Game` aggregate because combat damage assignment and lethal-combat semantics are aggregate-owned rules.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/rules/rules-map.md`
- `docs/rules/notes/combat.md`
- this slice doc

## Test Impact

- a deathtouch trampler assigns 1 damage to a blocker and the rest to the defending player

## Rules Reference

- 510.1c
- 702.2
- 702.19

## Rules Support Statement

This slice closes the currently supported `Deathtouch + Trample` interaction for combat assignment only. It does not imply broader damage-modification completeness.
