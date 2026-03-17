# Rules Map — DemonicTutor

This document maps implemented repository behavior to the official Magic Comprehensive Rules.

The goal is traceability between the domain model, features, slices, and the rule system.

This file is a routing aid.
It is not a literal restatement of the rules text.

---

## StartGame

- 103.1
- 103.2

---

## DrawOpeningHands

- 103.3

---

## Mulligan

- 103.4

---

## PlayLand

- 305.1
- 305.2
- 305.3

---

## Tap Lands for Mana

- 605.1
- 605.3a

---

## AdvanceTurn

- 500–514

---

## Turn Phases

- 501
- 502
- 503
- 504
- 505

---

## DrawCard

- 121.1
- 121.2

---

## Lose On Empty Draw

- 121.4
- 704.5b

---

## Lose On Zero Life

- 104.3b
- 704.5a

---

## Zero-Toughness Creature Dies

- 704.5f

---

## Cleanup Hand Size Discard

- 514.1
- 514.1a

---

## Cast Non-Land Spells

- 601.1
- 601.2

---

## Pay Mana Cost

- 202.1
- 202.1a

---

## Player Life

- 118.1
- 118.2

---

## Turn Number

- 500

---

## Pilot Features

The initial Gherkin pilot currently targets these rule areas:

- `features/turn-flow/turn_progression.feature`
- `features/spells/cast_creature_spell.feature`
- `features/combat/combat_damage_marking.feature`
- `features/combat/creature_destruction.feature`
- `features/turn-flow/cleanup_damage_removal.feature`
- `features/turn-flow/cleanup_hand_size_discard.feature`
- `features/turn-flow/lose_on_empty_draw.feature`
- `features/life/lose_on_zero_life.feature`
- `features/state-based-actions/zero_toughness_creature_dies.feature`

---

## Creature Destruction

- 704
- 704.5g
