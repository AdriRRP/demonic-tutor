# Expose Pending Choice Requests In The Public Game View

## Goal

Expose the extra player input that the current supported subset needs before a command can be completed.

## Why This Slice Existed Now

Choice-heavy cards and cleanup discard already exist, but without an explicit public prompt surface the UI would have to rediscover which extra inputs are required.

## Supported Behavior

- expose target-selection requests for supported spells and activated abilities
- expose explicit hand-card choice requests for the current discard-by-choice spell subset
- expose cleanup discard as a visible pending choice when the hand-size invariant blocks turn advancement

## Out Of Scope

- modal choose-one spells
- optional may prompts
- simultaneous-trigger ordering contracts

## Rules Support Statement

This slice does not claim general prompt support for Magic.

It only surfaces the explicit choice families already modeled in the current subset.
