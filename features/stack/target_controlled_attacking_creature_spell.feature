# status: implemented
# rules: 601.2c, 601.2f, 608.2b, 508.1m
# slices: controlled-attacking-creature-spell.md

Feature: Target a controlled attacking creature with a supported instant spell
  Scenario: Alice can target her attacker after attackers are declared
    Given Alice has priority after attackers are declared with a controlled-attacking-creature instant spell in hand
    When Alice casts the controlled-attacking-creature instant spell targeting her attacker
    Then the spell is on the stack under Alice's control
    When Alice passes priority
    And Bob passes priority
    Then Alice's attacker dies
