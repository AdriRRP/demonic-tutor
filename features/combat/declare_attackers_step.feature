# status: implemented
# rules: 506, 508
# slices: declare-attackers-step.md

Feature: Declare attackers step progresses into declare blockers

  Scenario: An empty declare-attackers step advances to DeclareBlockers
    Given a two-player game is in DeclareAttackers
    And Alice is the active player
    When the game advances the turn
    Then the phase becomes DeclareBlockers
