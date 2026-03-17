# status: implemented
# rules: 506, 508, 509, 510
# slices: combat-damage.md

Feature: Combat damage marking
  In order to keep combat state semantically correct
  As the play bounded context
  Combat damage is marked on the creatures that actually receive it

  Scenario: Blocked creatures mark damage on each other
    Given Alice attacks with a creature
    And Bob blocks with a creature
    When combat damage resolves
    Then the attacker's damage is marked on the blocking creature
    And the blocker's damage is marked on the attacking creature
    And the game emits CombatDamageResolved

  Scenario: Unblocked damage reduces defending player life
    Given Alice attacks with an unblocked creature
    And Bob is the defending player
    When combat damage resolves
    Then Bob loses life equal to the attacker's power
    And the game emits CombatDamageResolved

  Scenario: Unblocked lethal combat damage ends the game
    Given Alice attacks with an unblocked creature
    And Bob is at 3 life as the defending player
    When combat damage resolves
    Then Bob loses the game due to zero life
    And the game emits GameEnded for ZeroLife
