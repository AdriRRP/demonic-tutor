---
name: slice-design
description: Design or extend a DemonicTutor vertical slice with minimal scope, DDD discipline, and explicit rules support.
---

# Slice Design Skill

## Purpose

Use this skill when the task is to:

- design a new vertical slice
- extend an existing slice
- refine slice boundaries before implementation
- verify that a proposed slice is minimal, coherent, and truthful

This skill does not define project truth.
It operationalizes the existing project rules for slice work.

---

## Load Required Context

Before designing a slice, load only the minimum relevant context:

### Always load

- `CONSTRAINTS.md`
- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`
- `docs/architecture/vertical-slices.md`
- `docs/architecture/slice-template.md`

### Also load when needed

- `docs/domain/DOMAIN_GLOSSARY.md` if new terms or domain language are involved
- `docs/domain/context-map.md` if bounded contexts or ownership may change
- relevant ADRs if the slice touches an architectural simplification or boundary
- the relevant existing slice document if extending prior behavior

Do not load unrelated slice documents.

---

## Slice Design Goal

A slice must introduce **one coherent, testable, observable behavior**.

A good slice:

- solves one immediate modeling need
- introduces only the minimum domain concepts required
- preserves aggregate and bounded-context ownership
- does not imply unsupported Magic rules
- is easy to review and test

A bad slice:

- bundles multiple behaviors
- introduces speculative abstractions
- broadens rules support implicitly
- leaks infrastructure or UI concerns into the domain
- prepares for hypothetical future mechanics without active need

---

## Design Rules

### 1. Start from observable behavior

Define the slice in terms of behavior, not architecture.

Ask:

- What must become possible after this slice?
- What specific behavior will now be observable?
- What will still remain unsupported?

State the behavior explicitly.

---

### 2. Preserve truthfulness

Do not claim support for broader Magic rules than the implementation will actually provide.

Always distinguish between:

- implemented behavior
- intentionally unsupported behavior
- future possible evolution

If the slice is a simplification, say so explicitly.

---

### 3. Keep scope minimal

A slice may introduce only what is required for correctness of the target behavior.

Prefer:

- one command
- the smallest set of events
- the smallest set of validation rules
- local changes to the current aggregate

Avoid:

- introducing generic engines
- new aggregates without strong justification
- speculative domain objects
- modeling full subsystems too early

---

### 4. Respect ownership

Check whether the proposed behavior belongs to:

- the `Game` aggregate
- another bounded context
- application orchestration
- infrastructure
- analytics

Gameplay legality, turn progression, zone transitions, and player state belong to the gameplay domain boundary unless explicitly documented otherwise.

Do not move domain rules into helpers, UI, or infrastructure.

---

### 5. Design for reviewability

The slice should be explainable in a few paragraphs and testable with a small number of focused cases.

If the slice requires too many concepts at once, split it.

---

## Slice Design Procedure

When producing a new slice document or a substantial slice revision, use `docs/architecture/slice-template.md` as the canonical structure unless there is a strong reason not to.

Follow this sequence:

### Step 1 — Define the slice intent

Write one short statement:

- what the slice enables
- why it is needed now
- what it intentionally does not cover

### Step 2 — Define supported behavior

List only the behaviors that will actually be supported.

Be explicit and concrete.

### Step 3 — Define invariants and legality rules

State the minimum correctness rules that must hold.

These should be domain rules, not implementation notes.

### Step 4 — Define out of scope

Explicitly list what is not modeled yet.

This is mandatory for Magic rules work.

### Step 5 — Identify domain impact

Check whether the slice changes:

- aggregate behavior
- commands
- events
- value objects or entities
- domain errors
- glossary terms
- context map
- current-state documentation

### Step 6 — Identify tests

List the smallest set of tests needed to make the slice reviewable.

Prefer behavior-oriented tests.

---

## Expected Output Format

When using this skill, produce output in this structure:

### Slice Name

A short, explicit name.

### Goal

One paragraph.

### Supported Behavior

Bullet list of supported behavior.

### Invariants / Legality Rules

Bullet list of domain rules enforced by the slice.

### Out of Scope

Bullet list of intentionally unsupported behavior.

### Domain Impact

Only the parts of the model that must change.

### Documentation Impact

Only documents whose owned truth must change.

### Tests

Small list of focused tests.

### Open Questions

Only include questions that materially affect correctness.

---

## Review Checks

Before finalizing a slice proposal, verify:

- Is the slice minimal?
- Is the behavior observable?
- Is ownership clear?
- Does it preserve DDD boundaries?
- Does it avoid implying unsupported rules?
- Could it be split into a smaller slice?
- Does it require updating glossary, context map, current state, or ADRs?

If the answer is unclear, refine the slice before proceeding.

---

## Anti-Patterns

Do not produce slices that:

- "prepare for future combat system"
- "add generic card effect engine"
- "introduce reusable action framework"
- "support all spell timing"
- "model rules text generally"
- "anticipate multiplayer"

Unless explicitly required, these are signs of overreach.

---

## Notes for DemonicTutor

This repository evolves through **small vertical slices** over a **DDD-guided gameplay model**.

For this project specifically:

- prefer extending the current `Game` aggregate unless ownership clearly belongs elsewhere
- keep rule support narrower than the long-term vision
- document simplifications explicitly
- prefer one coherent slice over broad subsystem design
