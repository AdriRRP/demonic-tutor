# status: implemented
# rules: 601.2c, 601.2f, 608.2b, 509.1h
# slices: controlled-blocking-creature-spell.md

Feature: Target a controlled blocking creature with a supported instant spell
  Scenario: Bob cannot target Alice's attacker with a controlled-blocking-creature spell
    Given Bob has priority after blockers are declared with a controlled-blocking-creature instant spell in hand
    When Bob casts the controlled-blocking-creature instant spell targeting Alice's attacker
    Then casting fails because the creature target is not legal for the spell

  Scenario: Bob destroys his blocker with a controlled-blocking-creature spell
    Given Bob has priority after blockers are declared with a controlled-blocking-creature instant spell in hand
    When Bob casts the controlled-blocking-creature instant spell targeting his blocker
    Then the spell is on the stack under Bob's control
    When Bob passes priority
    And Alice passes priority
    Then Bob's blocker dies
