# Slice — Respond With Second Instant In End Of Combat Window

## Goal

Make the responding player's self-stacking behavior explicit in the `EndOfCombat` priority window that opens after combat damage resolves.

## Supported Behavior

- once combat damage resolves and the active player passes, the defending player becomes the priority holder in `EndOfCombat`
- the defending player may cast an instant response
- that responding player keeps priority immediately after casting
- the responding player may cast a second instant before passing
- the top response resolves first, and the earlier response remains on the stack
- after that top response resolves, priority reopens for the active player while the game remains active

## Explicit Limits

- this slice only covers the `EndOfCombat` window
- response spells remain limited to instants
- no richer end-of-combat rules, triggers, or abilities are modeled

## Domain Changes

- no new public command is introduced
- the responding-player self-stacking pattern now has explicit executable coverage in `EndOfCombat`
- this slice supersedes the older proposal phrasing that described the same window only as "after combat damage"

## Rules Support Statement

This slice anchors post-damage interaction to the explicit combat-step model. What used to be described as a response "after combat damage" now lives clearly in the `EndOfCombat` subphase, where the defending player may build a two-spell response stack before passing priority.

## Tests

- a unit test confirms the defending player can cast two instants consecutively while holding priority in `EndOfCombat`
- BDD coverage confirms the second response resolves first and the earlier response remains on the stack
