# Constraints — DemonicTutor

## Product constraints

- The application must be client-side first.
- The application must be deployable as a static web app.
- The system must favor low operational complexity.
- The system must prioritize speed and precision over feature breadth.
- The user experience may evolve gradually, but the core model must stay coherent.

## Domain constraints

- The project must not attempt to model all Magic rules from day one.
- Only the rules required by the current vertical slice may be modeled.
- Domain behavior must be explicit and traceable.
- Unsupported rules must never be implied as implemented.
- Card-specific complexity should be postponed unless truly needed.

## Modeling constraints

- The ubiquitous language must remain consistent across code and documentation.
- The domain model must be driven by explicit concepts, not UI convenience.
- Rules interpretations and modeling choices must be distinguished clearly.
- Observable gameplay behavior has priority over speculative abstractions.
- The project must avoid over-modeling early edge cases.

## Architectural constraints

- No business logic may live in the UI layer.
- The domain core must not depend on storage, network or rendering concerns.
- The domain core must remain deterministic.
- Event publication must not happen inside the aggregate itself.
- Analytics concerns must remain separate from gameplay rules.
- Concurrency is an optimization, not a prerequisite for correctness.
- The system must work correctly without parallelism.

## Technology constraints

- Rust is the main implementation language for the core.
- WebAssembly is the primary target for client-side execution of the core.
- The architecture must remain compatible with browser execution constraints.
- The design must not assume multithreaded browser execution by default.
- Infrastructure choices must remain simple until proven otherwise.

## Testing constraints

- Important domain behavior must be testable in isolation.
- Observable user-visible flows should later be covered with BDD scenarios.
- The project should prefer narrow vertical slices with tests over large unverified scaffolding.

## Development constraints

- Large decisions should be written down explicitly.
- The repository should evolve incrementally.
- Small coherent changes are preferred over broad speculative changes.
- New concepts should only be introduced when they solve a real modeling problem.
- The project must remain understandable without relying on agent memory.

## Agent-related constraints

- Agents may assist development, but they are not a source of domain truth.
- Agents must work from project documents and explicit instructions.
- Agents must not silently redefine scope, architecture or rules support.
- Agent outputs must be reviewable, constrained and incremental.
