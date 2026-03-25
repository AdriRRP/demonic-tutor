# Slice Name

Counter Target Spell

## Goal

Allow a supported instant spell to counter a target spell on the stack so reactive blue-style interaction becomes part of the playable subset.

## Why This Slice Exists Now

The current engine already has a real stack, priority passing, and spell targets. Counterspells are one of the highest-value missing interaction families because they make stack play, timing, and mana decisions much more realistic.

## Supported Behavior

- accept a supported instant spell that targets one spell on the stack
- validate that the chosen stack object is a legal spell target when cast
- remove the targeted spell from the stack on resolution if it remains legal
- move the countered spell to the correct post-counter zone in the simplified subset
- emit the normal resolution and spell outcome events coherently

## Invariants / Legality Rules

- only stack objects representing spells are legal targets
- a counterspell requires exactly one explicit stack target
- if the targeted spell is no longer on the stack on resolution, the counterspell does not counter anything
- countering a spell must not bypass normal stack resolution ordering

## Out of Scope

- countering activated or triggered abilities
- “unless its controller pays” tax counters
- uncounterable spells
- replacement effects that change where a countered spell goes

## Domain Impact

### Aggregate Impact

- extend stack-target legality and stack-object resolution for spell-on-stack targets

### Entity / Value Object Impact

- add a supported stack-target family for stack objects

### Commands

- no new public command required; reuse `CastSpell`

### Events

- existing stack and spell-resolution events may need a clearer countered outcome

### Errors

- invalid stack target for the supported counterspell family

## Ownership Check

This belongs to the `Game` aggregate because spell legality, stack targeting, and stack resolution are aggregate-owned gameplay rules.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`
- `docs/rules/rules-map.md`
- this slice doc

## Test Impact

- cast a supported counterspell targeting a spell on the stack
- countered spell leaves the stack and does not resolve
- counterspell fizzles if the target spell is already gone
- illegal cast rejects non-stack targets

## Rules Reference

- 114
- 115
- 601
- 608
- 701.5

## Rules Support Statement

This slice introduces a minimal explicit counterspell corridor for **spells on the stack only**. It does not imply full counter-magic support across abilities, replacement effects, or uncounterable text.

