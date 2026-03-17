# status: implemented
# rules: 510, 511
# slices: combat-damage-step.md

Feature: Combat damage step progresses into end of combat

  Scenario: An empty combat-damage step advances to EndOfCombat
    Given a two-player game is in CombatDamage
    And Alice is the active player
    When the game advances the turn
    Then the phase becomes EndOfCombat
