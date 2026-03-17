---
name: context-sync
description: Check whether a change in DemonicTutor also requires synchronized updates to canonical documentation such as glossary, context map, aggregate, current state, slice docs, or ADRs.
---

# Context Sync Skill

## Purpose

Use this skill when a task may have changed project truth and you need to determine whether canonical documentation must also be updated.

Typical use cases:

- after implementing or extending a slice
- after changing aggregate behavior
- after introducing new domain terms
- after changing bounded context ownership
- after refining architectural boundaries
- before finalizing a reviewable change set
- after a broad cleanup that should influence future agent behavior

This skill does not define project truth.
It checks whether already-owned truth must be synchronized.

---

## Load Required Context

### Always load

- `CONSTRAINTS.md`
- `docs/domain/current-state.md`

### Also load when needed

- `docs/domain/DOMAIN_GLOSSARY.md` if domain terms or meanings may have changed
- `docs/domain/context-map.md` if bounded contexts or ownership may have changed
- `docs/domain/aggregate-game.md` if aggregate responsibilities may have changed
- `docs/architecture/system-overview.md` if architectural structure may have changed
- `docs/architecture/vertical-slices.md` if slice policy or ordering may be affected
- relevant slice documentation if implemented behavior changed
- relevant ADRs if the change touches an architectural decision or temporary simplification

Do not load documentation that cannot plausibly be affected by the change.

---

## Core Rule

Documentation must be updated only when the **owned truth** of that document has changed.

Do not update documents mechanically.
Do not update documents just because code changed nearby.

A document requires synchronization only if the change modifies what that document is responsible for stating.

---

## Owned Truth by Document

Use this ownership model when checking synchronization.

| Document | Owned truth |
|------|------|
| `PROJECT.md` | product vision, identity, major goals, non-goals |
| `CONSTRAINTS.md` | hard project constraints |
| `docs/domain/DOMAIN_GLOSSARY.md` | domain terms and their meanings |
| `docs/domain/context-map.md` | bounded contexts and relationships |
| `docs/domain/aggregate-game.md` | `Game` aggregate responsibilities and boundaries |
| `docs/domain/current-state.md` | implemented system state and temporary constraints |
| `docs/architecture/system-overview.md` | high-level architecture and system properties |
| `docs/architecture/vertical-slices.md` | slice policy and slice index |
| `docs/slices/*` | behavior and scope of a specific implemented or proposed slice |
| `docs/architecture/adr/*` | accepted architectural decisions and rationale |
| `AGENTS.md` | agent entrypoint and repository-wide operational routing |
| `.agents/context/*` | operational working posture for future sessions |
| `.agents/skills/*` | reusable workflows and guardrails for repeated tasks |
| `features/*` and `features/README.md` | supported gameplay scenarios and their execution/reference status |

If a change does not alter the owned truth of a document, do not update it.

---

## Synchronization Triggers

### Update the glossary when:

- a new domain term is introduced
- an existing term changes meaning
- a term becomes important enough to be part of ubiquitous language

Do not update the glossary for temporary implementation details.

---

### Update the context map when:

- a new bounded context appears
- ownership of behavior shifts between contexts
- relationships between contexts change
- responsibilities move across context boundaries

Do not update the context map for purely internal changes inside one context.

---

### Update the aggregate document when:

- the `Game` aggregate gains or loses responsibilities
- a new internal entity or value object becomes part of the aggregate model
- an invariant or legality responsibility changes
- ownership moves out of or into the aggregate

Do not update it for trivial implementation refactors.

---

### Update current state when:

- implemented behavior changes
- supported slice set changes
- temporary constraints change
- the practical system state described in the document is no longer accurate

This is one of the most commonly affected documents.

---

### Update slice documents when:

- the supported behavior of a slice changes
- new invariants or legality rules are introduced
- out-of-scope boundaries change
- tests or domain impact materially change
- a once-live convenience action is replaced by a canonical domain action and the old slice must become historical

Update only the slice documents directly affected.

If an implemented slice is no longer the live source for behavior because later slices superseded it, mark it honestly instead of silently leaving it current-looking.

---

### Update the system overview when:

- architectural layering changes
- system-wide properties change
- deployment assumptions change
- major responsibilities move between layers

Do not update it for local slice changes unless they alter architecture at system level.

---

### Update vertical slices when:

- a new slice is introduced
- slice ordering or policy changes
- the slice index is no longer accurate

Do not treat it as a second current-state document.

### Update features when:

- supported gameplay behavior changed
- a feature status changed between `proposed`, `implemented`, and `historical`
- slice mappings changed
- an executable feature became reference-only or vice versa

---

### Update ADRs when:

- a new significant architectural decision is made
- an accepted simplification is explicitly superseded
- a previous architectural decision is no longer accurate and must be amended or replaced

Do not create ADRs for routine implementation details.

### Update operational agent context when:

- a repeated design error or drift pattern has become clear
- a stable workflow should be repeated in future sessions
- agents need new guardrails to preserve semantics, consistency, or repository hygiene
- a broad curation or release-preparation pattern should become standard closing behavior

### Update skills when:

- a workflow is now recurrent
- repeated cleanup or verification can be standardized
- future work would benefit from narrower, reusable instructions

---

## Sync Procedure

Follow this sequence:

### Step 1 — Identify the change

State clearly what changed:

- behavior
- ownership
- terminology
- architecture
- slice status
- constraints
- operational workflow or repository guardrails

### Step 2 — Identify possible truth owners

List which documents might own the changed truth.

### Step 3 — Eliminate false positives

For each candidate document, ask:

- Did its owned truth actually change?
- Or did only implementation details change?

Remove documents whose truth did not change.

### Step 4 — Classify required updates

For each affected document, classify the update as:

- required
- optional
- not needed

Only “required” documents must be updated before considering the change synchronized.

### Step 5 — Explain why

For every required update, state in one sentence why the document must change.

This prevents mechanical or speculative edits.

---

## Expected Output Format

When using this skill, produce output in this structure:

### Change Summary

One short paragraph describing what changed.

### Candidate Documents

List of documents that might be affected.

### Required Updates

Bullet list of documents that must be updated, with a one-line reason each.

### Optional Updates

Only include if genuinely useful.

### No Update Needed

List documents considered but rejected, with a short reason if helpful.

### Sync Verdict

One of:

- `Synchronized`
- `Documentation update required`
- `Architectural decision update required`

---

## Review Checks

Before finalizing, verify:

- Did I update only documents whose owned truth changed?
- Did I avoid updating documents mechanically?
- Did I miss any canonical document affected by the change?
- Did I confuse implementation details with project truth?
- Did I treat `current-state.md` as the implementation snapshot rather than architecture?
- Did I treat ADRs as decisions rather than status docs?

---

## Anti-Patterns

Do not:

- update every nearby markdown automatically
- treat all code changes as documentation changes
- update glossary with implementation jargon
- modify context map for local aggregate changes
- create ADRs for routine slice work
- restate canonical content inside the sync output

---

## Notes for DemonicTutor

In this repository, the most commonly affected documents after meaningful changes are:

- `docs/domain/current-state.md`
- relevant slice documentation
- `docs/domain/aggregate-game.md`

Less frequently affected, but important when needed:

- `docs/domain/DOMAIN_GLOSSARY.md`
- `docs/domain/context-map.md`
- `docs/architecture/adr/*`
- `features/*`
- `features/README.md`

The goal of this skill is not to increase documentation churn.

The goal is to keep canonical truth accurate with the smallest necessary set of updates.
