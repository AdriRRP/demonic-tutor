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

## Upkeep Priority Window

- 117
- 503

---

## Draw Priority Window

- 117
- 504

---

## End Step Priority Window

- 117
- 507

---

## DrawCard

- 121.1
- 121.2

---

## Draw Multiple Cards

- 121.1
- 121.2
- 121.4

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

## State-Based Actions Review

- 704
- 704.5a
- 704.5f
- 704.5g

---

## Stack Foundation

- 117
- 405

---

## Cast Instant In Upkeep Window

- 117
- 503
- 601
- 608

---

## Cast Second Instant In Upkeep Window

- 117
- 503
- 601
- 608

---

## Respond In Upkeep Window

- 117
- 503
- 601
- 608

---

## Respond In Draw Window

- 117
- 504
- 601
- 608

---

## Respond With Second Instant In Draw Window

- 117
- 504
- 601
- 608

---

## Respond In First Main Window

- 117
- 505
- 601
- 608

---

## Respond With Second Instant In First Main Window

- 117
- 505
- 601
- 608

---

## Respond In Second Main Window

- 117
- 505
- 601
- 608

---

## Respond With Second Instant In Second Main Window

- 117
- 505
- 601
- 608

---

## Respond In End Step Window

- 117
- 507
- 601
- 608

---

## Respond With Second Instant In End Step Window

- 117
- 507
- 601
- 608

---

## Respond In Beginning Of Combat Window

- 117
- 506
- 601
- 608

---

## Respond After Attackers

- 117
- 508
- 601
- 608

---

## Respond After Blockers

- 117
- 509
- 601
- 608

---

## Respond After Combat Damage

- 117
- 510
- 511
- 601
- 608

---

## Respond With Second Instant Spell

- 117
- 601
- 608

---

## Respond With Second Instant In Upkeep Window

- 117
- 503
- 601
- 608

---

## Cast Instant In Draw Window

- 117
- 504
- 601
- 608

---

## Cast Second Instant In Draw Window

- 117
- 504
- 601
- 608

---

## Cast Instant In First Main Window

- 117
- 505
- 601
- 608

---

## Cast Second Instant In First Main Window

- 117
- 505
- 601
- 608

---

## Cast Instant In Second Main Window

- 117
- 505
- 601
- 608

---

## Cast Second Instant In Second Main Window

- 117
- 505
- 601
- 608

---

## Cast Instant In End Step Window

- 117
- 507
- 601
- 608

---

## Cast Second Instant In End Step Window

- 117
- 507
- 601
- 608

---

## Cast Instant In Beginning Of Combat Window

- 117
- 506
- 601
- 608

---

## Cast Second Instant In Beginning Of Combat Window

- 117
- 506
- 601
- 608

---

## Cast Instant After Attackers

- 117
- 508
- 601
- 608

---

## Cast Second Instant After Attackers

- 117
- 508
- 601
- 608

---

## Cast Instant After Blockers

- 117
- 509
- 601
- 608

---

## Cast Second Instant After Blockers

- 117
- 509
- 601
- 608

---

## Cast Instant After Combat Damage

- 117
- 510
- 511
- 601
- 608

---

## Cast Second Instant After Combat Damage

- 117
- 510
- 511
- 601
- 608

---

## Beginning Of Combat Priority Window

- 117
- 506
- 507
- 508

---

## Post Combat Damage Priority Window

- 117
- 510
- 511

---

## Cleanup Hand Size Discard

- 514.1
- 514.1a

---

## Cast Spells

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
- `features/turn-flow/draw_multiple_cards.feature`
- `features/spells/cast_creature_spell.feature`
- `features/combat/combat_damage_marking.feature`
- `features/combat/combat_priority_windows.feature`
- `features/combat/beginning_of_combat_priority_window.feature`
- `features/combat/post_combat_damage_priority_window.feature`
- `features/combat/single_blocker_per_attacker.feature`
- `features/combat/creature_destruction.feature`
- `features/turn-flow/cleanup_damage_removal.feature`
- `features/turn-flow/cleanup_hand_size_discard.feature`
- `features/turn-flow/lose_on_empty_draw.feature`
- `features/life/lose_on_zero_life.feature`
- `features/state-based-actions/zero_toughness_creature_dies.feature`
- `features/stack/stack_foundation.feature`
- `features/stack/respond_with_instant_spell.feature`
- `features/stack/cast_instant_in_upkeep_window.feature`
- `features/stack/cast_second_instant_in_upkeep_window.feature`
- `features/stack/cast_instant_in_draw_window.feature`
- `features/stack/cast_second_instant_in_draw_window.feature`
- `features/stack/cast_instant_in_second_main_window.feature`
- `features/stack/cast_second_instant_in_second_main_window.feature`
- `features/stack/cast_instant_in_end_step_window.feature`
- `features/stack/cast_second_instant_in_end_step_window.feature`
- `features/stack/cast_instant_in_beginning_of_combat_window.feature`
- `features/stack/cast_second_instant_in_beginning_of_combat_window.feature`
- `features/stack/cast_instant_after_attackers.feature`
- `features/stack/cast_second_instant_after_attackers.feature`
- `features/stack/cast_instant_after_blockers.feature`
- `features/stack/cast_second_instant_after_blockers.feature`
- `features/stack/cast_instant_after_combat_damage.feature`
- `features/stack/cast_second_instant_after_combat_damage.feature`
- `features/turn-flow/upkeep_priority_window.feature`
- `features/turn-flow/draw_priority_window.feature`
- `features/turn-flow/main_phase_priority_window.feature`
- `features/turn-flow/end_step_priority_window.feature`

---

## Creature Destruction

- 704
- 704.5g
