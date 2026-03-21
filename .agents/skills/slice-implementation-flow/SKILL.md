---
name: slice-implementation-flow
description: Orchestrate the full DemonicTutor slice flow from choosing the next coherent slice through feature/spec work, implementation, consistency updates, semantic commit, and optional push.
---

# Slice Implementation Flow Skill

## Purpose

Use this skill when the user wants a full end-to-end slice workflow rather than a narrow subtask.

Typical use cases:

- choose the next slice to implement
- design and implement a new gameplay slice
- drive work from features and rules references
- close the work with tests, docs, semantic commit, and optional push

This skill is an **orchestrator**.
It coordinates existing narrower skills instead of replacing them.

---

## Skills This Flow Uses

This flow should delegate conceptually to these repository skills as needed:

- `scenario-design`
- `slice-design`
- `ddd-review`
- `rules-consistency`
- `context-sync`
- `adr-drafting`
- `repo-curation`
- `release-prep` when the user explicitly wants release closing behavior

Use only the subset needed for the active slice.

---

## Load Required Context

### Always load

- `CONSTRAINTS.md`
- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`
- `docs/architecture/vertical-slices.md`
- `.agents/context/core-agent.md`

### Also load when needed

- `docs/domain/DOMAIN_GLOSSARY.md`
- `docs/domain/context-map.md`
- `docs/architecture/gherkin-features.md`
- `docs/rules/rules-map.md`
- relevant notes under `docs/rules/notes/`
- relevant slice docs under `docs/slices/`
- relevant `.feature` files under `features/`
- relevant ADRs
- `docs/development/development.md`
- release-related files when the user asks for commit or push

Load the minimum set that can safely support the decision.

---

## Core Principle

The flow is:

`choose slice -> specify behavior -> verify semantics -> implement -> test -> synchronize truth -> close cleanly`

Do not jump straight to code if slice choice, behavior scope, or rules meaning are still ambiguous.

---

## When To Consult The Official Comprehensive Rules

The repository's own rules map, rules notes, features, and canonical docs come first for ordinary slice work.

Consult the current official Magic Comprehensive Rules only when:

- the repository has no rules note for the rule area
- the next slice depends on wording that is semantically ambiguous
- the rule area is timing-sensitive or state-based enough that paraphrase is risky
- you suspect the external rule changed and repository truth may be stale
- the feature would otherwise overstate support because the rule nuance is unclear

Do **not** consult the full rulebook mechanically for every slice.

Do **not** turn the full rulebook into a literal backlog.

Preferred authority path:

`repo canonical docs -> repo rules map / notes -> official comprehensive rules if needed`

When using the official rules, record only the minimal repository-owned interpretation needed for the slice.

Reference point for the current official text, when needed:

- [Magic Comprehensive Rules](https://media.wizards.com/2026/downloads/MagicCompRules%2020260227.txt)

If the effective date or file changes, use the most recent official Wizards rules text instead of hard-coding an outdated copy.

---

## Flow Stages

### Stage 1 — Choose the next slice

Identify the best next slice by checking:

- current implemented behavior
- current proposed slices and features
- semantic gaps in the gameplay model
- whether one slice unlocks the next coherent behavior

Prefer slices that:

- remove a domain inconsistency
- complete an already-started gameplay loop
- strengthen semantic truthfulness
- remain small and testable

Do not pick a slice just because it is broader or more exciting.

If several slices are plausible, rank them briefly and explain the top recommendation.

---

### Stage 2 — Specify behavior

If the slice is rule-heavy or semantically subtle:

- create or refine a `.feature`
- update or create the corresponding slice document
- map rules and out-of-scope boundaries explicitly
- decide whether the feature is executable now or is an implemented/proposed reference artifact

Use:

- `scenario-design`
- `slice-design`

The feature should describe behavior.
The slice document should describe implementation scope.

For stack/priority slices that widen an existing window, explicitly decide whether the slice covers:

- active-player casting in that window
- non-active instant response after the first pass
- active-player self-stacking while retaining priority

If only one or two are in scope, state the exclusions explicitly.

For spell-effect slices that add explicit targets or richer resolution semantics, also decide explicitly:

- who chooses the target and when
- what makes the target legal at cast time
- whether resolution reuses shared life, damage, or SBA paths
- whether the relevant instant/sorcery timing windows now need broader wording

---

### Stage 3 — Check semantic integrity

Before implementation, verify:

- DDD ownership is correct
- names follow ubiquitous language
- no duplicate public action is being introduced
- rules support is not overstated

Use:

- `ddd-review`
- `rules-consistency`

If the review finds a blocking issue, correct the design before coding.

---

### Stage 4 — Implement

Implement the smallest coherent version of the slice:

- domain behavior first
- tests for observable behavior
- projections or application wiring only when needed

Prefer:

- explicit code
- minimal new surface area
- canonical events over shortcut-specific ones
- module splits by domain capability when a file has become crowded enough to hide the slice
- small helpers in application orchestration instead of generic event-publication frameworks

Avoid:

- speculative infrastructure
- generic rule engines
- broad refactors unless they are directly required for correctness

---

### Stage 5 — Validate

Run the relevant tests while implementing, then run repository-wide validation before closing.

Minimum closure check:

- targeted tests for the slice
- `./scripts/check-all.sh`

If validation fails, resolve before moving to documentation closure.

When a slice includes an internal structural refactor:

- stage both added modules and removed superseded files
- search for stale file-path references in architecture docs and agent skills before closing

---

### Stage 6 — Synchronize repository truth

After the code is correct, determine which owned-truth documents changed.

Update only what is required:

- current state
- aggregate docs
- glossary
- rules notes
- features
- slice docs
- ADRs
- agent context
- skills

When many adjacent stack slices accumulate, prefer also checking whether:

- public summaries like `README.md` and `current-state.md` now understate supported timing
- older foundational stack docs need a historical note or narrower wording
- feature indexes and rules maps need broader umbrella wording in addition to per-slice additions

Use:

- `context-sync`
- `adr-drafting` if the slice creates or supersedes a meaningful architectural decision
- `repo-curation` when the change set is broad enough to risk repository drift

---

### Stage 7 — Close cleanly

If the user asked for closure beyond implementation:

- create semantic commit(s)
- optionally prepare release metadata
- optionally push

Use:

- `release-prep` for commit/release orchestration

Do not push or cut a release unless the user explicitly wants that step.

---

## Stop Conditions

Pause and realign if any of these are true:

- the best next slice is genuinely ambiguous
- the rule meaning is unclear even after checking repository truth
- implementation would require multiple unrelated behaviors
- the slice would overstate unsupported Magic rules
- the change unexpectedly becomes architectural rather than slice-sized

In those cases, produce a concise recommendation and stop broadening scope blindly.

---

## Expected Output Format

When using this skill, produce:

### Next Slice Recommendation

What slice should come next and why.

### Flow Plan

Which stages will be performed in this run.

### Artifacts

Which features, slice docs, code, tests, and docs will change.

### Validation

What checks will confirm closure.

### Closure Status

One of:

- `Design ready`
- `Implemented and synchronized`
- `Blocked pending clarification`
