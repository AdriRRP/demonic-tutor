# Limited Set Capability Matrix

This document is the canonical authoring matrix for the first curated limited set.

It describes what a set designer may currently express through `LibraryCard` without exceeding the supported subset.

It does not claim broader Magic support than the current engine actually implements.

---

## Purpose

Use this matrix when:

- authoring curated limited-set cards
- reviewing whether a proposed card fits the current engine
- deciding whether a design needs a new slice instead of squeezing through an unsupported shape

The matrix is downstream from code:

- authored cards are classified through `SupportedLimitedSetCardProfile`
- unsupported authored shapes are rejected when `PlayerLibrary` input is loaded

---

## Supported Base Families

### Land

- supported profile:
  - monocolor mana land with one explicit mana ability
- allowed authored components:
  - `activated_mana_ability`
- rejected examples:
  - land with spell resolution
  - land with triggered ability
  - land creature

### Creature

- supported profile:
  - creature body plus one supported subfamily
- allowed authored components:
  - power/toughness
  - supported keyword set
  - one supported activated ability, or
  - one supported triggered ability, or
  - one supported controller static effect
- current supported keywords:
  - `Flying`
  - `Reach`
  - `Haste`
  - `Vigilance`
  - `Menace`
  - `Trample`
  - `FirstStrike`
  - `DoubleStrike`
  - `Lifelink`
  - `Hexproof`
  - `Indestructible`
  - `Defender`
  - `Deathtouch`
- rejected examples:
  - creature with spell resolution profile
  - creature with both activated and triggered ability
  - creature with static anthem plus another activated or triggered subfamily

### Instant / Sorcery

- supported profile:
  - spell card with one explicit supported resolution profile
- allowed authored components:
  - card type `Instant` or `Sorcery`
  - one supported `SupportedSpellRules`
  - optional supported casting rules already exercised by the engine
- supported resolution families:
  - damage to player / creature / any target / combat-context creature subsets
  - gain life / lose life
  - modal choose-one life spell
  - `tap target creature`
  - `untap target creature`
  - `target creature can't block this turn`
  - `+1/+1 counter` placement
  - bounded distribution of two `+1/+1` counters among up to two creatures
  - `loot`
  - `rummage`
  - `scry 1`
  - `surveil 1`
  - single and multiple vanilla token creation
  - one keyworded creature-token profile
  - destroy / exile / bounce supported target subsets
  - chosen-card discard
  - mill
  - creature recursion to hand
  - instant-or-sorcery recursion to hand
  - bounded self-graveyard cast with exile on resolution
  - reanimate target creature card from own graveyard
  - counter target spell
- rejected examples:
  - spell card with activated ability
  - spell card with triggered ability
  - spell card with attachment or controller-static metadata
  - `scry` or `surveil` amounts beyond the explicit supported amount

### Artifact

- supported profile:
  - noncreature artifact with one supported activated or triggered subfamily
- allowed authored components:
  - one supported activated ability, or
  - one supported triggered ability
  - optional supported casting rules
- rejected examples:
  - artifact with spell resolution profile
  - artifact with both activated and triggered subfamilies
  - artifact creature in the current curated contract

### Enchantment

- supported profiles:
  - noncreature aura that enchants a creature, or
  - noncreature static enchantment
- allowed authored components:
  - for aura:
    - `AttachToTargetCreature`
    - `EnchantCreature`
    - optional attached stat boost
    - optional attached combat restriction
  - for static enchantment:
    - controller static effect
  - optional supported casting rules
- rejected examples:
  - enchantment with activated ability
  - enchantment with triggered ability
  - enchantment creature in the current curated contract
  - enchantment mixing aura attachment and controller-static anthem

### Planeswalker

- supported profile:
  - planeswalker with initial loyalty and one explicit loyalty ability
- allowed authored components:
  - `initial_loyalty`
  - one supported activated loyalty ability
  - optional supported casting rules
- rejected examples:
  - planeswalker without loyalty ability
  - planeswalker with triggered ability
  - planeswalker with spell resolution profile

---

## Supported Casting Rules

The current curated contract allows only the casting permissions already exercised by the engine:

- `ActivePlayerEmptyMainPhaseWindow`
- `OpenPriorityWindow`
- `OpenPriorityWindowDuringOwnTurn`
- `CastFromOwnGraveyard`
- `ExileOnResolutionWhenCastFromOwnGraveyard`

If a design needs another timing or permission model, it needs a new slice.

---

## Authoring Boundaries

The following are intentionally outside the current curated-set contract:

- control-changing effects
- copy effects
- prevention and replacement effects
- tutor and shuffle-heavy search effects
- multiplayer semantics
- generic distribution systems beyond the bounded explicit counter slice
- broad layers/timestamp/dependency coverage beyond the currently explicit static subsets
- authored hybrid or cross-family card shapes not listed above

---

## Operational Rule

For the current curated limited set:

- if a card shape is not representable by this matrix, it is not yet a supported card for authoring
- the right next step is a slice proposal, not a looser interpretation of the existing catalog
