# status: implemented
# rules: 117, 507, 601, 608
# slices: cast-second-instant-in-end-step-window.md

Feature: Cast a second instant while retaining priority in end step

  Scenario: Alice casts a second instant before passing priority in EndStep
    Given Alice is the active player in EndStep with two instant cards in hand and priority
    And the stack is empty
    When Alice casts the instant spell
    Then the spell is on the stack under Alice's control
    And Alice has priority
    When Alice casts the second instant spell
    Then the stack contains 2 spells controlled by Alice
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And Alice's original spell remains on the stack
    And Alice has priority again
