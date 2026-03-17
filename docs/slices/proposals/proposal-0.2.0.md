# Slice Proposal — Release 0.2.0

> **Note**: Historical proposal. This document records an earlier planning snapshot and does not describe the repository's current truth.

This document proposes atomic slices for a meaningful playtesting release (0.2.0).

## Current State (0.1.0)

- StartGame, DealOpeningHands, Mulligan, PlayLand, DrawCard, AdvanceTurn
- Infrastructure: EventStore, EventBus, GameLogProjection
- Player Life, Turn Number, Turn Phases
- Tap Lands for Mana, Cast Non-Land Spells, Pay Mana Cost
- 2 players, Setup → Beginning → Main → Ending phase, life tracking, mana system with cost payment

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

## Slice 10 — Turn Phases

**Goal:** Proper turn structure with phases.

**Domain:**
- `Phase` = `Setup`, `Beginning`, `Main`, `Ending`
- Turn flow: Setup → Main → Ending → Main (next player)

**Events:**
- `PhaseChanged { from, to }`

**Tests:** ~3

---

## Slice 11 — Tap Lands for Mana

**Goal:** Lands produce mana.

**Domain:**
- `Player` gains `mana: u32`
- `CardInstance` gains `tapped: bool`

**Commands:**
- `TapLandCommand { player_id, card_id }` — tap a land to add 1 mana

**Events:**
- `LandTapped { card_id }`
- `ManaAdded { player_id, amount }`

**Tests:** ~5

---

## Slice 12 — Cast Non-Land Spells

**Goal:** Enable spell casting without mana cost (for testing).

**Domain:**
- `CardType` expands: `Creature`, `Instant`, `Sorcery`, `Enchantment`, `Artifact`, `Planeswalker`
- Remove `CardType::NonLand`, use specific types

**Commands:**
- `CastSpellCommand { player_id, card_id }` — cast any non-land card (free for now)

**Events:**
- `SpellCast { player_id, card_id }`

**Tests:** ~4

---

## Slice 13 — Pay Mana Cost

**Goal:** Require mana payment for spells.

**Domain:**
- `CardDefinition` has `mana_cost: u32`
- Player must have enough mana to cast spell

**Commands:**
- `CastSpellCommand { player_id, card_id }` — checks mana cost

**Tests:** ~4

---

## Slice 14 — Creature Power/Toughness

**Goal:** Enable combat.

**Domain:**
- `CardInstance` gains `power: Option<i32>`, `toughness: Option<i32>`
- Creatures have P/T, non-creatures are None

**Tests:** ~2

---

## Slice 15 — Declare Attacker

**Goal:** Attack with creatures.

**Domain:**
- `CombatState` tracks declared attackers

**Commands:**
- `DeclareAttackerCommand { attacker_id }`

**Events:**
- `AttackerDeclared { player_id, creature_id }`

**Tests:** ~3

---

## Slice 16 — Combat Damage (Simplified)

**Goal:** Resolve combat damage.

**Commands:**
- `ResolveCombatCommand`

**Events:**
- `DamageDealt { target, amount }`

**Tests:** ~4

---

## Slice 17 — Graveyard Zone

**Goal:** Track dead creatures.

**Domain:**
- `Player` gains `graveyard: Vec<CardInstance>`
- Creatures with damage >= toughness go to graveyard after combat

**Events:**
- `CardMovedToGraveyard { card_id }`

**Tests:** ~3

---

## Release 0.2.0 Scope

**Minimal viable:** Slices 8-13 (Life, Turn, Phases, Mana, Spells)
**Full scope:** All slices (8-17)

---

## Open Questions

1. ¿Mana de colores o genérico?
2. ¿Combat simple o con blockers?
3. ¿Dónde se cargan las definiciones de cartas?
