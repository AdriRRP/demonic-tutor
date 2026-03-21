# status: implemented
# rules: 117.1a, 117.3b, 507.1
# slices: cast-flash-enchantment-in-beginning-of-combat-window.md

Feature: Cast a flash enchantment at beginning of combat
  Scenario: Alice casts and resolves a flash enchantment at BeginningOfCombat
    Given Alice is the active player in BeginningOfCombat with a flash enchantment card in hand and priority
    When Alice casts the enchantment spell
    Then the spell is on the stack under Alice's control
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome EnteredBattlefield
    And the card enters Alice's battlefield

