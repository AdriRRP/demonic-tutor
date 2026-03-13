# DemonicTutor Project Rules

This repository contains the early design and implementation of DemonicTutor, a client-side Magic: The Gathering deck playtesting and analysis application.

## Canonical project documents

Always treat the following files as the primary source of truth for this repository:

1. `PROJECT.md`
2. `CONSTRAINTS.md`
3. `DOMAIN_GLOSSARY.md`
4. `docs/system-overview.md`
5. `docs/context-map.md`
6. `docs/vertical-slices.md`
7. `docs/adr/*.md`

If a proposal conflicts with these files, these files win.

## Project intent

DemonicTutor is:

* client-side first
* static-deployable
* Rust-centered in the core
* WebAssembly-oriented
* DDD-guided
* event-driven
* incrementally developed

DemonicTutor is not:

* a full implementation of all Magic rules from day one
* a backend-heavy service
* a generic card marketplace or collection app
* an excuse for speculative over-engineering

## Development philosophy

Work incrementally.

Prefer:

* narrow vertical slices
* explicit naming
* small coherent changes
* draft-first proposals
* reviewable outputs

Avoid:

* broad speculative architecture
* unsupported claims about rules coverage
* unnecessary abstractions
* introducing concepts that belong only to future slices

## Domain rules

Do not claim support for Magic rules unless that support is explicitly modeled.

Only introduce the minimum rule subset required by the active vertical slice.

Treat official Magic rules as normative input when provided, but do not assume the implementation already covers them.

## Architecture rules

Do not place business logic in UI or infrastructure.

Keep the domain core deterministic.

Aggregates emit events but do not publish them directly.

Application services orchestrate loading, command execution, event persistence and event publication.

Analytics is observational and must not influence gameplay legality.

## Working style for this repository

When asked to perform a task:

1. restate the task in repository terms
2. identify the smallest sensible deliverable
3. produce a result that is directly reviewable
4. avoid changing many files at once unless explicitly requested

When proposing code or structure:

* respect the current active vertical slice
* prefer the simplest valid design
* list open questions only when they materially affect correctness

## Current bounded contexts

* `play`
* `deck`
* `analytics`

## Current implementation priority

Implement slices in this order:

1. `StartGame`
2. `DrawOpeningHand`
3. `PlayLand`

Do not jump ahead unless explicitly instructed.
