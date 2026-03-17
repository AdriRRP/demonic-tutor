# status: implemented
# rules: 506, 507, 508
# slices: beginning-of-combat-step.md

Feature: Beginning of combat progresses into declare attackers

  Scenario: Closing the empty beginning-of-combat window advances to DeclareAttackers
    Given Alice enters Combat from FirstMain
    When Alice passes priority
    And Bob passes priority
    And the game advances the turn
    Then the phase becomes DeclareAttackers
