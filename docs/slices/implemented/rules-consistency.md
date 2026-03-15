# Rules Consistency

## Summary

Fix rules inconsistencies between documented state and actual implementation. Implement the `DeclareBlockers` command that was defined but never implemented.

## Issues Found

1. **Phase `Beginning`**: Existed in enum but was never used (skipped in `advance_turn`) - FIXED: removed
2. **DeclareBlockers**: Command and event existed but no implementation - FIXED: now implemented
3. **Documentation**: `current-state.md` claimed "no combat system" but `DeclareAttackers` is implemented - FIXED: updated

## Scope

### Documentation Fixes
- `docs/domain/current-state.md` - Update to reflect implemented combat (declare attackers)
- Remove "no combat system" from constraints
- Update "next modeling decision" section

### Implementation
- `src/domain/game/combat.rs` - Implement `declare_blockers` method
- Ensure blockers can only be declared in Main phase (after fix)
- Ensure blocking creatures are controlled by defending player
- Ensure blocking creatures are not tapped
- Mark blocking creatures as `is_blocking`

### Commands Exists But Not Implemented
- `DeclareBlockersCommand` - Already defined in commands.rs
- `BlockersDeclared` event - Already defined in events.rs
- `is_blocking` field - Already exists in CardInstance

## Rules Reference

- 509.1 - Declare blockers
- 509.2 - Declaring blocking creatures
- 509.3 - Blocking restrictions
- 509.4 - Order of blockers

## Verification

After this slice:
- Phase model accurately reflects the game flow
- DeclareBlockers command works correctly
- Documentation matches implementation
