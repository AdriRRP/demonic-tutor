# Slice 65 — Cast Sorcery In Main Window

## Goal

Make sorcery-speed casting explicitly supported in the empty `FirstMain` and `SecondMain` priority windows already owned by the active player.

## Supported behavior

- the active player may cast a sorcery while holding priority in `FirstMain`
- the active player may cast a sorcery while holding priority in `SecondMain`
- the stack must be empty when that sorcery-speed cast starts
- the caster keeps priority after `SpellPutOnStack`
- after two consecutive passes, the sorcery resolves to the graveyard

## Out of scope

- casting sorceries as responses
- casting sorceries outside `FirstMain` or `SecondMain`
- targets, modes, and richer spell semantics

## Rules Support Statement

This slice formalizes behavior that the minimal stack model was structurally ready for but had not yet fixed as executable truth. The runtime now explicitly supports the active player casting a sorcery in an empty main-phase priority window, resolving it through the same LIFO stack flow already used by instants and permanent spells.

## Tests

- unit coverage for sorcery casting in `FirstMain`
- unit coverage for sorcery casting in `SecondMain`
- unit coverage for sorcery-speed casting while the active player already holds an empty main-phase priority window
- executable BDD coverage for both main phases
