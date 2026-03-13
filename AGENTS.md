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

## Source of Truth Priority

When reasoning about the project, use this precedence:

1. **Rust code** (`src/`) — actual implementation
2. **ADRs** (`docs/adr/*.md`) — architectural decisions
3. **`docs/current-state.md`** — implementation snapshot
4. **`DOMAIN_GLOSSARY.md`** — ubiquitous language
5. **Other documentation** — context and rationale

This order prevents contradictions between documentation and code.

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

## Implementation state

See `docs/current-state.md` for the current slice implementation status.

## Versioning and Changelog

This project follows Semantic Versioning (MAJOR.MINOR.PATCH).

- **Version**: Check `Cargo.toml` for current version
- **Changelog**: See `CHANGELOG.md` for release history

When making changes:
1. Do NOT modify version in `Cargo.toml` unless explicitly requested
2. Do NOT manually update `CHANGELOG.md` - it is generated at release time
3. Focus on implementing the feature correctly; release process handles versioning

## Documentation synchronization rules

If a task changes bounded contexts or their relationships, update `docs/context-map.md`.

This includes:
- the textual description
- the Mermaid diagram

Do not leave them out of sync.

Operational rules for agents are defined in `agents/core-agent.md`.
