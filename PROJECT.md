# Project — DemonicTutor

## Vision

DemonicTutor is a lightweight client-side application for testing, observing and analyzing Magic: The Gathering decks through real play sessions, event logging and live statistics.

It is both:
- a useful deck playtesting tool
- and a technical learning vehicle for Rust, WebAssembly, DDD and event-driven design

## Product identity

DemonicTutor should feel like:
- a laboratory for decks
- a rules-aware playtesting environment
- an observable gameplay engine
- a replayable and analyzable system

It should not feel like:
- a bloated generic card platform
- a backend-heavy service
- a monolithic simulator trying to solve all of Magic at once

## Primary goals

The project aims to build a system that can:
1. represent game sessions with explicit domain state
2. process player intent through commands
3. produce domain events as facts
4. persist and replay event histories
5. derive live and post-game statistics from actual play
6. remain precise, fast and architecturally clean

## Technical goals

The project is also intended to be a serious practice ground for:
- Rust
- WebAssembly
- tactical DDD
- event-driven application design
- BDD for observable behaviors
- gradual agent-assisted development

## Product goals

The product should eventually support:
- deck playtesting through real game flows
- event logging of relevant gameplay facts
- replayability from persisted history
- live statistics during a game
- post-game analysis and comparison
- future extension toward richer rules and multiplayer support

## Product philosophy

DemonicTutor must prioritize:
- correctness over breadth
- clarity over cleverness
- explicit modeling over hidden behavior
- incremental delivery over speculative architecture

## Initial scope

The initial milestone is intentionally narrow.

The repository should first provide:
- a clear product definition
- explicit constraints
- a minimal ubiquitous language
- a sound starting point for domain modeling

Later milestones will introduce:
- bounded contexts
- aggregates
- commands and events
- an initial vertical slice such as `StartGame`
- in-memory event store and event bus
- basic projections
- initial BDD scenarios

## Non-goals for the initial stage

At the beginning, DemonicTutor is not trying to be:
- a complete implementation of the full Magic Comprehensive Rules
- a generic card database manager
- a collection manager
- a marketplace
- a social platform
- a fully autonomous AI-designed system

## Modeling direction

The game domain will be approached incrementally.

Only the rule subset required by the current milestone should be modeled.

Official Magic rules may inform the model, but the implementation should not claim full rules coverage unless explicitly supported.

## Long-term direction

Once the core is solid, future iterations may include:
- richer phase and priority handling
- stack-aware interactions
- replay browser
- deck comparison tools
- multiplayer synchronization
- advanced analytics
- AI-assisted analysis of logs and replays

## Definition of success for phase 0

Phase 0 is successful when the project has:
- a stable name
- a clear written vision
- explicit constraints
- a coherent initial language for the domain
