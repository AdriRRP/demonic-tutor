# status: implemented
# rules: 601.2, 117.3b, 511.3
# slices: cast-own-turn-priority-artifact-after-combat-damage.md

Feature: Cast an own-turn-priority artifact after combat damage
  Scenario: Alice casts and resolves an own-turn-priority artifact after combat damage
    Given Alice is the active player after combat damage with an own-turn-priority artifact card in hand and priority
    When Alice casts the artifact spell
    Then the spell is on the stack under Alice's control
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome EnteredBattlefield
    And the card enters Alice's battlefield
