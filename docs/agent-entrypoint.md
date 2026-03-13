# Agent Entrypoint — DemonicTutor

Start here when working on the repository.

---

# Repository Orientation

Before proposing changes, read the project context in this order:

1. `docs/current-state.md`
2. `DOMAIN_GLOSSARY.md`
3. `CONSTRAINTS.md`
4. `docs/context-map.md`
5. `docs/aggregate-game.md`
6. `docs/vertical-slices.md`
7. `docs/slices/<relevant-slice>.md`
8. relevant `docs/adr/*.md`
9. `src/`
10. `docs/development.md` if code or quality checks are involved

Do not propose architecture or code before reading the relevant documents.

---

# Warm Start Protocol

Before producing a proposal:

1. Identify the responsibilities of the `Game` aggregate.
2. Identify the currently implemented vertical slices.
3. Verify which gameplay rules are actually implemented.
4. Check open architectural decisions in `docs/adr/`.

If something is unclear, ask before introducing new abstractions.

---

# Source of Truth

Use the following precedence when reasoning about the system:

1. Rust code in `src/`
2. ADRs in `docs/adr/`
3. `docs/current-state.md`
4. `DOMAIN_GLOSSARY.md`
5. Other documentation

If documentation contradicts the code, the code wins.

---

# Core Constraints

- Only slices listed in `docs/current-state.md` are implemented.
- Do not assume Magic rules beyond explicitly implemented slices.
- Do not introduce new aggregates without explicit justification.
- Keep documentation synchronized with architectural changes.

If bounded contexts or their relationships change, update `docs/context-map.md`, including its Mermaid diagram.

