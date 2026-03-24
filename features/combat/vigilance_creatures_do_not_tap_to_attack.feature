Feature: Vigilance creatures do not tap to attack
  Scenario: A creature with vigilance attacks without tapping
    Given Alice is in DeclareAttackers with a vigilance creature without summoning sickness
    When Alice declares that creature as an attacker
    Then the attacker declaration is accepted
    And that creature is attacking
    And that creature remains untapped
