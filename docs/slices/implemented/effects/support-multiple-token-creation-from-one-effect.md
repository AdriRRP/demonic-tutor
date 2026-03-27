# Support Multiple Token Creation From One Effect

`SupportMultipleTokenCreationFromOneEffect`

## Goal

Extend the current token-creation corridor so one supported spell may create more than one identical vanilla creature token as part of the same resolution.

## Supported Behavior

- a supported spell may create `N` vanilla creature tokens on resolution
- all created tokens share the same explicit supported power and toughness
- all created tokens enter under the resolving spell controller
- all created tokens participate in battlefield state and SBA normally

## Out Of Scope

- mixing different token profiles in one effect
- combining this corridor with keyworded token creation
- noncreature tokens
- text-driven token parsing

## Test Impact

- one supported spell creates two `1/1` creature tokens on resolution
- both created permanents are tokens with the expected stats

## Rules Support Statement

This slice adds only a bounded multiple-token corridor for identical vanilla creature tokens created by one supported effect.
