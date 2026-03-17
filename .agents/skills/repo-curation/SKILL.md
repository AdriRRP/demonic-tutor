---
name: repo-curation
description: Audit and synchronize code, canonical docs, ADRs, and agent context after broad refactors or before commit/release.
---

# Repository Curation Skill

## Purpose

Use this skill when work has already changed meaningful parts of the repository and you need to close the loop cleanly.

Typical use cases:

- after a broad semantic refactor
- before commit or release preparation
- after removing duplicate commands, events, or modules
- when code, docs, and agent context may have drifted apart
- when a stable lesson should become future guidance for agents

This skill does not invent new project truth.
It helps ensure that the repository reflects the truth already established by code and accepted decisions.

---

## Load Required Context

### Always load

- `CONSTRAINTS.md`
- `docs/domain/current-state.md`
- `.agents/context/core-agent.md`

### Also load when needed

- `docs/domain/DOMAIN_GLOSSARY.md`
- `docs/domain/aggregate-game.md`
- `docs/architecture/system-overview.md`
- `docs/architecture/vertical-slices.md`
- relevant ADRs
- relevant implemented slice documents
- relevant skills under `.agents/skills/`

Load only what the changed truth can plausibly affect.

---

## Core Goals

Close a change set so that:

- canonical documentation matches the code
- superseded historical documents are marked honestly
- operational agent guidance reflects repeated lessons
- skills capture workflows likely to recur
- no stale terminology or obsolete APIs remain in docs
- canonical gameplay actions remain the only live public entrypoints

---

## What to Check

### 1. Canonical Truth

Check whether:

- `current-state.md` matches current capabilities
- `aggregate-game.md` still describes current ownership and invariants
- glossary terms still mean what the code now models
- architecture docs still describe the actual layering and module structure

### 2. Slice Status

Check whether implemented slice docs:

- still describe live behavior
- should be updated
- should be marked superseded instead of pretending to be current
- should stop presenting convenience commands or shortcut events as live behavior after a canonical action replaced them
- should distinguish clearly between live slices, historical baseline slices, and superseded documents

### 3. ADR Status

Check whether accepted ADRs:

- remain accurate
- need a superseded status
- need a newer ADR reference

### 4. Operational Agent Context

Check whether:

- `AGENTS.md`
- `.agents/context/core-agent.md`
- existing skills

should be updated to capture lessons that are now stable and likely to recur.

### 5. Stale Terms

Search for obsolete names, old commands, old events, or outdated phase models.

Prefer removing or narrowing stale references over leaving historical wording in active docs.

Also search for:

- feature headers that still claim `proposed` or `implemented` incorrectly
- proposal docs that are already implemented but not marked historical
- architecture docs that still describe monolithic files after internal module splits

---

## Procedure

### Step 1 — Identify the stable lessons

List what the repository has now learned that should persist beyond the current session.

Examples:

- a domain-canonical command replaced a duplicate shortcut
- event payloads need enough semantic context for replay
- internal memory optimization should stay hidden behind readable APIs
- historical docs should be marked explicitly when they no longer describe live behavior
- executable and reference features need honest status metadata
- partial stack/priority support must be enforced consistently by new actions

### Step 2 — Identify truth owners

Map each lesson to:

- canonical docs
- ADRs
- operational agent context
- skills

### Step 3 — Update live docs first

Canonical docs and accepted ADRs come before operational agent context.

### Step 4 — Update agent context and skills

Only after canonical truth is synchronized.

### Step 5 — Validate

Run repository checks and search for stale terms.

---

## Expected Output Format

When using this skill, produce:

### Curation Scope

What area of the repository is being synchronized.

### Stable Lessons

The recurring practices or decisions being captured.

### Required Updates

Docs, ADRs, agent context, and skills that must change.

### Validation

Searches and checks used to confirm closure.

### Verdict

One of:

- `Repository synchronized`
- `Further curation required`
