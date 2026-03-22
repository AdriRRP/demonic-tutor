# Slice Proposal — Targeted Gain Life Spell

## Goal

Support a targeted spell that causes a player to gain life.

## Why This Slice Exists Now

The repo already has shared life-change semantics and explicit targeted life effects outside the stack corridor. A targeted gain-life spell is a small way to connect those semantics into richer spell gameplay.

## Supported Behavior

- a supported spell may target a legal player
- on resolution, the target gains the explicit amount of life

## Invariants / Legality Rules

- the spell requires one legal player target
- life change reuses shared player-life semantics and events

## Out of Scope

- prevention
- replacement effects on life gain
- multiple targets

## Domain Impact

- extend supported spell-resolution profiles with positive life change
- reuse existing life-change semantics and events

## Ownership Check

Life totals and spell resolution are already aggregate-owned.

## Documentation Impact

- current-state
- implemented slice doc

## Test Impact

- unit coverage for cast and resolve
- executable BDD for one positive corridor

## Rules Reference

- 114
- 118
- 608.2b

## Rules Support Statement

This slice adds a targeted gain-life spell to the supported stack subset. It does not imply broader replacement or prevention rules.
