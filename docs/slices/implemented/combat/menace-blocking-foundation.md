# Slice Name

Menace Blocking Foundation

## Goal

Support `Menace` as a combat blocking restriction in the current two-player combat model.

## Why This Slice Exists Now

The combat corridor already supports multiple blockers per attacker and ordered blocker groups. `Menace` becomes high-value once the engine can express its minimum-two-blockers rule without another combat redesign.

## Supported Behavior

- a creature with `Menace` cannot be blocked by exactly one creature
- a creature with `Menace` may remain unblocked
- a creature with `Menace` may be blocked by two or more legal blockers

## Invariants / Legality Rules

- `Menace` is checked during `DeclareBlockers`
- existing flying/reach legality still applies to each blocker individually
- `Menace` does not change combat damage assignment once blocking is legal

## Out of Scope

- costs or effects that force additional blockers
- multiplayer-specific “can’t be blocked except by” interactions
- dynamic keyword gain or loss outside the supported keyword subset

## Rules Reference

- 702.111

## Rules Support Statement

This slice adds only the minimum-two-blockers restriction for `Menace` in the current combat model.
