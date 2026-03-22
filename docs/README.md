# Documentation Map — DemonicTutor

This directory contains the repository’s documentation system: canonical truth, architecture, rules interpretation notes, slice history, and development guidance.

The goal of this map is simple: help a human or an agent find the right document quickly without loading the whole tree indiscriminately.

## How to use this directory

If you only need one rule:

- use **canonical docs** to understand what the repository currently claims as true
- use **slice docs** to understand how that truth evolved
- use **rules docs** to understand the repository-owned interpretation of Magic rules behind supported behavior
- use **development and agent docs** to understand how to change the system safely

## Documentation layers

### 1. Canonical project truth

These files define the stable truth of the repository.

- [`/Users/adrianramos/Repos/demonictutor/PROJECT.md`]( /Users/adrianramos/Repos/demonictutor/PROJECT.md )
  product identity and long-term intent
- [`/Users/adrianramos/Repos/demonictutor/CONSTRAINTS.md`]( /Users/adrianramos/Repos/demonictutor/CONSTRAINTS.md )
  non-negotiable modeling and architectural limits
- [`/Users/adrianramos/Repos/demonictutor/docs/domain/DOMAIN_GLOSSARY.md`]( /Users/adrianramos/Repos/demonictutor/docs/domain/DOMAIN_GLOSSARY.md )
  ubiquitous language
- [`/Users/adrianramos/Repos/demonictutor/docs/domain/context-map.md`]( /Users/adrianramos/Repos/demonictutor/docs/domain/context-map.md )
  bounded-context view
- [`/Users/adrianramos/Repos/demonictutor/docs/domain/aggregate-game.md`]( /Users/adrianramos/Repos/demonictutor/docs/domain/aggregate-game.md )
  aggregate ownership and responsibilities
- [`/Users/adrianramos/Repos/demonictutor/docs/domain/current-state.md`]( /Users/adrianramos/Repos/demonictutor/docs/domain/current-state.md )
  the current supported gameplay snapshot
- [`/Users/adrianramos/Repos/demonictutor/docs/architecture/system-overview.md`]( /Users/adrianramos/Repos/demonictutor/docs/architecture/system-overview.md )
  system layering
- [`/Users/adrianramos/Repos/demonictutor/docs/architecture/vertical-slices.md`]( /Users/adrianramos/Repos/demonictutor/docs/architecture/vertical-slices.md )
  how the repository evolves

If canonical docs disagree with lower-level docs, canonical docs win unless the code has already moved ahead and the docs need curation.

## 2. Architecture and repository structure

These files explain how the system is organized and how it should evolve.

- [`/Users/adrianramos/Repos/demonictutor/docs/architecture/game-aggregate-structure.md`]( /Users/adrianramos/Repos/demonictutor/docs/architecture/game-aggregate-structure.md )
  internal organization of the `Game` aggregate
- [`/Users/adrianramos/Repos/demonictutor/docs/architecture/agent-architecture.md`]( /Users/adrianramos/Repos/demonictutor/docs/architecture/agent-architecture.md )
  agent-assistance model and documentation precedence
- [`/Users/adrianramos/Repos/demonictutor/docs/architecture/gherkin-features.md`]( /Users/adrianramos/Repos/demonictutor/docs/architecture/gherkin-features.md )
  conventions for feature files
- [`/Users/adrianramos/Repos/demonictutor/docs/architecture/slice-template.md`]( /Users/adrianramos/Repos/demonictutor/docs/architecture/slice-template.md )
  canonical slice document template
- [`/Users/adrianramos/Repos/demonictutor/docs/architecture/adr-template.md`]( /Users/adrianramos/Repos/demonictutor/docs/architecture/adr-template.md )
  ADR template
- [`/Users/adrianramos/Repos/demonictutor/docs/architecture/adr/`]( /Users/adrianramos/Repos/demonictutor/docs/architecture/adr/ )
  accepted architectural decisions and historical decision trail

## 3. Rules interpretation support

These files connect repository behavior to Magic rules without turning the rulebook into an implementation backlog.

- [`/Users/adrianramos/Repos/demonictutor/docs/rules/README.md`]( /Users/adrianramos/Repos/demonictutor/docs/rules/README.md )
  entry point for rules docs
- [`/Users/adrianramos/Repos/demonictutor/docs/rules/rules-map.md`]( /Users/adrianramos/Repos/demonictutor/docs/rules/rules-map.md )
  supported behavior mapped to rule areas
- [`/Users/adrianramos/Repos/demonictutor/docs/rules/notes/`]( /Users/adrianramos/Repos/demonictutor/docs/rules/notes/ )
  focused repository-owned notes by rule area

## 4. Slice history

Slices are the main unit of incremental change in DemonicTutor.

- [`/Users/adrianramos/Repos/demonictutor/docs/slices/README.md`]( /Users/adrianramos/Repos/demonictutor/docs/slices/README.md )
  entry point for slice history and backlog
- [`/Users/adrianramos/Repos/demonictutor/docs/slices/implemented/README.md`]( /Users/adrianramos/Repos/demonictutor/docs/slices/implemented/README.md )
  implemented and historical slices grouped by capability
- [`/Users/adrianramos/Repos/demonictutor/docs/slices/proposals/README.md`]( /Users/adrianramos/Repos/demonictutor/docs/slices/proposals/README.md )
  live proposal backlog grouped by wave

Read slices when you want to understand:

- why a capability was introduced
- what was intentionally left out
- how a simplification entered the model

Do not use a slice doc as canonical truth if canonical docs already describe the live model more directly.

## 5. Development guidance

- [`/Users/adrianramos/Repos/demonictutor/docs/development/development.md`]( /Users/adrianramos/Repos/demonictutor/docs/development/development.md )
  coding standards, validation commands, refactor discipline, and runtime representation guidance

## 6. Agent-facing context

These live outside `docs/`, but they are part of the documentation system and matter for navigation:

- [`/Users/adrianramos/Repos/demonictutor/AGENTS.md`]( /Users/adrianramos/Repos/demonictutor/AGENTS.md )
  agent entry point
- [`/Users/adrianramos/Repos/demonictutor/.agents/context/core-agent.md`]( /Users/adrianramos/Repos/demonictutor/.agents/context/core-agent.md )
  working posture
- [`/Users/adrianramos/Repos/demonictutor/.agents/skills/README.md`]( /Users/adrianramos/Repos/demonictutor/.agents/skills/README.md )
  reusable workflows

## Recommended reading paths

### For a new human contributor

1. [`/Users/adrianramos/Repos/demonictutor/PROJECT.md`]( /Users/adrianramos/Repos/demonictutor/PROJECT.md )
2. [`/Users/adrianramos/Repos/demonictutor/CONSTRAINTS.md`]( /Users/adrianramos/Repos/demonictutor/CONSTRAINTS.md )
3. [`/Users/adrianramos/Repos/demonictutor/docs/domain/current-state.md`]( /Users/adrianramos/Repos/demonictutor/docs/domain/current-state.md )
4. [`/Users/adrianramos/Repos/demonictutor/docs/domain/aggregate-game.md`]( /Users/adrianramos/Repos/demonictutor/docs/domain/aggregate-game.md )
5. [`/Users/adrianramos/Repos/demonictutor/docs/development/development.md`]( /Users/adrianramos/Repos/demonictutor/docs/development/development.md )

### For domain work

1. glossary
2. context map
3. aggregate
4. current state
5. relevant rules notes
6. relevant slice docs

### For architectural work

1. system overview
2. vertical slices
3. game aggregate structure
4. relevant ADRs

### For agent work

1. `AGENTS.md`
2. agent architecture
3. core-agent context
4. only then the minimum task-specific docs

## Maintenance rule

This map should stay compact but trustworthy. Update it when:

- a canonical document is added, removed, or promoted
- an architectural area gets reorganized in a way that changes how people should navigate the repo
- the agent system gains or loses a stable entrypoint
