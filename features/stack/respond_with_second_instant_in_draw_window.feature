# status: proposed
# rules: 117, 504, 601, 608
# slices: stack-response-self-stacking-wave.md

Feature: Respond with a second instant during the draw priority window

  Scenario: Bob casts a second instant before passing priority in Draw
    Given Bob has priority in Draw with two instant cards in hand
    When Bob casts the first instant response spell
    And Bob casts the second instant response spell
    Then the stack contains 2 spells controlled by Bob above the original empty window state
    When Bob passes priority
    And Alice passes priority
    Then the top spell resolves first and Bob's original response remains on the stack
