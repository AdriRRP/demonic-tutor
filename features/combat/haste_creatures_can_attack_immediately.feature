Feature: Haste creatures can attack immediately
  Scenario: A creature with haste attacks on the turn it enters
    Given Alice is in DeclareAttackers with a haste creature that entered this turn
    When Alice declares that creature as an attacker
    Then the attacker declaration is accepted
    And that creature is attacking
