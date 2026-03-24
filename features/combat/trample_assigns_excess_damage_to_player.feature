# status: implemented
# rules: 510, 702.19
# slices: trample-assigns-excess-damage-to-player.md

Feature: Trample assigns excess damage to the defending player
  Scenario: A blocked trampling attacker deals excess combat damage to the player
    Given Alice attacks with a trample creature and Bob blocks with a smaller creature
    When combat damage resolves
    Then Bob loses 2 life from trample damage
    And Bob's blocker dies in combat
