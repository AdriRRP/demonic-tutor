# Slice 67 — Cast Artifact In Main Window

## Goal

Make artifact casting explicitly executable in the empty `FirstMain` and `SecondMain` priority windows already owned by the active player.

## Supported behavior

- the active player may cast an artifact while holding priority in `FirstMain`
- the active player may cast an artifact while holding priority in `SecondMain`
- the stack must be empty when that cast begins
- the caster keeps priority after `SpellPutOnStack`
- after two consecutive passes, the artifact resolves to the battlefield

## Out of scope

- artifact responses on an open stack beyond the currently supported instant model
- activated abilities of artifacts

## Rules Support Statement

This slice extends the now-explicit sorcery-speed timing model to noncreature permanent spells. Artifacts may be cast by the active player in an empty main-phase priority window, use the same stack loop as other spells, and resolve to the battlefield.
