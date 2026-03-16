# Slice Name

PermanentType

---

## Goal

Introduce a `Permanent` type concept to the domain model that encompasses all card types that can exist on the battlefield (creatures, lands, enchantments, artifacts, planeswalkers). This enables consistent handling of permanents across the codebase and prepares for future card types.

---

## Why This Slice Exists Now

The current implementation treats card types inconsistently - creatures and lands have specific methods, but there's no unified concept of "permanent". Adding this type now will:

1. Make the domain model more consistent
2. Enable future features like "all permanents" effects
3. Clarify which cards can exist on the battlefield

---

## Supported Behavior

- Add `CardType::is_permanent()` method that returns `true` for:
  - Land
  - Creature
  - Enchantment
  - Artifact
  - Planeswalker

---

## Invariants / Legality Rules

- `is_permanent()` returns true only for types that can exist on the battlefield
- Non-permanent types (Instant, Sorcery) cannot exist on the battlefield
- This is a read-only helper method, no gameplay behavior changes

---

## Out of Scope

- Actual enchantment, artifact, or planeswalker card behavior
- Permanent-specific effects or abilities
- Any changes to battlefield logic

---

## Domain Impact

### CardType Changes
```rust
impl CardType {
    #[must_use]
    pub const fn is_permanent(&self) -> bool {
        matches!(
            self,
            Self::Land | Self::Creature | Self::Enchantment | Self::Artifact | Self::Planeswalker
        )
    }
}
```

### No Command Changes
- No new commands required

### No Event Changes
- No new events required

---

## Ownership Check

This change belongs to the `CardType` value object in the domain because:
- It extends an existing value object with new behavior
- It doesn't affect aggregate boundaries or gameplay legality
- It's a pure helper method for domain reasoning

---

## Documentation Impact

- `docs/domain/DOMAIN_GLOSSARY.md` - add definition for "permanent"
- `docs/domain/aggregate-game.md` - update CardInstance section to mention permanent types

---

## Test Impact

- Verify `is_permanent()` returns true for Land, Creature, Enchantment, Artifact, Planeswalker
- Verify `is_permanent()` returns false for Instant, Sorcery
- No gameplay behavior tests needed

---

## Rules Reference

- 110.1 — A permanent is a card or token on the battlefield
- 110.2 — A permanent's controller is the player who put it onto the battlefield

---

## Rules Support Statement

This slice introduces a domain helper method to identify permanent card types. It does not implement any gameplay behavior for enchantments, artifacts, or planeswalkers.

---

## Review Checklist

- [x] Is the slice minimal?
- [x] Does it introduce one coherent behavior?
- [x] Are the legality rules explicit?
- [x] Is out-of-scope behavior stated clearly?
- [x] Does it avoid implying unsupported rules?
- [x] Is ownership clear?
- [x] Does it preserve bounded context and aggregate boundaries?
- [x] Is the slice easy to review and test?
