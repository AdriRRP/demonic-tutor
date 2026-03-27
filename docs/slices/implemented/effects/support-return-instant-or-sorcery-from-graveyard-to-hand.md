# Support Return Instant Or Sorcery From Graveyard To Hand

`SupportReturnInstantOrSorceryFromGraveyardToHand`

## Goal

Extend the current graveyard-value corridor so one supported spell may return an instant or sorcery card from a graveyard to its owner's hand.

## Supported Behavior

- a supported spell may target one card in a graveyard
- if that card is an instant or sorcery, it returns to its owner's hand on resolution
- the effect does nothing if the target is gone or no longer matches the supported card-type subset

## Out Of Scope

- returning arbitrary noncreature cards
- cost reduction or recursion loops
- ownership-changing graveyard semantics

## Test Impact

- a supported spell returns a previously used instant card from graveyard to hand

## Rules Support Statement

This slice adds only a bounded graveyard-recursion corridor for one targeted instant or sorcery card.
