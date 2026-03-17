---
name: adr-drafting
description: Draft a concise DemonicTutor ADR for a meaningful architectural decision, simplification, or superseding change.
---

# ADR Drafting Skill

## Purpose

Use this skill when a change introduces, confirms, or supersedes a meaningful architectural decision in DemonicTutor.

Typical use cases:

- a new structural decision is needed
- an accepted temporary simplification must be recorded
- an existing ADR is explicitly superseded
- a repeated implicit decision should become explicit

This skill helps draft ADRs.
It does not decide whether an ADR is justified by itself.

---

## Load Required Context

### Always load

- `CONSTRAINTS.md`
- `docs/architecture/agent-architecture.md`
- relevant ADRs in `docs/architecture/adr/`
- `docs/architecture/adr-template.md`

### Also load when needed

- `docs/architecture/system-overview.md`
- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`
- relevant slice documentation
- `docs/domain/context-map.md` if context ownership is involved

Do not load unrelated ADRs or slices.

---

## Core Rule

Create or update an ADR only when the change affects:

- architecture
- aggregate strategy
- bounded-context boundaries
- authority boundaries
- a meaningful simplification with architectural consequences

Do not create ADRs for routine implementation details.

---

## What Counts as ADR-Worthy

An ADR is justified when at least one is true:

- it changes architectural direction
- it fixes ownership of an important concern
- it records a temporary simplification that constrains future design
- it supersedes an earlier architectural assumption
- it reduces repeated ambiguity in future work
- it establishes the canonical domain entrypoint where the model previously had duplicates

An ADR is usually not justified for:

- naming tweaks
- small local refactors
- test-only changes
- slice details with no architectural impact

---

## ADR Structure

Use this structure:

### Title

Short, explicit, decision-focused.

### Status

Usually one of:

- Accepted
- Proposed
- Superseded

### Context

What problem or pressure exists now?

Describe only the context needed to justify the decision.

### Decision

State the decision clearly and concretely.

Prefer direct wording over discussion.

### Consequences

Describe both:

- positive outcomes
- trade-offs or limitations

### Notes

Optional.

Use only for clarifications, temporary constraints, or supersession references.

---

## Drafting Rules

### 1. Write decisions, not essays

An ADR should explain:

- what was decided
- why it was needed
- what trade-offs it introduces

Avoid narrative drift.

---

### 2. Keep architectural scope explicit

If the decision affects only:

- a slice
- an aggregate
- context ownership
- application orchestration

say so explicitly.

Do not make local decisions sound global.

---

### 3. Record simplifications honestly

If the decision is intentionally temporary, say so.

Temporary simplifications are valid ADR material when they constrain current design.

---

### 4. Prefer supersession over silent contradiction

If a prior ADR is no longer accurate:

- reference it
- mark the new relationship clearly
- do not silently contradict the old decision

This also applies when older slices or operational guidance still describe superseded behavior.

If the ADR establishes a canonical gameplay action, check whether older commands, events, slices, or operational guidance now need to be retired or marked superseded.

---

### 5. Stay consistent with project truth

An ADR must not contradict:

- code, if already implemented
- accepted canonical constraints
- current bounded context ownership

If it does, the discrepancy must be made explicit.

---

## Drafting Procedure

When drafting a new ADR, use `docs/architecture/adr-template.md` as the canonical structure unless the existing ADR series requires a justified deviation.

### Step 1 — Identify the decision

State what architectural decision is being recorded.

### Step 2 — Confirm it is ADR-worthy

Check whether it affects architecture, ownership, or a meaningful simplification.

### Step 3 — Load only relevant prior decisions

Find ADRs that may overlap or be superseded.

### Step 4 — Draft the ADR

Keep it concise and explicit.

### Step 5 — Check for contradiction

Verify consistency with:

- current code
- canonical documentation
- prior accepted ADRs

---

## Expected Output Format

When using this skill, produce:

### ADR Recommendation

- `ADR required`
- `ADR optional`
- `ADR not needed`

### Decision Summary

One short paragraph.

### Draft ADR

If required or useful, provide the ADR in repository format.

### Related ADRs

List prior ADRs that are relevant or superseded.

---

## Review Checks

Before finalizing, verify:

- Is this really an architectural decision?
- Is the decision explicit and scoped correctly?
- Is the ADR concise?
- Does it record trade-offs honestly?
- Does it contradict any accepted ADR without acknowledging it?
- Is this a decision rather than implementation commentary?

---

## Anti-Patterns

Do not:

- create ADRs for routine coding choices
- duplicate current-state documentation
- use ADRs as backlog notes
- hide uncertainty behind vague wording
- silently replace older architectural decisions

---

## Notes for DemonicTutor

In this project, ADRs are especially appropriate for:

- aggregate strategy
- event publication boundaries
- temporary rule-model simplifications
- bounded-context responsibility decisions
- agent architecture decisions
- stable stack/priority simplifications that materially constrain later slice design
- repository-wide guidance on how historical slices or proposals should be treated

Prefer a small ADR with a clear trade-off over a long ambiguous one.
