# Bind Each Browser Instance To One Remote Seat

## Goal

Turn the remote duel into a true one-seat-per-device experience by binding each browser instance to exactly one player seat.

## Why This Slice Existed Now

Remote play stopped being honest as long as the browser session still trusted incoming `player_id` values as if both seats belonged to the same local surface.

## Supported Behavior

- the host browser remains bound to the first viewer-scoped seat
- the remote peer browser remains bound to the opposing seat
- peer-issued commands are rejected locally if they target the wrong seat
- the authoritative host also rejects remote commands that try to act as the host seat
- seat ownership stays explicit through browser-session state instead of leaking into the `Game` aggregate

## Out Of Scope

- spectators
- dynamic seat switching
- private-hand scoping across remote payloads
- host migration

## Rules Support Statement

This slice does not widen Magic rules support.

It only makes remote browser ownership honest around the already-supported public command corridor.

## Constraints And Honesty Notes

- seat ownership lives in browser session orchestration, not in the domain model
- the authoritative host still executes all rules
- remote browsers may still receive broader public payloads than they should until private-view scoping lands in the next slice
