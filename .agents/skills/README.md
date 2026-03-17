# Agent Skills

This directory contains reusable workflows for agents working in the DemonicTutor repository.

Skills are specialized procedures used by the core agent.

Current skills:

- `slice-design` — design minimal truthful slices
- `context-sync` — synchronize code changes with owned documentation truth
- `ddd-review` — review bounded contexts, aggregates, invariants, and canonical actions
- `adr-drafting` — record meaningful architectural decisions and supersessions
- `rules-consistency` — ensure documented Magic support stays truthful
- `repo-curation` — close broad refactors or release-prep work cleanly
- `release-prep` — group semantic commits and cut validated releases
- `scenario-design` — design truthful Gherkin gameplay features tied to rules and slices
- `slice-implementation-flow` — orchestrate the full slice workflow from choice through implementation and closure

Common repository-wide guardrails these skills should reinforce:

- prefer canonical gameplay actions over convenience duplicates
- keep partial stack/priority support explicit and enforced
- distinguish executable features from implemented reference features
- mark historical or superseded docs honestly
- prefer explicit Rust and DDD structure over speculative generic frameworks
