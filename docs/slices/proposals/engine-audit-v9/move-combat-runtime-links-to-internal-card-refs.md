# Proposal Slice — Move Combat Runtime Links To Internal Card Refs

## Summary

Replace combat-time creature links such as blocking relationships from public `CardInstanceId` references to internal handles or compact combat-local references.

## Motivation

- remove public-id leakage from creature runtime state
- improve locality for combat bookkeeping as combat grows
- prepare future combat slices without textual ids in runtime flags

## Target Shape

- combat-time creature relationships use internal runtime references
- public `CardInstanceId` is derived only for events or external views
- combat bookkeeping remains compact and deterministic

## Invariants

- attacking and blocking relationships remain truthful and unambiguous
- the current one-blocker combat model remains intact unless a later slice changes it
- this slice does not expand supported Magic rules
