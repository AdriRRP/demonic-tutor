# Slice — Respond With Second Instant In Combat Damage Window

## Goal

Make the responding player's self-stacking behavior explicit in the `CombatDamage` priority window that opens after blockers are declared.

## Supported Behavior

- once blockers are declared and the active player passes, the defending player becomes the priority holder in `CombatDamage`
- the defending player may cast an instant response
- that responding player keeps priority immediately after casting
- the responding player may cast a second instant before passing
- the top response resolves first, and the earlier response remains on the stack
- after that top response resolves, priority reopens for the active player while the game remains active

## Explicit Limits

- this slice only covers the `CombatDamage` window
- response spells remain limited to instants
- the current supported targeted instant subset is allowed, but richer combat tricks, broader target semantics, and triggered abilities are not modeled

## Domain Changes

- no new public command is introduced
- the responding-player self-stacking pattern now has explicit executable coverage in `CombatDamage`
- this slice supersedes the older proposal phrasing that described the same window only as "after blockers"

## Rules Support Statement

This slice anchors post-blocker interaction to the explicit combat model. What used to be described as a response "after blockers" now lives clearly in the `CombatDamage` subphase, where the defending player may build a two-spell response stack before passing priority.

## Tests

- a unit test confirms the defending player can cast two instants consecutively while holding priority in `CombatDamage`
- BDD coverage confirms the second response resolves first and the earlier response remains on the stack
