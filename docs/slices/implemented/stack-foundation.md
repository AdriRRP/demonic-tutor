# Slice — Stack Foundation

## Goal

Introduce explicit stack and priority state into the `Game` aggregate without changing spell-casting behavior yet.

## Supported Behavior

- a started game now owns an explicit `StackZone`
- a started game now owns optional `PriorityState`
- the stack starts empty
- the game starts with no open priority window

## Out Of Scope

- putting spells onto the stack
- passing priority
- resolving stack objects
- stack-aware turn progression

## Aggregate Changes

- `Game` now contains `stack: StackZone`
- `Game` now contains `priority: Option<PriorityState>`
- stack object and priority model types are available for follow-up slices

## Rules Reference

- 117
- 405

## Rules Support Statement

This slice introduces only the aggregate-owned foundation for stack and priority. It does not yet implement public stack gameplay behavior. Spells still resolve through the pre-stack simplified flow until follow-up slices land.

## Tests

- `start_game` initializes an empty stack
- `start_game` initializes no open priority window
- BDD coverage confirms the foundation state is visible from a newly started game
