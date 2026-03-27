# Slice Name

`SupportDefenderAndStaticCannotAttackRestrictions`

## Goal

Add the first bounded static attack restriction subset for creatures that cannot attack under explicit card profiles.

## Why This Slice Exists Now

To build an honest limited-like environment, the engine needs a minimal static attack-restriction corridor that does not depend on attachments or temporary effects. `Defender` is the cleanest first step because it is ubiquitous, highly legible, and fits the current explicit-profile model.

## Supported Behavior

- support creatures with an explicit profile that cannot attack while on the battlefield
- reject those creatures during attacker declaration
- keep those creatures able to block normally unless another supported effect forbids it
- keep the restriction fully static while the permanent remains on the battlefield

## Invariants / Legality Rules

- the restriction is enforced by the canonical attacker-legality corridor
- the restriction is profile-based and static, not a generic text engine
- the slice must not imply support for broader power-based or conditional attack restrictions

## Out Of Scope

- "can attack as though it didn't have defender"
- attack restrictions that depend on controller board state, life totals, or choices
- static `cannot block` profiles
- equipment, auras, or layers that grant or remove defender generically

## Domain Impact

### Aggregate Impact

- one explicit static creature profile that maps to `cannot attack`

### Commands

- no new commands

### Events

- no new events

## Test Impact

- a creature with the supported static restriction cannot be declared as an attacker
- the same creature may still block if no other supported rule forbids it
- ordinary creatures remain unaffected

## Rules Support Statement

This slice adds only the first bounded static attack restriction subset, centered on explicit defender-like creatures.
