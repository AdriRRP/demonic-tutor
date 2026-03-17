# status: implemented
# rules: 117, 507, 601, 608
# slices: cast-instant-in-end-step-window.md

Feature: Cast an instant during the end-step priority window

  Scenario: Alice casts and resolves an instant during EndStep
    Given Alice is the active player in EndStep with an instant card in hand and priority
    And the stack is empty
    When Alice casts the instant spell
    Then the spell is on the stack under Alice's control
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome ResolvedToGraveyard
    And Alice has priority again
