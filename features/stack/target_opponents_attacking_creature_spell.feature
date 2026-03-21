# status: implemented
# rules: 601.2c, 601.2f, 608.2b, 508.1m
# slices: opponents-attacking-creature-spell.md, nonlethal-opponents-attacking-target-damage.md

Feature: Target an opponent's attacking creature with a supported instant spell
  Scenario: Bob destroys Alice's attacker with an opponent-attacking-creature spell
    Given Bob has priority after attackers are declared with an opponent-attacking-creature instant spell in hand
    When Bob casts the opponent-attacking-creature instant spell targeting Alice's attacker
    Then the spell is on the stack under Bob's control
    When Bob passes priority
    And Alice passes priority
    Then Alice's attacker dies

  Scenario: Bob cannot target his blocker with an opponent-attacking-creature spell
    Given Bob has priority after blockers are declared with an opponent-attacking-creature instant spell in hand
    When Bob casts the opponent-attacking-creature instant spell targeting his blocker
    Then casting fails because the creature target is not legal for the spell

  Scenario: Bob marks nonlethal damage on Alice's attacker with an opponent-attacking-creature spell
    Given Bob has priority after attackers are declared with a nonlethal opponent-attacking-creature instant spell in hand
    When Bob casts the opponent-attacking-creature instant spell targeting Alice's attacker
    Then the spell is on the stack under Bob's control
    When Bob passes priority
    And Alice passes priority
    Then Alice's attacker has 1 damage marked and remains attacking
