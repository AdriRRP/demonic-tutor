# Agent Entrypoint — DemonicTutor

Start here when working on the repository.

## Reading order

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

## Source of truth

When reasoning about the project, use this precedence:

1. Rust code in `src/`
2. ADRs in `docs/adr/`
3. `docs/current-state.md`
4. `DOMAIN_GLOSSARY.md`
5. other documentation

## Key rules

- Only the slices listed in `docs/current-state.md` are implemented.
- Do not assume Magic rules beyond explicitly implemented slices.
- Do not introduce new aggregates without explicit justification.
- Keep documentation synchronized with architecture changes.
- If bounded contexts or their relationships change, update `docs/context-map.md`, including its Mermaid diagram.
