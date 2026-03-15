# Rules Consistency

## Summary

Fix rules inconsistencies between documented state and actual implementation. Implement the `DeclareBlockers` command that was defined but never implemented.

## Issues Found

1. **Phase `Beginning`**: Exists in enum but is never used (skipped in `advance_turn`)
2. **DeclareBlockers**: Command and event exist but no implementation
3. **Documentation**: `current-state.md` claims "no combat system" but `DeclareAttackers` is implemented

## Scope

### Documentation Fixes
- `docs/domain/current-state.md` - Update to reflect implemented combat (declare attackers)
- Remove "no combat system" from constraints
- Update "next modeling decision" section

### Implementation
- `src/domain/game.rs` - Implement `declare_blockers` method
- Ensure blockers can only be declared in Beginning phase
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
