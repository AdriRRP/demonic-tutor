# Slice 66 — Reject Sorcery Response

## Goal

Keep the current stack model semantically honest by rejecting sorcery-speed spells as responses on an already open stack.

## Supported behavior

- after the active player passes a stack that already contains a spell, the non-active player may not cast a sorcery as a response
- the action fails with an explicit casting-permission error for the active-player empty-main-phase restriction
- the original spell remains on the stack

## Out of scope

- widening response timing to sorceries
- introducing a richer “sorcery speed” error taxonomy

## Rules Support Statement

The runtime already supports only instant responses during the currently implemented priority windows. This slice makes that limit explicit for sorceries so the repository does not imply broader stack timing than it really supports.

## Tests

- unit coverage rejecting a sorcery response on an open stack
- executable BDD coverage for the same rejection path
