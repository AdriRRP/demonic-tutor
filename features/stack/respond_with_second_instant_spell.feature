# status: implemented
# rules: 117, 601, 608
# slices: respond-with-second-instant-spell.md

Feature: Respond with a second instant while holding priority on an existing stack

  Scenario: Bob casts a second instant before passing priority after responding to Alice's spell
    Given Alice has cast a creature spell and Bob has priority with two instant cards in hand
    When Bob casts the first instant response spell
    And Bob casts the second instant response spell
    Then the stack contains Alice's original spell below two spells controlled by Bob
    When Bob passes priority
    And Alice passes priority
    Then the game emits StackTopResolved
    And Bob's original response remains on the stack above Alice's original spell
    And Alice has priority again
