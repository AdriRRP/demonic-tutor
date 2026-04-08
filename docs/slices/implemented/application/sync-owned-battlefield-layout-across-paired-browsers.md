# Sync Owned Battlefield Layout Across Paired Browsers

## Goal

Keep the free battlefield layout visually shared across paired browser sessions, so each player's arrangement of permanents reads as one common table instead of two drifting local presentations.

## Why This Slice Existed Now

Once the remote duel horizon was complete, the most obvious remaining mismatch between local and paired play was battlefield organization. Each browser could already drag permanents inside its own seat, but that arrangement stayed trapped in the local window. The next smallest valuable step was to treat battlefield placement as seat-owned presentation state and relay it through the existing browser session layer without teaching the Rust runtime about UI layout.

## Supported Behavior

- each seat keeps a presentation-only battlefield layout keyed by `player_id` and `card_id`
- dragging permanents inside the local battlefield half still feels local first, but the updated layout now propagates through the active session transport
- same-origin local room peers receive that layout through `BroadcastChannel`
- remote WebRTC peers receive the same layout through the host-authoritative browser transport
- the top and bottom battlefields render from the shared presentation layout instead of drifting into separate local arrangements
- battlefield layout remains scoped to the seat that owns those permanents; a browser only publishes layout updates for its bound seat

## Out Of Scope

- persisting battlefield layout in Rust or in the `Game` aggregate
- animating layout changes between browsers
- synchronizing every UI preference or dock position
- making battlefield layout authoritative gameplay state

## Rules Support Statement

This slice does not add new Magic rules.

It only synchronizes browser presentation for already-supported battlefield permanents and keeps that presentation out of the gameplay model.
