# Expose Canonical Game Snapshot Projection

## Goal

Expose one stable client-facing snapshot of the current game so UI code can render the supported subset without reading raw aggregate internals.

## Why This Slice Existed Now

The roadmap to a playable limited set needs a point where frontend work can begin safely. A canonical public snapshot is the first half of that bridge.

## Supported Behavior

- project game id, active player, phase, turn number, terminal status, and priority
- project player public state
- expose hand counts instead of hidden opponent hand contents
- expose battlefield, graveyard, exile, and stack as stable public views
- expose visible permanent state such as tapping, combat flags, loyalty, token state, and supported keywords

## Out Of Scope

- secret per-player hand rendering policies
- animation timelines
- replay event ordering guarantees
- multiplayer visibility rules

## Rules Support Statement

This slice does not add new Magic rules.

It adds a truthful public read model over the currently supported subset.
