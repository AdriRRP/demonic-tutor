# status: implemented
# rules: 117, 509, 510, 601, 608
# slices: respond-with-second-instant-in-combat-damage-window.md

Feature: Respond with a second instant in the combat-damage window

  Scenario: Bob casts a second instant before passing priority in CombatDamage
    Given Bob has priority in CombatDamage with two instant cards in hand
    When Bob casts the first instant response spell
    And Bob casts the second instant response spell
    Then the stack contains 2 spells controlled by Bob
    When Bob passes priority
    And Alice passes priority
    Then the game emits StackTopResolved
    And Bob's original response remains on the stack
    And Alice has priority again
