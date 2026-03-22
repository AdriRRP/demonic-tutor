# Slice Proposal — Destroy Target Creature Uses Shared Resolution Corridor

## Goal

Ensure `destroy target creature` resolves through the same owned stack, target, event, and post-resolution review corridor as the current targeted spells.

## Why This Slice Exists Now

If destruction lands as a special-case branch, later spell effects will start to fragment the core resolution model. This slice keeps the engine stable as effect diversity grows.

## Supported Behavior

- destroy-target spells emit the normal supported stack-resolution events
- the effect reuses shared legal-target revalidation and shared post-resolution review

## Invariants / Legality Rules

- no ad hoc resolution shortcut bypasses the common corridor
- target loss on resolution behaves consistently with the current targeted-spell subset

## Out of Scope

- new effect families
- triggered death handling

## Domain Impact

- refactor or harden resolution helpers without changing aggregate ownership

## Ownership Check

This remains aggregate-owned stack resolution behavior.

## Documentation Impact

- implemented slice doc only if the owned truth changes materially

## Test Impact

- unit coverage proving destroy uses the same cast/resolve corridor

## Rules Reference

- 608
- 701.7
- 704

## Rules Support Statement

This slice is about implementation integrity. It keeps direct destruction inside the shared supported resolution model rather than widening the engine through special cases.
