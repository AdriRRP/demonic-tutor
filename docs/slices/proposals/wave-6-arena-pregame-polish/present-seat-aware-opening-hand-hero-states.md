# Present Seat-Aware Opening-Hand Hero States

## Goal

Turn the current truthful setup overlay into a more legible pregame presentation that clearly distinguishes “you decide now” from “the other player is deciding” while keeping the hand visible.

## Why This Slice Exists Now

The browser already supports honest remote `Setup`, repeated London mulligans, and explicit bottom selection. The next gap is not rules support but clarity and mood: the pregame state still reads as a service overlay more than a game client. The smallest next step is to elevate the hero state and seat-aware copy without changing the underlying flow.

## Supported Behavior

- the pregame overlay presents stronger viewer-scoped hero states for deciding versus waiting
- the starting player reveal and current decision holder read clearly at a glance
- keep and mulligan controls feel anchored to the active pregame state instead of generic modal buttons

## Invariants / Legality Rules

- only the current decision holder may act
- both players still see only their own opening hand in clear
- the pregame overlay never obscures the need to bottom cards before keeping

## Out Of Scope

- animated coin flips
- rules changes to mulligan or who starts
- card recommendation heuristics

## Domain Impact

### Aggregate Impact

- none

## Ownership Check

This behavior belongs to the browser client.

It presents already-supported setup state more clearly but does not alter gameplay authority or rules.

## Documentation Impact

- `docs/architecture/web-client.md`
- `apps/web/README.md`
- this slice document

## Test Impact

- deciding and waiting states render different copy and action affordances
- keep and mulligan remain available only to the active chooser

## Rules Reference

- 103.2 — Starting player selection
- 103.5 — Opening hand decisions

## Rules Support Statement

This slice does not add new mulligan rules.

It only improves the browser presentation of the already-supported remote setup flow.
