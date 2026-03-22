# status: implemented
# rules: 114, 117, 601, 608, 704.5a, 704.5g
# slices: spell-target-foundation.md, target-any-player-spell-foundation.md, cast-targeted-instant-at-player.md, resolve-targeted-instant-damage-to-player.md, targeted-player-damage-can-end-the-game.md, reject-missing-target-on-cast.md, reject-invalid-player-target-on-cast.md, cast-targeted-instant-at-creature.md, resolve-targeted-damage-to-creature.md, targeted-creature-damage-uses-sba-review.md

Feature: Targeted instant spells

  Scenario: Alice casts a targeted instant spell at Bob
    Given Alice is the active player in FirstMain with a targeted instant spell in hand
    And Bob is a valid target player
    When Alice casts the targeted instant spell targeting Bob
    Then the spell is on the stack under Alice's control
    And the spell is on the stack targeting Bob

  Scenario: A targeted instant spell deals damage to Bob when it resolves
    Given Alice is the active player in FirstMain with a targeted instant spell in hand
    When Alice casts the targeted instant spell targeting Bob
    And Alice passes priority
    And Bob passes priority
    Then Bob loses 2 life

  Scenario: Targeted player damage can end the game
    Given Alice is the active player in FirstMain with a lethal targeted instant spell in hand
    And Bob is at 2 life
    When Alice casts the targeted instant spell targeting Bob
    And Alice passes priority
    And Bob passes priority
    Then the game ends with Bob losing

  Scenario: A targeted instant spell cannot be cast without a target
    Given Alice is the active player in FirstMain with a targeted instant spell in hand
    When Alice casts the targeted instant spell without a target
    Then casting fails because the spell target is missing

  Scenario: A targeted instant spell cannot target a missing player
    Given Alice is the active player in FirstMain with a targeted instant spell in hand
    When Alice casts the targeted instant spell targeting a missing player
    Then casting fails because the target player does not exist

  Scenario: A targeted instant spell can destroy Bob's creature
    Given Alice is the active player in FirstMain with a targeted instant spell and Bob's creature on the battlefield
    When Alice casts the targeted instant spell targeting Bob's creature
    And Alice passes priority
    And Bob passes priority
    Then Bob's creature dies
