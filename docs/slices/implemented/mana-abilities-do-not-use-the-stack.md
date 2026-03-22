# Slice — Mana Abilities Do Not Use The Stack

## Goal

Make the current land-for-mana corridor explicit as a stack-free action while a priority window is open.

## Supported behavior

- the current priority holder may tap a land for generic mana in the currently supported open stack windows
- tapping that land does not put a new object onto the stack
- tapping that land does not pass or change priority by itself
- the previously pending spell remains on the stack unchanged

## Current scope

This slice only covers the currently supported land mana action:

- tapping a land already on the battlefield
- adding 1 generic mana to that player's transient mana pool
- proving the behavior in an open response window on an existing stack

It does not introduce broader mana abilities, colored mana, or richer resource actions.

## Rules reference

- 605.1
- 605.3a
- 117.1a
- 117.3b

## Rules support statement

For the currently supported land mana action, DemonicTutor treats mana production as stack-free: the action adds mana immediately, leaves the current stack unchanged, and keeps priority with the acting player.
