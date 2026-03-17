---
name: release-prep
description: Prepare semantic commits and cut a DemonicTutor release with synchronized version, changelog, validation, and tag creation.
---

# Release Preparation Skill

## Purpose

Use this skill when the user explicitly asks to:

- prepare a commit-ready closing pass
- group changes into semantic commits
- cut a release
- update version and changelog for a release
- create a release tag

This skill does not invent release scope.
It organizes and closes the change set already accepted by the repository state.

---

## Load Required Context

### Always load

- `CONSTRAINTS.md`
- `docs/development/development.md`
- `CHANGELOG.md`
- `Cargo.toml`
- `AGENTS.md`

### Also load when needed

- `docs/domain/current-state.md`
- relevant ADRs
- relevant slice docs
- `.agents/context/core-agent.md`
- `.agents/skills/repo-curation/SKILL.md`

Load only the release-relevant documents.

---

## Core Goals

Close a release candidate so that:

- the working tree is grouped into semantic commits
- canonical docs and operational guidance are already synchronized
- the changelog reflects the actual released scope
- the crate version matches the release
- repository checks pass before the release commit
- the final tag is created from a validated commit

---

## Commit Discipline

Prefer a small number of meaningful commits over one large undifferentiated snapshot.

Typical grouping:

1. domain or architecture changes
2. tests or supporting fixtures
3. documentation, ADRs, agent guidance, and skills
4. release commit (`Cargo.toml`, `CHANGELOG.md`, release metadata)

Do not force this exact structure if the change set has a clearer semantic grouping.

Each commit message should:

- use a semantic prefix such as `feat`, `fix`, `refactor`, `docs`, `test`, or `chore`
- describe one coherent outcome
- avoid mixing release metadata with functional changes

---

## Release Rules

When cutting a release:

- update `CHANGELOG.md` first so the released scope is honest
- update `Cargo.toml` version only in the release commit unless the user requests otherwise
- keep the release commit focused on release metadata
- create an annotated tag after validation

If the repository already tracks an `Unreleased` section for the target version, convert it to a dated release entry rather than rewriting history.

---

## Procedure

### Step 1 — Audit the working tree

Identify:

- semantic change groups
- release-relevant files
- whether repository curation is already complete

### Step 2 — Define commit grouping

Write the intended commit sequence before staging.

### Step 3 — Commit semantic change groups

Stage by intent, not by directory alone.

### Step 4 — Validate before release

Run repository checks before creating the release commit.

### Step 5 — Create the release commit

Update:

- `Cargo.toml`
- `CHANGELOG.md`
- any other explicit release metadata if the repository uses it

Then commit with a `chore(release): ...` message.

### Step 6 — Tag the release

Create an annotated tag matching the released version.

### Step 7 — Summarize release readiness

Report:

- commit sequence
- final version
- tag created
- validation status

---

## Expected Output Format

When using this skill, produce:

### Release Scope

Short summary of what is being released.

### Commit Plan

The intended semantic commit grouping.

### Release Updates

Version, changelog, and tag changes.

### Validation

Checks run before release.

### Verdict

One of:

- `Release prepared`
- `Release blocked`
