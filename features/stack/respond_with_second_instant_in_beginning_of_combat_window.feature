# status: implemented
# rules: 117, 506, 601, 608
# slices: respond-with-second-instant-in-beginning-of-combat-window.md

Feature: Respond with a second instant at the beginning of combat

  Scenario: Bob casts a second instant before passing priority at the beginning of Combat
    Given Bob has priority at the beginning of Combat with two instant cards in hand
    And the stack is empty
    When Bob casts the first instant response spell
    Then the spell is on the stack under Bob's control
    And Bob has priority
    When Bob casts the second instant response spell
    Then the stack contains 2 spells controlled by Bob
    And Bob has priority
    When Bob passes priority
    And Alice passes priority
    Then the game emits StackTopResolved
    And Bob's original response remains on the stack
    And Alice has priority again
