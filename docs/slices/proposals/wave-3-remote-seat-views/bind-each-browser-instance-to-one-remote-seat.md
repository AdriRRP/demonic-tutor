# Slice Name

Bind Each Browser Instance To One Remote Seat

## Goal

Turn the remote duel into a true one-seat-per-device experience by binding each browser instance to exactly one player seat.

## Why This Slice Exists Now

The current client still carries local-seat assumptions from same-window or same-origin play. Remote play should not reuse those abstractions implicitly, because they blur ownership and player intent.

## Supported Behavior

- one browser instance is bound to one player seat
- local gameplay controls only target that seat
- the opponent seat remains visible but non-local
- role and seat ownership stay explicit in the remote session state

## Invariants / Legality Rules

- a client cannot emit gameplay commands as the other seat
- local seat binding stays outside the `Game` aggregate
- seat ownership is transport/session state, not domain state

## Out of Scope

- spectators
- dynamic seat switching mid-game
- host migration

## Domain Impact

### Aggregate Impact

- none

### Commands

- existing commands remain unchanged

## Ownership Check

This behavior belongs to browser session orchestration because it governs which client may invoke which already-existing public command.

## Documentation Impact

- `docs/architecture/web-client.md`
- `docs/slices/proposals/remote-duel-horizon.md`
- this slice document

## Test Impact

- host is bound to one seat
- peer is bound to the other seat
- a client cannot act as the remote seat

## Rules Support Statement

This slice does not widen Magic rules support.
