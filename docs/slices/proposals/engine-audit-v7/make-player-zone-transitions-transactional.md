# Proposal Slice — Make Player Zone Transitions Transactional

## Summary

Refactor player-owned zone-to-zone movement so internal transitions apply as one transactional semantic change instead of a sequence of partial mutations.

## Motivation

- remove partial-failure windows from central movement helpers
- keep visible zones and primary location index synchronized by construction
- prepare safer ownership and location indexing at the aggregate level

## Target Shape

- zone transitions update source, destination, and primary location as one semantic operation
- internal invariant failures do not strand cards between visible and indexed state
- zone movement helpers become the trustworthy core for future rule growth

## Invariants

- a live player-owned card still belongs to at most one player-owned zone
- visible zone behavior remains unchanged
- this slice does not expand supported Magic rules
