# Support Concede, Rematch, And Seeded Game Setup Commands

## Status

Implemented

## Goal

Support the minimum session loop needed for repeated real UI playtests.

## Scope

- add an explicit `concede` command that ends an active game with a dedicated `Conceded` game-end reason
- expose deterministic seeded public game setup that starts a game, shuffles deck contents from one seed, deals opening hands, and returns the public session envelope
- expose a public rematch helper that reuses the same seeded setup with a new `game_id`

## Out Of Scope

- in-game shuffle mechanics
- sideboarding
- match score tracking
- automatic rematch persistence derived from previously stored setup state

## Notes

- the current seeded setup is an application-layer session helper over the existing lifecycle corridor, not a new gameplay mechanic inside the aggregate
- `rematch` currently reuses the same authored card pools and seed material with a new game id so UI playtests can restart deterministically
