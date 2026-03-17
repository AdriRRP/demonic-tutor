# status: implemented
# rules: 117, 504, 601, 608
# slices: respond-with-second-instant-in-draw-window.md

Feature: Respond with a second instant during the draw priority window

  Scenario: Bob casts a second instant before passing priority in Draw
    Given Bob has priority in Draw with two instant cards in hand
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
