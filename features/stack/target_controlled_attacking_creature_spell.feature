# status: implemented
# rules: 601.2c, 601.2f, 608.2b, 508.1m
# slices: controlled-attacking-creature-spell.md, nonlethal-controlled-attacking-target-damage.md, reject-illegal-controlled-attacking-target.md

Feature: Target a controlled attacking creature with a supported instant spell
  Scenario: Alice can target her attacker after attackers are declared
    Given Alice has priority after attackers are declared with a controlled-attacking-creature instant spell in hand
    When Alice casts the controlled-attacking-creature instant spell targeting her attacker
    Then the spell is on the stack under Alice's control
    When Alice passes priority
    And Bob passes priority
    Then Alice's attacker dies

  Scenario: Alice marks nonlethal damage on her attacker after attackers are declared
    Given Alice has priority after attackers are declared with a nonlethal controlled-attacking-creature instant spell in hand
    When Alice casts the controlled-attacking-creature instant spell targeting her attacker
    Then the spell is on the stack under Alice's control
    When Alice passes priority
    And Bob passes priority
    Then Alice's attacker has 1 damage marked and remains attacking

  Scenario: Bob cannot target Alice's attacker with a controlled-attacking-creature instant
    Given Bob has priority after attackers are declared with a controlled-attacking-creature instant spell in hand
    When Bob casts the controlled-attacking-creature instant spell targeting Alice's attacker
    Then casting fails because the creature target is not legal for the spell
