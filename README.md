# DemonicTutor

[![CI](https://github.com/AdriRRP/demonic-tutor/actions/workflows/ci.yml/badge.svg)](https://github.com/AdriRRP/demonic-tutor/actions/workflows/ci.yml)
[![Coverage](https://github.com/AdriRRP/demonic-tutor/actions/workflows/coverage.yml/badge.svg)](https://github.com/AdriRRP/demonic-tutor/actions/workflows/coverage.yml)
[![Security](https://github.com/AdriRRP/demonic-tutor/actions/workflows/security.yml/badge.svg)](https://github.com/AdriRRP/demonic-tutor/actions/workflows/security.yml)
[![codecov](https://codecov.io/gh/AdriRRP/demonic-tutor/branch/main/graph/badge.svg)](https://codecov.io/gh/AdriRRP/demonic-tutor)

DemonicTutor is a client-side application for playing, observing and analyzing Magic: The Gathering deck behavior through real game sessions, event logging and live statistics.

The project is designed as a practical laboratory for:
- deck testing
- game observability
- replayability
- rules-aware domain modeling
- analytics derived from actual play

## Current status

This repository is in active development.

Current implemented capabilities include:

- two-player game setup with opening hands and simplified London mulligan
- full phase progression: `Setup -> Untap -> Upkeep -> Draw -> FirstMain -> Combat -> SecondMain -> EndStep`
- land play, land tapping, and mana payment
- spell casting through a canonical `CastSpell` action for non-land cards
- creature runtime state with power, toughness, summoning sickness, and combat damage tracking
- attacker declaration, blocker declaration, and combat damage resolution
- event store, event bus, and gameplay log projection for replayable state changes

## Version

See `CHANGELOG.md` for release history and current version.

## Guiding idea

DemonicTutor is not intended to be a full implementation of all Magic rules from the beginning.

It is intended to become:
- a precise and fast gameplay core
- a replayable event-driven system
- a deck analysis tool based on real sessions
- a solid Rust + WebAssembly learning project

## Initial documentation

- `PROJECT.md` defines the product vision and scope.
- `CONSTRAINTS.md` defines technical and modeling restrictions.
- `DOMAIN_GLOSSARY.md` defines the initial ubiquitous language.

## Development philosophy

The system will be developed incrementally, with narrow vertical slices and explicit decisions.

The first priority is correctness and clarity of the domain model.
Breadth, advanced UX and richer rules support come later.

## Development

See `docs/development/development.md` for quality commands and panic-free policy.

Quick check: `./scripts/check-all.sh`
