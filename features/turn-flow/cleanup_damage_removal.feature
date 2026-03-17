# status: implemented
# rules: 120.6, 514
# slices: cleanup-damage-removal.md

Feature: Marked damage is cleared when the turn ends
  In order to keep combat damage transient
  As the play bounded context
  Marked damage is removed from surviving creatures when the game leaves the turn

  Scenario: Surviving creature loses marked damage at end of turn
    Given a creature survives combat with damage marked on it
    When the game advances from EndStep to the next player's Untap
    Then that surviving creature has no damage marked on it

