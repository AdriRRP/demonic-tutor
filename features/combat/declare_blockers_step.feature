# status: implemented
# rules: 509, 510
# slices: declare-blockers-step.md

Feature: Declare blockers step progresses into combat damage

  Scenario: An empty declare-blockers step advances to CombatDamage
    Given a two-player game is in DeclareBlockers
    And Alice is the active player
    When the game advances the turn
    Then the phase becomes CombatDamage
