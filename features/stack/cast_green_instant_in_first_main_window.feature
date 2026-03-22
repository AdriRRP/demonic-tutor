# status: implemented
# rules: 106.1, 106.4, 601.2f
# slices: colored-mana-foundation.md

Feature: Cast a green instant with matching colored mana

  Scenario: Alice casts and resolves a green instant using green mana from a Forest
    Given Alice is the active player in FirstMain with a green instant card in hand and priority
    And Alice has enough mana to pay its cost
    When Alice casts the instant spell
    Then the spell is on the stack under Alice's control
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome ResolvedToGraveyard
    And Alice has priority again
