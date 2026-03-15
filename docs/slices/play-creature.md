# Slice Name

PlayCreature

---

## Goal

Enable players to play creature cards from their hand onto the battlefield with power and toughness, while enforcing summoning sickness (creatures cannot attack or use tap abilities until the next turn).

---

## Why This Slice Exists Now

This slice is the next logical step after `CastSpell` because:
1. Creatures are a fundamental card type in Magic that require explicit modeling
2. It unlocks the combat system, which requires creatures on the battlefield
3. It establishes the pattern for permanent card types (creatures enter battlefield with state)
4. It provides the foundation for damage and destruction mechanics

---

## Supported Behavior

- accept `PlayCreatureCommand`
- verify player exists and is active
- verify the card is in player's hand
- verify the card is a creature type with power/toughness
- verify player has sufficient mana
- move creature from hand to battlefield
- set `has_summoning_sickness = true` (creature enters tapped due to summoning sickness)
- emit `CreatureEnteredBattlefield` event

---

## Invariants / Legality Rules

- only the active player may play creatures through this command
- the creature card must be in the player's hand
- the creature must have a valid power/toughness value
- the player must have sufficient mana to pay the creature's cost
- the creature enters the battlefield with summoning sickness (cannot attack that turn)

---

## Out of Scope

- attacking / combat damage
- blocking / combat damage resolution
- declare attackers step
- declare blockers step
- combat phase progression
- state-based actions for damage destruction (future slice)
- creature abilities (activated/triggered)
- +1/+1 counters or -1/-1 counters
- damage prevention or replacement effects
- regeneration
- summoning sickness removal (automatic at next turn only)
- creature type subtypes
- creature token creation

---

## Domain Impact

### Aggregate Impact
- extend `Game` with `play_creature` behavior
- add creature validation logic

### Entity / Value Object Impact
- extend `CardInstance` to include:
  - `power: Option<u32>`
  - `toughness: Option<u32>`
  - `card_type: CardType` (add Creature variant)
  - `has_summoning_sickness: bool`
- extend `CardType` enum to include `Creature`

### Commands
- add `PlayCreatureCommand`

### Events
- add `CreatureEnteredBattlefield`

### Errors
- add `NotACreature` error variant
- add `NotEnoughMana` error variant
- add `CardNotInHand` error variant

---

## Ownership Check

This behavior belongs to the `Game` aggregate because:
- it involves zone transitions (hand → battlefield)
- it enforces gameplay legality (mana cost, active player)
- it manages creature state (summoning sickness)
- it produces domain events for observable behavior

---

## Documentation Impact

- `docs/domain/current-state.md` - update capabilities
- `docs/domain/aggregate-game.md` - extend aggregate responsibilities
- `docs/domain/DOMAIN_GLOSSARY.md` - add creature-related terms
- `docs/architecture/vertical-slices.md` - add to slice evolution

---

## Test Impact

- play creature succeeds for valid creature in hand with mana
- play creature fails when card is not a creature
- play creature fails when player has insufficient mana
- play creature fails when card is not in hand
- play creature fails when player is not active
- creature enters battlefield with summoning sickness
- `CreatureEnteredBattlefield` event is emitted with correct data

---

## Rules Reference

- 302.1 — Creature cards are spell cards in hand, permanents on battlefield
- 302.2 — Creature spell resolves by putting creature on battlefield
- 302.4 — Power and toughness are creature characteristics
- 302.4a — Power determines combat damage dealt
- 302.4b — Toughness determines damage needed to destroy
- 302.6 — Creature cannot attack or use tap abilities the turn it enters (summoning sickness)
- 120.3 — Damage is marked on creatures
- 704.5f — Creature with toughness 0 or less is destroyed
- 704.5g — Creature with damage >= toughness is destroyed

---

## Rules Support Statement

This slice introduces creature cards with power and toughness as permanents on the battlefield. It models summoning sickness correctly (creatures enter the battlefield but cannot attack that turn). This slice does not model the combat phase, damage dealing, or state-based actions for damage destruction — those are future slices.

---

## Open Questions

- Should creatures untap automatically at the beginning of their controller's turn? (handled by turn advancement, but worth noting)
- How should we handle creatures with variable power/toughness? (defer to future slice)

---

## Review Checklist

- [x] Is the slice minimal?
- [x] Does it introduce one coherent behavior?
- [x] Are the legality rules explicit?
- [x] Is out-of-scope behavior stated clearly?
- [x] Does it avoid implying unsupported rules?
- [x] Is ownership clear?
- [x] Does it preserve bounded context and aggregate boundaries?
- [x] Are documentation updates limited to changed truth owners?
- [x] Is the slice easy to review and test?
