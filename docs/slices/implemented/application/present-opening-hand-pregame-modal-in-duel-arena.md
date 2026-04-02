# Present Opening-Hand Pregame Modal In Duel Arena

## Goal

Make the browser arena begin like a real duel: pick the starting player at random, then run the opening-hand keep/mulligan flow before the battlefield enters the first main phase.

## Why This Slice Existed Now

The wasm-backed duel arena had become playable, but it still skipped straight into `FirstMain`. That made the match feel less like a real card game and hid one of the few pregame rules the engine already supported. The next smallest valuable step was to expose that setup flow as a game-start modal instead of jumping over it.

## Supported Behavior

- a new duel no longer advances directly to `FirstMain`
- the wasm browser adapter chooses the starting player at random for each new duel or reset
- the duel remains in `Setup` while the opening-hand modal is active
- players decide in turn order starting with the chosen starter
- each player may either keep the current opening hand or take the one mulligan supported by the current engine slice
- after a mulligan, that same player must explicitly keep the new hand before the next player decides
- once both players keep, the adapter advances the underlying game to the normal first main priority window
- the same flow works both in hot-seat and in the same-origin two-window local room

## Out Of Scope

- full London Mulligan support with repeated rounds and bottoming cards
- secure remote hidden-information handling across browsers
- moving keep/mulligan into the shared public gameplay contract for every client
- animation polish beyond the modal and seat-handoff flow

## Rules Support Statement

This slice does not expand the mulligan rules themselves.

It exposes the already-supported simplified setup/mulligan behavior through the browser adapter and the web client.

## Architectural Notes

- the pregame controller lives in `src/interfaces/web/wasm.rs`
- `Keep` remains a browser-adapter orchestration step because the domain currently has no canonical keep command
- `Mulligan` still executes against the real `GameService` lifecycle API and persists the canonical domain event
- the adapter blocks battlefield/public gameplay commands until pregame completes, so the duel cannot skip `Setup`

## Constraints And Honesty Notes

- the active rules slice still supports only one mulligan per player
- the random starting player is chosen locally inside the browser-owned authoritative session
- the same-origin peer window remains a trusted local participant, not a secure remote client
