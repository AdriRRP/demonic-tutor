# status: implemented
# rules: 117.1a, 117.3b, 509.5
# slices: cast-flash-artifact-after-blockers.md

Feature: Cast a flash artifact after blockers are declared
  Scenario: Alice casts and resolves a flash artifact after blockers
    Given Alice is the active player after declaring blockers with a flash artifact card in hand and priority
    When Alice casts the artifact spell
    Then the spell is on the stack under Alice's control
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits SpellCast with outcome EnteredBattlefield
    And the card enters Alice's battlefield
