# Support Cannot Block This Turn Effects

`SupportCannotBlockThisTurnEffects`

## Goal

Add the first explicit temporary combat restriction that prevents one target creature from blocking for the rest of the turn.

## Supported Behavior

- a supported spell may target exactly one creature on the battlefield
- if the target is still legal on resolution, that creature cannot be declared as a blocker for the rest of the current turn
- the restriction is cleared during normal end-of-turn cleanup
- if the target is gone on resolution, the spell has no effect

## Out Of Scope

- multiple-target falter effects
- static or attached `cannot block`
- removing a blocker from combat after it has already blocked
- broader attack-and-block combined restrictions

## Test Impact

- the affected creature cannot be declared as a blocker later that turn
- the effect is target-based and fizzles cleanly if the target disappears
- turn cleanup removes the temporary restriction

## Rules Support Statement

This slice adds only a bounded `target creature can't block this turn` spell subset for one battlefield creature.
