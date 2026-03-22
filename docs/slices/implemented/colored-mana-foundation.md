# Slice — Colored Mana Foundation

## Goal

Introduce the first minimal colored-mana corridor without breaking the existing generic mana model.

## Supported behavior

- lands may now carry a produced mana color in the current supported subset
- the current subset supports `Forest -> Green` and `Mountain -> Red`
- spells may now carry a minimal colored mana requirement in the current supported subset
- a green instant may be paid with green mana from a `Forest`
- that same green instant is rejected if only red mana is available
- generic costs still work as before and may be paid with any currently available mana

## Current scope

This is intentionally tiny:

- only `Green` and `Red` are modeled
- only the currently exercised `Forest` and `Mountain` cases produce color
- only a minimal single-color instant cost is exercised
- hybrid costs, multicolor costs, and colored mana symbols beyond the current subset are not implemented

## Rules reference

- 106.1
- 106.4
- 601.2f

## Rules support statement

DemonicTutor now supports a minimal colored mana foundation on top of the existing transient mana pool: colored lands can add colored mana, colored spell requirements can be validated during casting, and generic costs remain payable with any available mana.
