# status: implemented
# rules: 117.1a, 117.3b, 605.1, 605.3a, 601.2f
# slices: respond-with-paid-instant-spell.md

Feature: Respond with a paid instant spell
  Scenario: Bob taps a land and pays for an instant response in the same window
    Given Alice has cast a creature spell and Bob can pay for an instant response with a land on the battlefield
    When Bob taps his land for mana
    And Bob casts the instant response spell
    Then Bob's instant is on top of the stack under Bob's control
    And Bob has priority
    When Bob passes priority
    And Alice passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome ResolvedToGraveyard
    And Alice's original spell remains on the stack
    And Alice has priority again
