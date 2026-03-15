# Context Map — DemonicTutor

This document defines the bounded contexts of the DemonicTutor system and the relationships between them.

The goal is to establish clear domain boundaries and prevent responsibility leakage between parts of the system.

---

```mermaid
flowchart LR

Deck["Deck Context"]
Play["Play Context"]
Analytics["Analytics Context"]

Deck -->|deck definitions| Play
Play -->|domain events| Analytics
````

---

# Bounded Contexts

The system is divided into three primary bounded contexts:

* **Play**
* **Deck**
* **Analytics**

Each context owns its own model, language and responsibilities.

---

# Play Context (Core Domain)

The **Play context** models the runtime behavior of a match.

This is the **core domain** of DemonicTutor.

It is responsible for:

* game lifecycle
* player participation
* card zones
* turn progression
* phase progression
* action legality
* domain event production

---

## Core Concepts

Examples include:

* Game
* Player
* Turn
* Phase
* Zone
* CardInstance

The aggregate responsible for gameplay invariants is:

```
Game
```

---

# Deck Context (Supporting Domain)

The **Deck context** models deck definitions independently from gameplay.

Responsibilities include:

* deck composition
* card definitions
* deck metadata
* import/export of deck lists

Decks are **static structures** used to initialize gameplay.

---

## Core Concepts

Examples include:

* Deck
* CardDefinition
* DeckEntry

Deck data is consumed by the Play context when initializing a game.

After initialization, deck definitions are not modified by gameplay.

---

# Analytics Context (Generic Domain)

The **Analytics context** derives insights from gameplay events.

It does not influence gameplay legality.

Its role is strictly **observational**.

Responsibilities include:

* match statistics
* event timelines
* replay models
* gameplay metrics

Analytics models are projections derived from domain events.

---

# Context Relationships

The contexts interact in a simple directional flow:

```
Deck → Play → Analytics
```

### Deck → Play

Relationship:

```
Upstream → Downstream
```

Deck provides deck definitions required to initialize a match.

Play depends on deck data but does not modify it.

---

### Play → Analytics

Relationship:

```
Publisher → Subscriber
```

Play produces domain events during gameplay.

Analytics subscribes to those events to derive projections and statistics.

Analytics never modifies gameplay state.

---

# Integration Style

The integration model is intentionally simple.

Deck data is read when a game starts.

Play produces domain events.

Analytics subscribes to those events.

This architecture enables:

* replayability
* observability
* separation between gameplay and analysis
* incremental feature growth

---

# Evolution of the Context Map

New contexts may be introduced as the system evolves.

Possible future contexts include:

* **Rules Engine**
* **Replay Engine**
* **AI Analysis**

New contexts should be introduced only when the current contexts can no longer evolve safely without responsibility overlap.

---

## Maintenance Note

If the bounded contexts or their relationships change, both the textual description and the Mermaid diagram must be updated.

The diagram and text must always remain consistent.
