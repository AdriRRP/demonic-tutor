# Slice — KeywordAbilities

## Goal

Introduce keyword abilities as creature-level modifiers that affect combat interactions. This slice adds Flying and Reach as the first keywords, enabling creatures with evasion and defining how blocking works against flying creatures.

## Status

`implemented`

## Why This Slice

This slice is the logical next step after the combat foundation because:

1. the combat system already supports declaring attackers and blockers
2. the single-blocker-per-attacker limit is already modeled
3. flying is the most common keyword ability and fundamentally changes combat
4. the domain model can represent a small closed set of supported keyword abilities without introducing a generic rules engine
5. future combat slices (trample, first strike, double strike) will follow the same pattern

---

## Supported Behavior

### Flying (CR 702.2)

- a creature with Flying cannot be blocked except by creatures with Flying or Reach
- when a defending player attempts to block a flying attacker with a non-flying, non-reach creature, the action is rejected
- flying affects only the blocking legality, not attack declarations

### Reach (CR 702.2c)

- a creature with Reach can block creatures with Flying
- Reach does not grant Flying to the creature with Reach
- Reach only enables the blocking interaction

### Creature Construction

- creatures can be created with Flying keyword during spell casting
- creatures can be created with Reach keyword during spell casting
- creature construction carries an explicit set of supported keyword abilities

---

## Invariants / Legality Rules

- Flying creatures can only be blocked by creatures with Flying or Reach
- Non-flying creatures without Reach cannot block flying attackers
- A creature can have both Flying and Reach (redundant but legal)
- Keywords are set at creature creation time and do not change during the game

---

## Out of Scope

- Trample (CR 702.2) — excess damage to defending player
- First Strike (CR 702.7) — damage in first combat damage step
- Double Strike (CR 702.4) — damage in both steps
- Lifelink (CR 702.2) — gain life equal to damage dealt
- Deathtouch (CR 702.2) — any damage is lethal
- Vigilance (CR 702.2) — attack without tapping
- Haste — no summoning sickness
- Protection — cannot be targeted, damaged, enchanted, or blocked by specific colors
- Hexproof/Shroud — cannot be targeted
- Indestructible — cannot be destroyed by damage or "destroy" effects
- Keyword counters that add or remove keywords dynamically
- Keyword abilities on non-creature permanents (Auras, Equipment)

---

## Domain Impact

### CardInstance / CreatureRuntime Changes

Store supported keywords as a closed set on creature runtime state:

```rust
enum KeywordAbility {
    Flying,
    Reach,
}

struct KeywordAbilitySet(u8);

struct CreatureRuntime {
    power: u32,
    toughness: u32,
    damage: u32,
    blocking_target: Option<CardInstanceId>,
    keywords: KeywordAbilitySet,
}
```

### CardInstance Methods

```rust
impl CardInstance {
    pub const fn has_flying(&self) -> bool { ... }
    pub const fn has_reach(&self) -> bool { ... }
    pub const fn can_block(&self, attacker: &CardInstance) -> bool { ... }
}
```

### Command Changes

- creature creation paths should accept an explicit supported-keyword set
- no new public command required for initial keywords

### Event Changes

- no new event required; keywords are creature properties set at creation

### Errors

Add new error variant:

```rust
pub enum CardError {
    // ... existing ...
    CannotBlockFlyingWithoutFlyingOrReach {
        player: PlayerId,
        blocker: CardInstanceId,
        attacker: CardInstanceId,
    },
}
```

---

## Ownership Check

This behavior belongs to the `Game` aggregate because:

- keyword abilities modify creature combat behavior
- blocking legality is enforced by the aggregate during `declare_blockers`
- creature properties must remain consistent within the authoritative game state

---

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md` — update CardInstance section
- `docs/domain/DOMAIN_GLOSSARY.md` — add Flying, Reach entries
- `docs/rules/rules-map.md` — add Keyword Abilities section
- `docs/rules/notes/keyword_abilities.md`
- `features/combat/keyword_abilities.feature`

---

## Test Impact

- a flying creature can be blocked by a creature with flying
- a flying creature can be blocked by a creature with reach
- a flying creature cannot be blocked by a creature without flying or reach
- a non-flying creature can block a non-flying attacker
- a creature with both flying and reach behaves as flying
- executable BDD coverage mirrors those blocking legality scenarios plus an unblocked flying attacker dealing player damage

---

## Rules Reference

- 702.2 — Flying
- 702.2b — "A creature with flying can block only creatures with flying."
- 702.2c — "A creature can block a creature with flying only if it has flying or reach."
- 702.2d — Reach

---

## Implementation Notes

### Keyword Storage

Store supported creature keywords in a small closed set rather than as one boolean field per keyword. This keeps the model explicit, compact in memory, and easier to extend when new supported keyword slices arrive.

### Blocking Legality Check

In `declare_blockers` rule function:

```rust
fn can_block_creature_with_flying(blocker: &CardInstance, attacker: &CardInstance) -> bool {
    if attacker.has_flying() {
        blocker.has_flying() || blocker.has_reach()
    } else {
        true
    }
}
```

### Creature Creation Path

Extend the creature casting path to accept keyword parameters:

```rust
// In spell resolution or creature creation
let mut creature = CardInstance::new_creature_with_keywords(
    id,
    definition_id,
    mana_cost,
    power,
    toughness,
    KeywordAbilitySet::only(KeywordAbility::Flying),
);
```

---

## Rules Support Statement

This slice introduces Flying and Reach as creature-level keywords that affect blocking legality and now has executable unit and BDD coverage for the supported interaction set. No other keyword abilities are modeled. Future slices will add trample, first strike, and other combat-relevant keywords incrementally.
