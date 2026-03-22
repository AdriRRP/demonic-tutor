# status: implemented
# rules: 114.1, 114.4, 601.2c
# slices: target-opponents-creature-in-first-main.md

Feature: Target an opponents creature outside combat

  Scenario: Alice casts an opponents-creature instant at Bob's creature in FirstMain
    Given Alice is the active player in FirstMain with an opponents-creature instant spell and Bob's creature on the battlefield
    When Alice casts the opponents-creature instant spell targeting Bob's creature
    Then the spell is on the stack targeting Bob's creature
