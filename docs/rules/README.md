# Rules Documentation — DemonicTutor

This directory contains **repository-owned rules support material**. Its job is not to copy the Magic Comprehensive Rules; its job is to explain how DemonicTutor interprets the parts of the rules it currently models.

## Why this directory exists

DemonicTutor needs a disciplined middle layer between:

- raw code
- canonical docs about current support
- the external Magic rulebook

This directory provides that middle layer.

It helps answer:

- what rule area does a supported behavior correspond to?
- what is the repository-owned interpretation of that rule area?
- where should a new slice look before adding more behavior?

## What lives here

### [`/Users/adrianramos/Repos/demonictutor/docs/rules/rules-map.md`]( /Users/adrianramos/Repos/demonictutor/docs/rules/rules-map.md )

The rules map is the high-level index from supported repository behavior to Magic rule sections.

Use it when you want to know:

- whether a behavior is already represented somewhere
- which rule area a slice belongs to
- whether a repository rule note already exists

### [`/Users/adrianramos/Repos/demonictutor/docs/rules/notes/`]( /Users/adrianramos/Repos/demonictutor/docs/rules/notes/ )

Rules notes are focused, repository-owned interpretations of specific rule areas.

Use them when you need:

- the current simplified repository stance on a rule area
- wording that is safer than inferring directly from code
- a starting point for a new slice touching that area

Examples of current note families include turn flow, combat, casting, and exile.

## What does not live here

This directory is not:

- a backlog of the full Magic Comprehensive Rules
- a verbatim mirror of Wizards rules text
- the canonical source of implemented support on its own

Canonical truth still lives primarily in:

- [`/Users/adrianramos/Repos/demonictutor/docs/domain/current-state.md`]( /Users/adrianramos/Repos/demonictutor/docs/domain/current-state.md )
- [`/Users/adrianramos/Repos/demonictutor/docs/domain/aggregate-game.md`]( /Users/adrianramos/Repos/demonictutor/docs/domain/aggregate-game.md )
- accepted ADRs
- and, above all, the code

## Precedence and trust model

When sources conflict, use this order:

1. implemented code
2. accepted ADRs
3. canonical repository documentation
4. rules notes and other lower-level guidance
5. external rules references

Rules notes should support canonical docs, not compete with them.

## Recommended workflow for rule-heavy work

### If you are implementing a slice

1. check [`/Users/adrianramos/Repos/demonictutor/docs/domain/current-state.md`]( /Users/adrianramos/Repos/demonictutor/docs/domain/current-state.md )
2. check [`/Users/adrianramos/Repos/demonictutor/docs/rules/rules-map.md`]( /Users/adrianramos/Repos/demonictutor/docs/rules/rules-map.md )
3. read the relevant note under `notes/`
4. read the relevant implemented slice docs
5. only consult the external Comprehensive Rules if the repository-owned interpretation is missing or ambiguous

### If you are auditing consistency

Check whether:

- `rules-map.md` still matches the currently supported behavior
- notes still describe the live simplification honestly
- feature files and slice docs use the same terminology

## Relationship to `features/`

`features/` contains behavior-facing specifications. This directory contains rule-facing interpretation support.

That distinction matters:

- `docs/rules/` says what part of the rules the repository is modeling
- `features/` says what observable behavior the repository promises

For feature organization and execution, see [`/Users/adrianramos/Repos/demonictutor/features/README.md`]( /Users/adrianramos/Repos/demonictutor/features/README.md ).
