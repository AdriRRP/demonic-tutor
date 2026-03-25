# Rules Notes — Keyword Abilities

## Purpose

Summarize the rule areas DemonicTutor currently uses to model keyword abilities on creatures.

This is a repository-owned interpretation note, not a copy of the Comprehensive Rules.

## Relevant Rules

- 702.2 — Flying
- 702.2b — "A creature with flying can block only creatures with flying."
- 702.2c — "A creature can block a creature with flying only if it has flying or reach."
- 702.2d — Reach

## Current DemonicTutor Interpretation

### Flying

- a creature with Flying cannot be blocked except by creatures with Flying or Reach
- Flying is set at creature creation time
- Flying does not affect attack declarations
- Flying does not grant any additional combat advantages beyond blocking restrictions

### Reach

- a creature with Reach can block creatures with Flying
- Reach does not grant Flying to the creature
- Reach is set at creature creation time
- A creature can have both Flying and Reach (redundant but legal)

### Blocking Legality

- When a defender declares blockers, the game validates that each blocker can legally block its assigned attacker
- A blocker can block a flying attacker if and only if the blocker has Flying or Reach
- A blocker can block a non-flying attacker without restriction from keywords
- A creature with `Menace` cannot be blocked by exactly one creature in the current combat model

### Lifelink

- a creature with `Lifelink` causes its controller to gain life equal to combat damage it deals in the supported subset
- this currently applies to damage dealt to players and to creatures during combat
- life gain is resolved in the same combat-damage corridor before SBA review completes

### Hexproof

- a creature with `Hexproof` cannot be targeted by opponents in the supported targeted-spell subset
- the creature's controller can still target that creature with supported spells
- cast validation and resolution revalidation share the same `Hexproof` legality corridor

### Indestructible

- a creature with `Indestructible` is not destroyed by lethal damage in the current SBA subset
- `destroy target creature` does not move an indestructible creature to the graveyard
- `Indestructible` does not stop a creature from dying to zero toughness

## Out of Scope

- Trample (excess damage to defending player)
- First Strike (damage in first combat damage step)
- Double Strike (damage in both steps)
- Deathtouch (any damage is lethal)
- Vigilance (attack without tapping)
- Haste (no summoning sickness)
- Protection (cannot be targeted, damaged, enchanted, or blocked by specific colors)
- Shroud (cannot be targeted)
- Keyword counters
- Keywords on non-creature permanents

## Related Features

- `features/combat/keyword_abilities.feature`
