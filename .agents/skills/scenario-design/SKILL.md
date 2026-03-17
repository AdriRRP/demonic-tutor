---
name: scenario-design
description: Design or curate Gherkin gameplay features that stay truthful to DemonicTutor slices, rules references, and ubiquitous language.
---

# Scenario Design Skill

## Purpose

Use this skill when the task is to:

- introduce or revise Gherkin gameplay features
- map gameplay behavior to rules references
- connect features with slices
- review whether a proposed scenario is truthful and minimal

This skill does not turn the full Magic Comprehensive Rules into a direct implementation backlog.

---

## Load Required Context

### Always load

- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`
- `docs/architecture/vertical-slices.md`
- `docs/architecture/gherkin-features.md`
- `docs/rules/rules-map.md`
- `features/README.md`

### Also load when needed

- `docs/domain/DOMAIN_GLOSSARY.md`
- relevant slice docs
- relevant rules notes under `docs/rules/notes/`
- relevant ADRs

Load only the rule areas and slices actually involved.

---

## Core Rules

Scenarios must:

- describe observable gameplay behavior
- use canonical gameplay actions
- reference relevant rules sections
- map to one or more slices
- state only supported or explicitly proposed behavior
- make it clear through status metadata whether they are executable or reference-only when that distinction matters

Scenarios must not:

- copy the rulebook literally
- imply unsupported stack or priority behavior
- use convenience commands that are no longer canonical
- hide implementation assumptions behind vague Given/Then steps

---

## Procedure

### Step 1 — Identify the behavior

State the gameplay behavior being specified.

### Step 2 — Check current support

Verify whether the behavior is:

- implemented
- proposed
- historical

### Step 3 — Map rules and slices

Attach:

- relevant rule sections
- relevant slice documents

Also decide whether the feature is:

- executable now
- implemented reference-only
- proposed

### Step 4 — Write scenarios

Prefer a small number of scenarios that cover:

- happy path
- critical legality rejection
- semantically important outcome

### Step 5 — Check truthfulness

Verify that the scenario wording does not overstate support.

If the repository already has a newer slice that supersedes the feature's original role, consider whether the feature should become `historical` or reference a different implemented slice.

---

## Expected Output Format

When using this skill, produce:

### Behavior

Short description of the behavior.

### Status

`implemented`, `proposed`, or `historical`

### Rule References

Relevant rule sections.

### Slice Mapping

Relevant slices.

### Feature Draft

One feature with focused scenarios.
