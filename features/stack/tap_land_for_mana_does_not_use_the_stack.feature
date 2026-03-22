# status: implemented
# rules: 117.1a, 117.3b, 605.1, 605.3a
# slices: mana-abilities-do-not-use-the-stack.md

Feature: Tapping a land for mana does not use the stack
  Scenario: Bob taps a land for mana while holding priority on Alice's spell
    Given Alice has cast a creature spell and Bob can pay for an instant response with a land on the battlefield
    When Bob taps his land for mana
    Then Alice's original spell remains on the stack
    And Bob has priority
