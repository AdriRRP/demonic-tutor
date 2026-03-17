# ADR 0014 — Canonical gameplay actions prefer real domain semantics over convenience duplicates

## Status
Accepted

## Context

The play context had accumulated command and documentation shortcuts that were convenient to implement but weaker than the real game language.

The clearest example was `PlayCreatureCommand`, which duplicated the concept already represented by `CastSpellCommand`. In Magic, creatures are spells while being cast, and only lands are played rather than cast.

Keeping both entrypoints created avoidable problems:

- duplicate legality and validation paths
- drift between documentation and gameplay terminology
- weaker event semantics because projections had to infer the real action from a shortcut
- pressure to preserve technically convenient APIs after the real domain action was already clear

## Decision

When one gameplay concept has a clear domain-canonical action, the repository should use that action directly and remove convenience duplicates.

For the current play model this means:

- non-land cards are cast through `CastSpell`
- lands are played through `PlayLand`
- duplicate commands, events, and slice docs should be removed or marked superseded once the canonical action is established

This rule applies to future gameplay modeling as well:

- prefer the real game action name
- avoid parallel commands for the same domain fact
- enrich canonical events rather than inventing shortcut-specific events

## Consequences

### Positive

- the public domain model better matches ubiquitous language
- command handling and legality checks stay centralized
- event streams are easier to understand in replay and projections
- future slices have a clearer standard for naming and scope

### Negative

- temporary convenience APIs may need to be removed once the domain model matures
- historical slice documents must be curated so they do not continue to look current

## Notes

This decision does not forbid temporary simplifications. It requires those simplifications to stay semantically honest and to be retired once the canonical domain action is known.
