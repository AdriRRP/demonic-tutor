---
name: rules-consistency
description: Verify that DemonicTutor does not overstate Magic rules support across code, current state, slices, glossary, and architectural documentation.
---

# Rules Consistency Skill

## Purpose

Use this skill to verify that the repository remains truthful and internally consistent about supported Magic rules.

Typical use cases:

- after adding or extending a slice
- after changing gameplay legality
- when introducing new domain concepts from Magic
- before finalizing documentation updates
- when reviewing whether wording overstates implemented support

This skill does not expand rules support.
It checks consistency between implemented behavior and documented claims.

---

## Load Required Context

### Always load

- `CONSTRAINTS.md`
- `docs/domain/current-state.md`
- relevant slice documentation

### Also load when needed

- `docs/domain/DOMAIN_GLOSSARY.md`
- `docs/domain/aggregate-game.md`
- `docs/architecture/vertical-slices.md`
- relevant ADRs documenting temporary simplifications
- relevant code paths if implementation claims must be checked

Load only the slices and docs touched by the rule change.

---

## Core Rule

The repository must never imply broader Magic rules support than the implementation actually provides.

When in doubt:

- narrow the claim
- document the simplification
- state out-of-scope behavior explicitly

Truthfulness is more important than completeness.

---

## What to Check

Evaluate consistency across these dimensions:

### 1. Implemented Behavior vs Documentation

Check that documented behavior matches:

- what the domain actually enforces
- what the slice really supports
- what current-state claims is implemented

Potential issue:

- docs claim more legality or timing support than code provides

---

### 2. Simplification Disclosure

Check whether temporary simplifications are stated clearly.

Examples:

- explicit draw action instead of full draw step
- simplified turn model
- no stack
- no priority
- two-player only

Potential issue:

- simplified behavior presented as if it were real full rules support

---

### 3. Glossary Drift

Check whether glossary terms create false expectations.

Potential issue:

- glossary includes concepts like stack, exile, or priority, but active docs or slice wording imply those are implemented

Glossary terms may exist as language without implying implementation.
This distinction must remain clear.

---

### 4. Slice Scope Truthfulness

Check whether slice docs:

- clearly describe supported behavior
- state out of scope behavior
- avoid implying adjacent mechanics

Potential issue:

- a slice about tapping lands implies mana system completeness
- a spell-casting slice implies timing, stack, or abilities support

---

### 5. Current State Accuracy

Check whether `current-state.md` describes:

- implemented slices accurately
- temporary constraints honestly
- aggregate responsibilities without overstating rules coverage

Potential issue:

- current-state reads broader than what slices and code actually support

---

### 6. ADR Consistency

Check whether accepted ADRs that document simplifications are still reflected in current wording.

Potential issue:

- documentation now contradicts an accepted simplification without saying it was superseded

---

## Review Procedure

### Step 1 — Identify the rules area

State clearly which rules area is being reviewed.

Examples:

- turn flow
- draw behavior
- spell casting
- mana payment
- legality checks

### Step 2 — Identify the implementation claim

What does the repository currently appear to claim?

### Step 3 — Compare sources

Compare:

- current state
- relevant slice docs
- relevant glossary terms
- relevant ADRs
- relevant implementation behavior

### Step 4 — Identify inconsistency type

Classify issues as:

- overstated support
- missing simplification note
- outdated documentation
- ambiguous wording
- no issue

### Step 5 — Recommend the smallest correction

Prefer:

- narrowing language
- clarifying out-of-scope rules
- updating current-state
- updating slice docs
- updating or superseding ADRs if needed

Do not expand implementation just to satisfy wording.

---

## Expected Output Format

When using this skill, produce:

### Rules Area

Short description of the rules area reviewed.

### Sources Checked

List the documents and implementation areas reviewed.

### Findings

List any inconsistencies found.

### Required Corrections

List the smallest documentation or wording corrections required.

### Verdict

One of:

- `Consistent`
- `Consistency update required`
- `Rules support overstated`

---

## Review Checks

Before finalizing, verify:

- Did I distinguish implemented behavior from domain language?
- Did I avoid treating glossary presence as implementation support?
- Did I check current-state against slice docs rather than alone?
- Did I preserve honesty about temporary simplifications?
- Did I recommend the smallest correction rather than a broader redesign?

---

## Anti-Patterns

Do not:

- assume glossary terms imply implementation
- expand rule claims based on future roadmap
- “fix” inconsistency by inventing broader support
- confuse long-term vision with current support
- rewrite unrelated docs while checking one rules area

---

## Notes for DemonicTutor

This repository intentionally models Magic incrementally.

That means the same concept may appear in:

- glossary
- vision
- ADRs
- roadmap

without being fully implemented.

This is acceptable.

What is not acceptable is allowing those references to create false claims about current rules support.

When uncertain, prefer narrower wording and explicit out-of-scope statements.
