# Slice Name

Return Target Permanent To Hand

## Goal

Allow a supported spell to return a target permanent from the battlefield to its owner's hand.

## Why This Slice Exists Now

Bounce is one of the most common interaction families in real games and exercises a valuable corridor: battlefield target legality, owner-aware zone movement, and spell resolution without destruction.

## Supported Behavior

- accept a supported spell targeting a permanent on the battlefield
- validate the target permanent at cast time and resolution time
- move the targeted permanent from battlefield to its owner's hand on resolution
- preserve owner semantics even when the acting player does not own the permanent

## Invariants / Legality Rules

- only permanents on the battlefield are legal targets
- the card returns to its owner's hand, not the controller's hand
- if the target is gone on resolution, the spell does nothing

## Out of Scope

- returning multiple permanents
- self-bounce with additional costs or bonuses
- returning non-permanent cards from graveyard, exile, or stack
- replacement effects that modify the destination

## Domain Impact

### Aggregate Impact

- extend supported targeted resolution outcomes with battlefield-to-hand movement

### Entity / Value Object Impact

- extend permanent target rules beyond creature-only targeting where required

### Commands

- no new public command required; reuse `CastSpell`

### Events

- existing zone-move and spell-resolution events may need clearer hand-return outcomes

### Errors

- invalid permanent target for the supported bounce family

## Ownership Check

This belongs to the `Game` aggregate because battlefield legality, ownership-aware zone transitions, and spell resolution are aggregate responsibilities.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`
- `docs/rules/rules-map.md`
- this slice doc

## Test Impact

- return your own permanent to hand
- return an opponent permanent to its owner's hand
- reject non-battlefield target
- spell fizzles if the permanent is gone on resolution

## Rules Reference

- 110
- 114
- 608.2
- 701.16

## Rules Support Statement

This slice introduces a minimal explicit bounce corridor for **battlefield permanents only**. It does not imply broad hand-return support across all zones or full permanent subtyping rules.

