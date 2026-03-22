# Slice — Respond With Second Instant In Declare Blockers Window

## Goal

Make the responding player's self-stacking behavior explicit in the `DeclareBlockers` priority window that opens after attackers are declared.

## Supported Behavior

- once attackers are declared and the active player passes, the defending player becomes the priority holder in `DeclareBlockers`
- the defending player may cast an instant response
- that responding player keeps priority immediately after casting
- the responding player may cast a second instant before passing
- the top response resolves first, and the earlier response remains on the stack
- after that top response resolves, priority reopens for the active player while the game remains active

## Explicit Limits

- this slice only covers the `DeclareBlockers` window
- response self-stacking in this window remains limited to the currently supported instant subset
- the current supported targeted instant subset is allowed, but richer combat tricks, broader target semantics, and triggered abilities are not modeled

## Domain Changes

- no new public command is introduced
- the responding-player self-stacking pattern now has explicit executable coverage in `DeclareBlockers`
- this slice supersedes the older proposal phrasing that described the same window only as "after attackers"

## Rules Support Statement

This slice anchors post-attacker interaction to the explicit combat model. What used to be described as a response "after attackers" now lives clearly in the `DeclareBlockers` subphase, where the defending player may build a two-spell response stack before passing priority.

## Tests

- a unit test confirms the defending player can cast two instants consecutively while holding priority in `DeclareBlockers`
- BDD coverage confirms the second response resolves first and the earlier response remains on the stack
