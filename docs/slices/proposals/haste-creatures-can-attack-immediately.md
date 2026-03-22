# Slice Proposal — Haste Creatures Can Attack Immediately

## Goal

Support `Haste` so a creature can attack on the turn it enters under its controller's control.

## Why This Slice Exists Now

The repo already models summoning sickness and keyword abilities. `Haste` is the smallest next keyword with immediate gameplay value and low architectural risk.

## Supported Behavior

- a creature with `Haste` may be declared as an attacker on the turn it entered the battlefield

## Invariants / Legality Rules

- the exception applies only to attacking and relevant tap abilities if those become supported
- non-haste creatures keep the current summoning-sickness restriction

## Out of Scope

- granting haste temporarily
- activated tap abilities unless directly required

## Domain Impact

- extend the current closed keyword set with `Haste`
- refine attacker-legality checks against summoning sickness

## Ownership Check

Combat legality remains aggregate-owned.

## Documentation Impact

- current-state
- glossary
- implemented slice doc

## Test Impact

- unit and BDD coverage for immediate attack legality

## Rules Reference

- 302.6
- 508
- 702.10

## Rules Support Statement

This slice adds `Haste` only for the currently supported attack-legality model.
