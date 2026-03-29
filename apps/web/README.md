# apps/web — DemonicTutor browser shell

This directory contains the current browser-facing client for DemonicTutor.

It is intentionally thin:

- Solid for UI composition
- Vite for dev/build
- the real Rust engine embedded through WebAssembly

Gameplay rules do not live here.

The client consumes the public gameplay contract exported by the Rust crate through the browser adapter in:

- `src/interfaces/web/`

For the architectural rationale, see:

- `docs/architecture/web-client.md`

## Commands

Install dependencies:

```bash
cd apps/web
npm install
```

Start the development server:

```bash
cd apps/web
npm run dev
```

Build the client:

```bash
cd apps/web
npm run build
```

The web scripts regenerate the wasm package from the repository root crate before bundling.

Quality commands:

```bash
cd apps/web
npm run format
npm run format:check
npm run lint
npm run typecheck
npm run build
npm run check
npm run audit
npm run deps:check
```

`deps:check` only fails when the installed frontend dependencies are behind the exact versions pinned in `package.json`. Newer releases are reported for Dependabot to handle without leaving CI permanently red.

The repository CI now treats these frontend checks as first-class quality gates, and Dependabot also tracks `apps/web` dependencies directly.

## Current Scope

Today this app is a first integration shell.

It proves:

- browser embedding of the Rust engine
- public snapshot rendering
- legal-action and choice-request projection
- replay/timeline rendering

The next major slice is expected to be a playable two-player hot-seat arena for generating real logs and exercising more gameplay interactions from the UI.

## Guardrails

- do not duplicate gameplay rules in TypeScript
- do not invent a second public contract inside the client
- keep browser-specific Rust glue in `src/interfaces/web/`
- prefer small vertical slices over broad frontend framework work
