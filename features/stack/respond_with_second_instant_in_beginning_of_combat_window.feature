# status: proposed
# rules: 117, 506, 601, 608
# slices: stack-response-self-stacking-wave.md

Feature: Respond with a second instant at the beginning of combat

  Scenario: Bob casts a second instant before passing priority at the beginning of Combat
    Given Bob has priority at the beginning of Combat with two instant cards in hand
    When Bob casts the first instant response spell
    And Bob casts the second instant response spell
    Then the stack contains 2 spells controlled by Bob above the original empty window state
    When Bob passes priority
    And Alice passes priority
    Then the top spell resolves first and Bob's original response remains on the stack
