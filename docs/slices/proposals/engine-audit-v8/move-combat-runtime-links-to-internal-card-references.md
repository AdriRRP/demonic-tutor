# Proposal Slice — Move Combat Runtime Links To Internal Card References

## Summary

Replace combat-time creature links such as blocking relationships from public `CardInstanceId` references to internal handles or compact combat-local references.

## Motivation

- remove public-id leakage from creature runtime state
- improve locality for combat bookkeeping as the combat model grows
- prepare future combat slices without hard-wiring textual ids into runtime flags

## Target Shape

- combat-time creature relationships use internal runtime references
- public `CardInstanceId` is derived only when an external event or API requires it
- combat bookkeeping remains compact and deterministic

## Invariants

- attacking and blocking relationships remain truthful and unambiguous
- the current one-blocker combat model remains intact unless a later slice changes it
- this slice does not expand supported Magic rules
