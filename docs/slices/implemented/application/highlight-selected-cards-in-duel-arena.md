# Highlight Selected Cards In Duel Arena

## Goal

Make the currently selected card visually obvious in the duel arena, whether the player is hovering a hand card, opening a battlefield action menu, or inspecting a card in detail.

## Why This Slice Existed Now

The arena already exposed legal actions and card inspection, but the table still made players infer which card was the live focus. A shared selected-card highlight reduces that ambiguity and keeps interaction closer to a physical card game feel.

## Supported Behavior

- the hovered hand card now receives a dedicated selected-card highlight in addition to the hand zoom gesture
- the battlefield card whose action bubble is open stays visibly selected on the table
- opening card inspection preserves one selected source card instead of leaving the table visually neutral
- selected-card highlighting is separate from the existing legal-action highlight so “playable” and “currently chosen” do not blur together

## Out Of Scope

- target-line rendering across the battlefield
- drag-path animations between zones
- new gameplay actions or legality rules

## Rules Support Statement

This slice does not add new Magic rules.

It adds a clearer visual affordance for already supported interaction state in the browser client.
