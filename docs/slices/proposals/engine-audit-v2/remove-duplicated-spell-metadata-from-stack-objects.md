# Slice Proposal — Remove Duplicated Spell Metadata From Stack Objects

## Goal

Make `SpellOnStack` carry one canonical spell representation instead of storing stack-local duplicates of metadata already present in the spell snapshot.

## Why This Slice Exists Now

The stack refactor already moved away from full card runtime values, but `SpellOnStack` still duplicates `source_card_id`, `card_type`, and supported spell rules next to the snapshot.

This slice exists to:

1. reduce stack payload size
2. eliminate internal duplicate sources of truth
3. keep stack objects semantically minimal before future growth

## Supported Behavior

- stack behavior remains unchanged
- spell resolution still has access to the metadata it needs
- stack objects carry one canonical spell representation

## Invariants / Legality Rules

- no observable event payload changes
- no gameplay legality changes
- stack resolution semantics remain deterministic

## Out of Scope

- new stack object families
- event contract redesign
- broader stack rules

## Domain Impact

- stack object modeling becomes smaller and less redundant

## Documentation Impact

- this slice document

## Test Impact

- full stack, targeting, and resolution regression coverage remains green

## Rules Reference

- none beyond current stack behavior

## Rules Support Statement

This slice is a stack-model refactor only. It does not expand Magic rules support.
