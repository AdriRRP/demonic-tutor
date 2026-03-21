# status: implemented
# rules: 601.2c, 601.2f, 608.2b, 509.1h
# slices: target-blocking-creature-spell-foundation.md, reject-non-blocking-creature-for-block-only-spell.md, cast-targeted-blocking-creature-spell-after-blockers.md, resolve-targeted-blocking-creature-damage.md, nonlethal-blocking-target-damage.md

Feature: Target a blocking creature with a supported instant spell
  Scenario: Alice cannot target a player with a blocking-creature spell
    Given Alice is the active player in FirstMain with a blocking-creature instant spell in hand
    When Alice casts the blocking-creature instant spell targeting Bob
    Then casting fails because the spell only supports creature targets

  Scenario: Alice cannot target a creature that is not currently blocking
    Given Alice is the active player in FirstMain with a blocking-creature instant spell and Bob's creature on the battlefield
    When Alice casts the blocking-creature instant spell targeting Bob's creature
    Then casting fails because the target creature is not currently blocking

  Scenario: Alice destroys Bob's blocker after blockers are declared
    Given Bob has declared blockers and Alice still has a blocking-creature instant spell in hand with priority
    When Alice casts the blocking-creature instant spell targeting Bob's blocker
    Then the spell is on the stack under Alice's control
    When Alice passes priority
    And Bob passes priority
    Then Bob's blocker dies

  Scenario: Alice marks nonlethal damage on Bob's blocker after blockers are declared
    Given Bob has declared blockers and Alice still has a nonlethal blocking-creature instant spell in hand with priority
    When Alice casts the blocking-creature instant spell targeting Bob's blocker
    Then the spell is on the stack under Alice's control
    When Alice passes priority
    And Bob passes priority
    Then Bob's blocker has 1 damage marked and remains blocking
