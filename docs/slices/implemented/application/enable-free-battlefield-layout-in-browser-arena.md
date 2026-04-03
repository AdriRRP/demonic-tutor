# Enable Free Battlefield Layout In Browser Arena

## Goal

Let players arrange permanents already on the battlefield freely inside their own half of the arena, so the table starts to feel like a physical play surface instead of a fixed row of cards.

## Why This Slice Existed Now

After the duel HUD, zone piles, and hidden opponent hand were already reading as graphical tabletop objects, the battlefield itself was still locked into a presentation that looked more like UI scaffolding than a real table. The next smallest valuable step was to turn battlefield placement into local presentation state with drag-and-drop, while keeping it completely outside the Rust gameplay model.

## Supported Behavior

- battlefield permanents now render on an absolute-position surface instead of a fixed strip
- battlefield cards are slightly smaller so a fuller board fits more comfortably in the available play area
- the local player's battlefield cards can be dragged to arbitrary positions inside their seat's half of the table
- newly appeared permanents get a stable default placement before the player rearranges them
- the battlefield still accepts legal hand-to-battlefield drags for simple plays
- click-based battlefield actions and inspect detail continue to work on top of the freeform layout

## Out Of Scope

- synchronizing battlefield positions between multiple browser windows
- changing combat legality, target rules, or any Rust-owned gameplay semantics
- adding animation systems for movement beyond the drag interaction itself
- letting one player reorganize the opponent seat's battlefield

## Rules Support Statement

This slice does not add new Magic rules.

It changes only the local browser presentation of already-supported battlefield permanents.
