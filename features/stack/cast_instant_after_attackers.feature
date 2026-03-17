# status: implemented
# rules: 117, 508, 601, 608
# slices: cast-instant-after-attackers.md

Feature: Cast an instant after attackers are declared

  Scenario: Alice casts and resolves an instant after declaring attackers
    Given Alice has declared attackers and still has an instant card in hand with priority
    And the stack is empty
    When Alice casts the instant spell
    Then the spell is on the stack under Alice's control
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome ResolvedToGraveyard
    And Alice has priority again
