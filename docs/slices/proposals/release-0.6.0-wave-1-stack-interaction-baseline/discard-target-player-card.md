# Slice Name

Discard Target Player Card

## Goal

Allow a supported spell or effect to make a target player discard a card from hand.

## Why This Slice Exists Now

Discard is one of the highest-value missing interaction families because it connects targeting, hand zones, ownership, and graveyard movement without requiring a large new subsystem.

## Supported Behavior

- accept a supported effect targeting a player
- choose one card from that player's hand through the currently modeled explicit command/effect corridor
- move the chosen card from hand to graveyard
- emit ordinary zone-move and spell-resolution outcomes coherently

## Invariants / Legality Rules

- the target player must exist
- the discarded card must currently be in that player's hand
- the discarded card moves to its owner's graveyard
- if the targeted player or chosen card is no longer legal at resolution, the effect does not discard that card

## Out of Scope

- random discard
- “opponent reveals hand” presentation concerns
- discard by card type restriction
- multiple-card discard in one slice

## Domain Impact

### Aggregate Impact

- add a hand-to-graveyard targeted discard corridor

### Commands

- may require a second explicit choice payload beyond just target player

### Events

- existing discard and graveyard movement events should be reused where possible

### Errors

- reject missing or invalid hand-card choice

## Ownership Check

This belongs to the `Game` aggregate because hand legality, target-player resolution, and zone transitions are aggregate-owned rules.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`
- `docs/rules/rules-map.md`
- this slice doc

## Test Impact

- target player discards a chosen card from hand
- active player can force opponent discard
- reject a chosen card not in that player's hand
- spell does nothing if the chosen card is already gone on resolution

## Rules Reference

- 114
- 121
- 608.2
- 701.8

## Rules Support Statement

This slice introduces a minimal explicit discard corridor with **chosen-card discard**, not full hidden-information handling, random discard, or hand-reveal semantics.
