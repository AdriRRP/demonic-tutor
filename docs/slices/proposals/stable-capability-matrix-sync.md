# Slice Proposal — Stable Capability Matrix Sync

## Goal

Compress the engine's stable-v1 support into canonical matrices for mana, casting, targeting, and combat.

## Why This Slice Exists Now

Once the main v1 slices land, the truth will be too broad to remain readable as a linear checklist. A capability-matrix pass will keep the repo executable and understandable for both humans and agents.

## Supported Behavior

- canonical docs summarize supported mana, casting, targeting, and combat subsets in compact matrices
- slice docs cross-reference those matrices instead of restating the entire model

## Invariants / Legality Rules

- docs must remain exact about the supported subset
- matrices must not imply broader Magic support than the code actually has

## Out of Scope

- runtime changes
- proposal of future unsupported mechanics

## Domain Impact

- no runtime change
- documentation-only stabilization

## Ownership Check

This belongs to canonical documentation and agent context synchronization.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/DOMAIN_GLOSSARY.md`
- relevant slice docs
- `.agents/context/core-agent.md` if a stable guardrail emerges

## Test Impact

- no runtime tests

## Rules Reference

- repository-owned documentation sync only

## Rules Support Statement

This slice is documentation-only. It keeps a larger supported engine subset navigable without overstating rules support.
