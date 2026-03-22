# Slice Proposal — Trample Assigns Excess Damage To Player

## Goal

Support `Trample` so excess combat damage from a blocked attacker can be assigned to the defending player.

## Why This Slice Exists Now

The repo already supports blocked combat damage with one blocker per attacker. That simplification makes `Trample` a good next combat keyword because the assignment problem is still small.

## Supported Behavior

- a blocked attacker with `Trample` assigns lethal damage to its blocker and remaining damage to the defending player

## Invariants / Legality Rules

- the current combat model still assumes at most one blocker per attacker
- lethal-to-blocker must be satisfied before excess reaches the player

## Out of Scope

- multiple blockers
- damage prevention
- deathtouch interactions

## Domain Impact

- extend the closed keyword set with `Trample`
- refine combat-damage assignment in the supported blocker model

## Ownership Check

Combat damage assignment remains aggregate-owned.

## Documentation Impact

- current-state
- glossary
- implemented slice doc

## Test Impact

- unit and BDD coverage for excess damage assignment

## Rules Reference

- 510
- 702.19

## Rules Support Statement

This slice adds `Trample` only inside the current one-blocker combat model.
