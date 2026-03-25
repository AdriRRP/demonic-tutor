# Slice Name

Blocked Attacker Stays Blocked Across Damage Passes

## Goal

Preserve the blocked status of an attacker across the later combat-damage pass when first strike or double strike removes all of its blockers early.

## Why This Slice Exists Now

Once combat is split into multiple supported damage passes, a blocked attacker must not become effectively unblocked just because its blockers died before the regular pass.

## Supported Behavior

- an attacker that was blocked remains blocked for the later supported combat-damage pass even if no blocking creature survives into that pass
- a blocked attacker with no surviving blockers does not assign combat damage to the defending player unless another supported rule such as trample explicitly allows excess damage

## Invariants / Legality Rules

- blocked status is determined by combat declaration history, not only by the set of currently surviving blockers
- later combat-damage passes may filter surviving blockers for creature-to-creature damage, but they must not reinterpret the attacker as unblocked

## Out of Scope

- removing a creature from combat through effects not yet modeled
- rules-complete combat rewrite and redirection effects

## Domain Impact

### Aggregate Impact

- refine the aggregate-owned split combat-damage corridor so blocked-state semantics survive across supported passes

## Ownership Check

This belongs to the `Game` aggregate because combat blocking state and later-pass damage assignment are aggregate-owned combat rules.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/rules/notes/combat.md`
- this slice doc

## Test Impact

- a double-strike attacker that kills its only blocker in the first pass does not hit the player in the regular pass

## Rules Reference

- 509
- 510
- 702.4

## Rules Support Statement

This slice closes a semantic hole in the supported split combat-damage model. It does not imply full support for all effects that can remove creatures from combat or rewrite combat damage.
