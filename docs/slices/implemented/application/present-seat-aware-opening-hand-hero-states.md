# Present Seat-Aware Opening-Hand Hero States

## Goal

Turn the truthful remote `Setup` overlay into a clearer pregame hero state that immediately tells each device whether it is deciding, bottoming, or waiting while keeping the opening hand visible.

## Why This Slice Existed Now

The browser already supported remote `Setup`, repeated London mulligans, and explicit bottom selection from the visible hand fan. The next gap was legibility: the overlay still felt like a service panel instead of a game pregame state. This slice promotes the viewer-scoped pregame state into a stronger hero presentation without changing gameplay authority or mulligan legality.

## Supported Behavior

- the overlay now renders stronger seat-aware hero states for deciding, bottoming, and waiting
- each device can see at a glance whether it goes first or second and whether it is the current chooser
- keep and mulligan controls now feel anchored to the active pregame state instead of generic modal buttons
- both seat states remain visible in compact status cards without exposing hidden information

## Out Of Scope

- animated coin flips
- mulligan heuristics or recommendations
- new mulligan rules support
- Arena-perfect presentation parity

## Rules Support Statement

This slice does not add new Magic rules.

It only improves how the browser presents the already-supported remote opening-hand flow.

