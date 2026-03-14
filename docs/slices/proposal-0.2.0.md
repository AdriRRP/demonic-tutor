# Slice Proposal — Release 0.2.0

This document proposes atomic slices for a meaningful playtesting release (0.2.0).

## Current State (0.1.0)

- StartGame, DealOpeningHands, Mulligan, PlayLand, DrawCard, AdvanceTurn
- Infrastructure: EventStore, EventBus, GameLogProjection
- Only 2 players, Setup → Main phase, no life, no combat

---

## Slice 8 — Player Life

**Goal:** Add player life tracking.

**Domain:**
- `Player` gains `life: u32` (default 20)

**Events:**
- `LifeChanged { player_id, from, to }`

**Tests:** ~2

---

## Slice 9 — Turn Number

**Goal:** Track turn count.

**Domain:**
- `Game` gains `turn_number: u32`

**Events:**
- `TurnNumberChanged { old, new }`

**Tests:** ~2

---

## Slice 10 — Beginning Phase

**Goal:** Proper turn start with Beginning phase.

**Domain:**
- `Phase` = `Beginning`, `FirstMain`, `Combat`, `SecondMain`, `Ending`
- Turn flow: Beginning → FirstMain → Combat → SecondMain → Ending → Beginning (next turn)

**Events:**
- `PhaseChanged { from, to }`

**Tests:** ~3

---

## Slice 11 — End Turn Command

**Goal:** Allow ending current phase/turn.

**Commands:**
- `EndTurnCommand` — advances to next phase; if Ending, starts next player's turn

**Tests:** ~3

---

## Slice 12 — Tap Lands for Mana

**Goal:** Lands produce mana.

**Domain:**
- `Player` gains `mana_pool: u32`
- `Battlefield` cards gain `tapped: bool`

**Commands:**
- `TapLandCommand { card_id }` — tap a land to add 1 mana

**Events:**
- `LandTapped { card_id }`
- `ManaAdded { player_id, amount }`

**Tests:** ~3

---

## Slice 13 — Cast Non-Land Spells (Simplified)

**Goal:** Enable spell casting without mana cost (for testing).

**Domain:**
- `CardType` expands: `Creature`, `Instant`, `Sorcery`, `Enchantment`, `Artifact`, `Planeswalker`
- Remove `CardType::NonLand`, use specific types

**Commands:**
- `CastSpellCommand { card_id }` — cast any non-land card (free for now)

**Events:**
- `SpellCast { card_id, caster_id }`

**Tests:** ~3

---

## Slice 14 — Pay Mana Cost

**Goal:** Require mana payment for spells.

**Domain:**
- `CardDefinition` has `mana_cost: u32`
- Player must have enough mana to cast spell

**Commands:**
- `CastSpellCommand { card_id }` — checks mana cost

**Tests:** ~4

---

## Slice 15 — Creature Power/Toughness

**Goal:** Enable combat.

**Domain:**
- `CardInstance` gains `power: Option<i32>`, `toughness: Option<i32>`
- Creatures have P/T, non-creatures are None

**Tests:** ~2

---

## Slice 16 — Declare Attacker

**Goal:** Attack with creatures.

**Domain:**
- `CombatState` tracks declared attackers

**Commands:**
- `DeclareAttackerCommand { attacker_id }`

**Events:**
- `AttackerDeclared { player_id, creature_id }`

**Tests:** ~3

---

## Slice 17 — Combat Damage (Simplified)

**Goal:** Resolve combat damage.

**Commands:**
- `ResolveCombatCommand`

**Events:**
- `DamageDealt { target, amount }`

**Tests:** ~4

---

## Slice 18 — Graveyard Zone

**Goal:** Track dead creatures.

**Domain:**
- `Player` gains `graveyard: Vec<CardInstance>`
- Creatures with damage >= toughness go to graveyard after combat

**Events:**
- `CardMovedToGraveyard { card_id }`

**Tests:** ~3

---

## Release 0.2.0 Scope

**Minimal viable:** Slices 8-12 (Life, Phases, Mana)
**Full scope:** All 11 slices (8-18)

---

## Open Questions

1. ¿Mana de colores o genérico?
2. ¿Combat simple o con blockers?
3. ¿Dónde se cargan las definiciones de cartas?
