# status: implemented
# rules: 117, 510, 511, 601, 608
# slices: cast-instant-after-combat-damage.md

Feature: Cast an instant after combat damage resolves

  Scenario: Alice casts and resolves an instant after combat damage
    Given combat damage has resolved and Alice still has an instant card in hand with priority
    And the stack is empty
    When Alice casts the instant spell
    Then the spell is on the stack under Alice's control
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome ResolvedToGraveyard
    And Alice has priority again
