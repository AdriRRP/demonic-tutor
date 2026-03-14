# Slice Proposal — Release 0.2.0

This document proposes a multi-slice plan to implement basic Magic: The Gathering gameplay elements sufficient for a meaningful playtesting release (0.2.0).

## Current State (0.1.0)

**Implemented:**
- StartGame, DealOpeningHands, Mulligan, PlayLand, DrawCard, AdvanceTurn
- Infrastructure: EventStore, EventBus, GameLogProjection
- Zones: Library, Hand, Battlefield
- Only 2 players, 7-card opening hand, Setup → Main phase

**Missing for basic playtesting:**
- Life totals
- Proper turn phases (Beginning, Combat, Ending)
- Spell/ability casting
- Mana system
- Creature combat
- Graveyard

---

## Slice 8 — Life Totals & Turn Phases

### Goal
Add player life tracking and proper turn phase structure.

### Changes

**Domain:**
- `Player` gains `life: u32` (default 20)
- `Phase` expands to: `Beginning`, `FirstMain`, `Combat`, `SecondMain`, `Ending`
- `Game` tracks `turn_number: u32`

**Commands:**
- `EndTurnCommand` — advances to next phase/turn

**Events:**
- `LifeChanged` — emitted when life total changes
- `PhaseChanged` — emitted when phase changes
- `TurnEnded` — emitted when turn ends

**Tests:** ~6 new tests

---

## Slice 9 — Spell Casting & Mana

### Goal
Enable casting non-land spells with mana costs.

### Changes

**Domain:**
- Expand `CardType` to include: `Creature`, `Instant`, `Sorcery`, `Enchantment`, `Artifact`, `Planeswalker`
- Add `ManaCost` to `CardInstance` (or `CardDefinition`)
- Add `ManaPool` to `Player` with methods: `add_mana`, `spend_mana`, `tap_for_mana`
- `Land` produces 1 generic mana (simplified)

**Commands:**
- `CastSpellCommand` — cast a non-land card by paying mana cost
- `TapForManaCommand` — tap a land to add mana to pool

**Events:**
- `SpellCast` — emitted when spell is cast
- `ManaAdded` — emitted when mana is added to pool
- `ManaSpent` — emitted when mana is spent
- `CardMoved` — generic event for zone transitions

**Tests:** ~8 new tests

---

## Slice 10 — Combat Phase

### Goal
Implement basic creature combat.

### Changes

**Domain:**
- Add `Power`, `Toughness` to `CardInstance`
- `Battlefield` distinguishes `Lands` from `Creatures`
- Combat phases: `DeclareAttackers`, `DeclareBlockers`, `CombatDamage`

**Commands:**
- `DeclareAttackerCommand` — declare a creature as attacker
- `DeclareBlockerCommand` — assign blocker to attacker
- `ResolveCombatCommand` — resolve combat damage

**Events:**
- `AttackerDeclared` — emitted when attacker is declared
- `BlockerDeclared` — emitted when blocker is assigned
- `CombatDamageDealt` — emitted during damage resolution
- `CreatureDestroyed` — emitted when creature dies

**Tests:** ~10 new tests

---

## Slice 11 — Graveyard & Exile

### Goal
Implement additional zones and card movement.

### Changes

**Domain:**
- Add `Graveyard` and `Exile` zones to `Player`
- Cards move to graveyard on: spell resolution, creature death, discard

**Commands:**
- `DiscardCommand` — discard a card from hand

**Events:**
- `CardDiscarded` — emitted when card is discarded
- `CardExiled` — emitted when card is exiled

**Tests:** ~5 new tests

---

## Slice 12 — Basic Abilities (Optional)

### Goal
Simple static abilities and triggered effects.

### Changes

**Domain:**
- Add `abilities: Vec<Ability>` to `CardInstance`
- Simple abilities: `Hexproof`, `Trample`, `Flying`, `Deathtouch`

This slice is optional and can be deferred to 0.3.0.

---

## Release 0.2.0 Scope

**Included:**
- Slice 8: Life & Turn Phases
- Slice 9: Spell Casting & Mana
- Slice 10: Combat
- Slice 11: Graveyard & Exile

**Estimated Tests:** ~30 new tests
**Estimated Commands:** ~8 new commands

---

## Open Questions

1. **Mana colors** — Should we track specific colors (W, U, B, R, G) or just generic mana?
2. **Combat damage order** — How to handle multiple blockers?
3. **Card definitions** — Where are card definitions loaded from?
4. **State-based actions** — When to check and apply SBAs (e.g., lethal damage)?

---

## Recommendation

Start with **Slice 8** (Life & Turn Phases) as it provides:
- Clear, measurable progress
- Foundation for subsequent slices
- Minimal complexity increase
