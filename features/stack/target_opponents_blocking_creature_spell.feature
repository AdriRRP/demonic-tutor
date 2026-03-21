# status: implemented
# rules: 601.2c, 601.2f, 608.2b, 509.1h
# slices: opponents-blocking-creature-spell.md

Feature: Target an opponent's blocking creature with a supported instant spell
  Scenario: Alice can target Bob's blocker after blockers are declared
    Given Alice has priority after blockers are declared with an opponent-blocking-creature instant spell in hand
    When Alice casts the opponent-blocking-creature instant spell targeting Bob's blocker
    Then the spell is on the stack under Alice's control
    When Alice passes priority
    And Bob passes priority
    Then Bob's blocker dies
