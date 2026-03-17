# status: implemented
# rules: 117, 405, 601, 608
# slices: stack-priority-minimal.md

Feature: Responding with an instant spell while holding priority

  Scenario: Bob responds to Alice's spell with an instant
    Given Alice has cast a creature spell and Bob has priority with an instant in hand
    When Bob casts the instant response spell
    Then Bob's instant is on top of the stack under Bob's control
    And Alice has priority again
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome ResolvedToGraveyard
    And Alice's original spell remains on the stack
    And Alice has priority again

  Scenario: Bob cannot respond with a creature spell yet
    Given Alice has cast an instant spell and Bob has priority with a creature card in hand
    When Bob tries to cast the creature response spell
    Then the action is rejected because only instant responses are currently supported
