# System Overview — DemonicTutor

## Purpose

DemonicTutor is a lightweight client-side application for playing, observing and analyzing Magic: The Gathering decks through real game sessions, event logging and derived statistics.

The system functions as:

* a deck playtesting laboratory
* an observable gameplay engine
* a replayable event-driven system
* a technical exploration of Rust, WebAssembly and Domain-Driven Design

The system prioritizes correctness, observability and incremental evolution.

---

# High-Level Architecture

DemonicTutor is structured around three conceptual layers:

1. Domain Core
2. Application Layer
3. Interface Layer

The domain core models gameplay behavior and rules.

The application layer orchestrates commands, aggregates, events and projections.

The interface layer handles user interaction and visualization.

Information flow:

User Interface → Application Layer → Domain Core

---

# Key System Properties

## Client-side first

The system must run entirely in the browser.

Core gameplay logic will be compiled from Rust to WebAssembly.

No backend services are required for core functionality.

---

## Deterministic domain core

The gameplay domain must behave deterministically.

Given the same command history, the system must always produce the same resulting state.

This enables:

* replay
* debugging
* analytics
* reproducibility

---

## Event-oriented behavior

Game actions produce domain events representing facts that occurred.

These events can be:

* persisted
* replayed
* analyzed
* projected into statistics

---

## Incremental rule modeling

DemonicTutor does not attempt to implement the full Magic rules from the beginning.

Rules are introduced incrementally based on the needs of each vertical slice.

---

# Core Responsibilities

## Gameplay Engine

Responsible for representing the current state of a game and validating player actions.

Responsibilities include:

* turn progression
* phase progression
* zone management
* player actions
* domain invariants

---

## Deck Representation

Responsible for representing deck definitions independently from gameplay.

Decks can be:

* created
* imported
* referenced during game initialization

---

## Event Observation

Every meaningful gameplay change may produce domain events.

These events form the basis for:

* replay
* analytics
* debugging

---

## Analytics and Statistics

Derived information based on domain events.

Examples include:

* draw probabilities
* mana curve usage
* card appearance frequency
* decision timelines

Analytics must never influence gameplay legality.

---

# Deployment Model

The system should be deployable as a static web application.

Possible deployment targets include:

* GitHub Pages
* static hosting
* local file execution
* CDN hosting

No persistent backend is required for core functionality.

---

# Long-Term Vision

Future iterations may include:

* richer rule coverage
* multiplayer synchronization
* replay visualization
* advanced deck comparison
* AI-assisted analysis of gameplay logs

These capabilities must evolve from a stable domain core.
