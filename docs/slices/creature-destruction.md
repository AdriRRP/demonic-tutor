# CreatureDestruction

---
## Goal
Implement creature destruction as a state-based action where creatures with damage marked on them greater than or equal to their toughness are destroyed and moved to the graveyard. This completes the combat interaction loop started by combat damage resolution.

---
## Why This Slice Exists Now
This slice follows CombatDamage because:
1. Combat damage is already being dealt and marked on creatures
2. Creatures have toughness modeled
3. Without creature destruction, combat has no lasting consequences
4. It enables meaningful gameplay interactions where creatures can be removed from play
5. It's the next logical step in combat resolution after damage assignment

---
## Supported Behavior
- After combat damage is dealt, check all creatures on the battlefield
- Creatures with damage marked >= toughness are destroyed
- Destroyed creatures are moved from battlefield to graveyard
- Destruction happens as a state-based action (automatic check)
- Creatures with indestructible ability are not destroyed (future ability)
- Regeneration can prevent destruction (future ability)
- Emit CreatureDestroyed event for each destroyed creature

---
## Invariants / Legality Rules
- Destruction occurs as a state-based action after combat damage resolution
- Only creatures on the battlefield can be destroyed
- A creature's toughness must be known to evaluate destruction
- Damage marked persists until end of turn (unless destroyed)
- Destruction checks happen at specific times (after combat, during cleanup)
- No player choice involved in destruction (automatic game rule)

---
## Out of Scope
- Triggered abilities that activate on creature destruction
- Replacement effects that modify destruction (e.g., "instead of destruction")
- Multiple state-based actions in sequence
- Destruction from sources other than combat damage (e.g., spells)
- Indestructible keyword ability
- Regeneration ability
- Tokens ceasing to exist after destruction
- Graveyard order or interactions
- State-based actions outside of combat (e.g., during upkeep)
- Legend rule
- Planeswalker uniqueness rule

---
## Domain Impact
### Aggregate Impact
- Extend `Game` with state-based action checking for creature destruction
- Add destruction check after combat damage resolution in phase progression

### Entity / Value Object Impact
- `CardInstance` - no changes needed (already has damage field)
- Add `is_destroyed()` method to check if creature should be destroyed
- `Zone` tracking - creatures move from battlefield to graveyard zone

### Commands
- No new commands required (automatic state-based action)

### Events
- Add `CreatureDestroyed` event containing:
  - Game ID
  - Destroyed creature's CardInstanceId
  - Destroyed creature's controller PlayerId

### Errors
- No new error variants required

---
## Ownership Check
This behavior belongs to the `Game` aggregate because:
- It involves state-based actions that affect game state
- It enforces game rules about creature existence
- It modifies creature zone placement
- It produces domain events
- It happens automatically without player input

---
## Documentation Impact
- `docs/domain/current-state.md` - add creature destruction to implemented capabilities
- `docs/domain/aggregate-game.md` - extend aggregate responsibilities for state-based actions
- `docs/slices/creature-destruction.md` - this document

---
## Test Impact
- Verify creatures with damage >= toughness are destroyed after combat
- Verify creatures with damage < toughness survive
- Verify destroyed creatures move from battlefield to graveyard
- Verify destruction happens automatically after combat damage
- Verify events are emitted correctly for destroyed creatures
- Verify no destruction occurs outside appropriate timing

---
## Rules Reference
- 704.5g — If a creature has toughness greater than 0, and the total damage marked on it is greater than or equal to its toughness, that creature has been dealt lethal damage and is destroyed. Regeneration can replace this event.
- 704.5h — If a creature has toughness 0 or less, it's destroyed.
- 704 — State-based actions (general rule)
- 110.4 — A creature's toughness is the amount of damage needed to destroy it.

---
## Rules Support Statement
This slice implements creature destruction as a state-based action as defined in Magic Comprehensive Rules 704.5g. It does not implement regeneration, indestructible, or other abilities that modify or prevent destruction. Destruction is checked automatically after combat damage resolution.

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