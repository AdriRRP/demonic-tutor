# Slice Proposal — Destroy Target Creature Foundation

## Goal

Support a targeted spell effect that destroys a creature directly instead of dealing damage.

## Why This Slice Exists Now

The current spell-effect subset is still centered on damage. `Destroy target creature` is the next high-value effect because it is simple, common, and semantically distinct from lethal damage.

## Supported Behavior

- a supported spell may target a legal creature
- on resolution, the creature is destroyed and moved to graveyard through the owned removal corridor

## Invariants / Legality Rules

- the spell requires exactly one legal creature target
- target legality is shared between cast and resolution
- destruction applies only if the target remains legal on resolution

## Out of Scope

- regeneration
- indestructible
- destroy effects on noncreatures

## Domain Impact

- extend supported spell-resolution profiles with explicit destroy semantics
- reuse existing graveyard movement and SBA-safe post-resolution review

## Ownership Check

This belongs to aggregate-owned stack resolution and creature-state legality.

## Documentation Impact

- current-state
- glossary if `destroy` needs explicit wording
- implemented slice doc

## Test Impact

- unit coverage for cast, resolve, and target-loss paths
- executable BDD for one positive corridor

## Rules Reference

- 114
- 608.2b
- 701.7
- 704

## Rules Support Statement

This slice adds direct creature destruction to the supported spell-effect subset. It does not imply regeneration, indestructible, or broader destroy semantics.
