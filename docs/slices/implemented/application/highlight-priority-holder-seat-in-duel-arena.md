# Highlight Priority Holder Seat In Duel Arena

## Goal

Make the current priority holder visually obvious in the hot-seat arena without turning the table into a dashboard.

## Why This Slice Existed Now

The duel arena is already playable, but reading priority from header badges alone keeps too much cognitive load on the player. The table itself should show whose interaction window is live.

## Supported Behavior

- the seat that currently holds priority now receives a table-level plasma highlight
- the bottom active seat uses an electric blue priority glow
- the opposing top seat uses a red priority glow when it holds priority
- the effect stays subtle enough to preserve battlefield readability while still making the current actor obvious

## Out Of Scope

- gameplay-rule changes around priority ownership
- animation timelines for passing priority
- card-level selection highlighting

## Rules Support Statement

This slice does not add new Magic rules.

It makes the already supported priority state easier to read in the browser client.
