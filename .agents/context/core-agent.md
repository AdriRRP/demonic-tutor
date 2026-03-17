# Core Agent Behavior

This document defines how agents should work inside the DemonicTutor repository.

It describes operational behavior, not project truth.

---

## Role

Agents are disciplined contributors to the DemonicTutor repository.

They must:

- respect canonical documentation
- preserve architectural and domain boundaries
- produce small, reviewable changes
- avoid speculative behavior

Agents do not define project truth.

---

## Working Method

When performing work:

1. restate the task in repository terms
2. identify the smallest meaningful deliverable
3. load only the required context
4. make the smallest correct change
5. ensure the result is directly reviewable

Do not broaden the task scope unless correctness requires it.

If the task appears to require broader changes, explicitly explain why the scope must expand before proceeding.

Prefer:

- narrow vertical slices
- explicit naming
- deterministic logic
- incremental change
- semantically canonical domain actions
- removal of duplicate entrypoints once the real domain model is clear
- internal optimizations hidden behind stable, readable domain APIs
- focused feature scenarios for rule-heavy behavior when they clarify supported gameplay semantics

Avoid:

- broad refactors
- speculative abstractions
- hidden assumptions
- unrelated edits
- keeping obsolete commands, events, or docs alive once they are no longer canonical

---

## Domain Safety

Agents must not:

- claim support for Magic rules that are not implemented
- invent mechanics not grounded in canonical documentation
- bypass aggregate ownership
- violate ubiquitous language
- preserve a technically convenient model when it contradicts real domain semantics

Do not infer future rule support from glossary terms, ADRs, or long-term vision.

If domain truth is unclear:

- consult canonical documentation
- make uncertainty explicit
- avoid guessing

---

## Change Discipline

Prefer the smallest change that preserves correctness.

If a change affects multiple concerns, verify whether it requires updates to:

- slice documentation
- current state documentation
- glossary
- context map
- ADRs
- operational agent context
- reusable skills

When a session establishes a stable new design rule, naming rule, or repository-closing workflow, update the operational context or skills before ending the work so the lesson persists across sessions.

Documentation updates are required only when the owned truth of that document has changed.

If gameplay behavior is already tracked through repository features, verify whether the relevant `.feature` files must also be updated.

When the user asks for a full slice workflow rather than a local implementation task, prefer using the repository's slice-flow orchestration skill instead of improvising the process from scratch.

---

## Review Standard

Agent outputs should be:

- small
- explicit
- easy to review
- easy to revert

Prefer explicit localized changes over clever or wide-reaching edits.

When a broad cleanup is explicitly requested, keep the cleanup structured:

1. audit inconsistencies
2. fix canonical truth
3. fix operational agent context
4. validate the whole repository
5. ensure historical docs are marked honestly when they are no longer the live source of truth

---

## Failure Posture

When context is missing, ambiguous, or conflicting:

- stop expanding scope
- defer to canonical documentation
- escalate according to the authority model
- when sources conflict, resolve them according to the authority model defined in the agent architecture.
- surface the ambiguity clearly

Fail safely rather than invent behavior.
