# status: implemented
# rules: 601.2, 117.3b
# slices: cast-own-turn-priority-enchantment-in-beginning-of-combat-window.md

Feature: Cast an own-turn-priority enchantment at beginning of combat
  Scenario: Alice casts and resolves an own-turn-priority enchantment at BeginningOfCombat
    Given Alice is the active player in BeginningOfCombat with an own-turn-priority enchantment card in hand and priority
    When Alice casts the enchantment spell
    Then the spell is on the stack under Alice's control
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome EnteredBattlefield
    And the card enters Alice's battlefield
