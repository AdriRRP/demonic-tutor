# status: implemented
# rules: 601.2, 117.3b
# slices: cast-own-turn-priority-enchantment-in-upkeep-window.md

Feature: Cast an own-turn-priority enchantment in upkeep
  Scenario: Alice casts and resolves an own-turn-priority enchantment in Upkeep
    Given Alice is the active player in Upkeep with an own-turn-priority enchantment card in hand and priority
    When Alice casts the enchantment spell
    Then the spell is on the stack under Alice's control
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome EnteredBattlefield
    And the card enters Alice's battlefield
