# Slice Name

Destroy Target Artifact Or Enchantment

## Goal

Allow supported removal spells to target and destroy artifacts or enchantments on the battlefield.

## Why This Slice Exists Now

The engine already supports creature destruction and exile. Extending interaction to common noncreature permanents gives a large gameplay payoff with relatively small model growth.

## Supported Behavior

- accept a supported spell targeting an artifact on the battlefield
- accept a supported spell targeting an enchantment on the battlefield
- destroy the targeted permanent on resolution if it remains legal
- route destruction through the existing shared graveyard movement and SBA-friendly resolution path

## Invariants / Legality Rules

- only battlefield artifacts or enchantments are legal targets
- if the target is gone on resolution, the spell does nothing
- destruction uses the owner-aware graveyard movement rules already modeled by the aggregate

## Out of Scope

- regeneration
- indestructible
- “nonartifact” or “nonenchantment” restrictions
- modal “artifact or enchantment and something else” cards

## Domain Impact

### Aggregate Impact

- extend supported target families to noncreature permanent kinds
- reuse or extend the existing destroy-on-resolution corridor

### Commands

- no new public command required; reuse `CastSpell`

### Events

- no fundamentally new public event family required

### Errors

- reject unsupported permanent kind targets

## Ownership Check

This belongs to the `Game` aggregate because it is ordinary spell legality and resolution over aggregate-owned battlefield state.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/rules/rules-map.md`
- this slice doc

## Test Impact

- destroy target artifact
- destroy target enchantment
- reject creature target for this spell family
- target gone on resolution means no effect

## Rules Reference

- 110
- 114
- 608.2
- 701.7

## Rules Support Statement

This slice adds explicit artifact/enchantment destruction for the currently modeled permanent subset. It does not imply support for indestructible, regeneration, or broader permanent interaction families.

