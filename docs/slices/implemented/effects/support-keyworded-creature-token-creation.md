# Support Keyworded Creature Token Creation

`SupportKeywordedCreatureTokenCreation`

## Goal

Extend the current token-creation corridor so supported effects may create one creature token with one explicit supported keyword.

## Supported Behavior

- a supported spell may create exactly one creature token on resolution
- the created token may carry one explicit supported keyword from the current keyword subset
- the token enters the battlefield with that keyword already active
- public battlefield projection surfaces the keyword normally

## Out Of Scope

- creating more than one token
- combining multiple keywords on one token from this corridor
- noncreature tokens
- generic text-driven token parsing

## Test Impact

- a supported spell can create one flying token
- the created token is still marked as a token and has the expected stats

## Rules Support Statement

This slice adds only a bounded one-token corridor with one explicit supported keyword on the created creature token.
