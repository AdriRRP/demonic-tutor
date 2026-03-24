//! BDD coverage for setup combat windows declare blockers response priority.

use {
    super::super::super::{support, GameplayWorld},
    super::common::{attacker_card, blocker_card, prepare_priority_after_blockers_declared},
};

impl GameplayWorld {
    pub fn setup_non_active_priority_after_blockers_declared_with_instant(&mut self) {
        let mut bob_cards = support::filled_library(vec![blocker_card()], 7);
        bob_cards.push(support::instant_card("bdd-window-instant", 0));

        prepare_priority_after_blockers_declared(
            self,
            "bdd-combat-priority-blockers-response",
            vec![attacker_card()],
            bob_cards,
        );
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant"));
        self.pass_priority("Alice");
        self.reset_observations();
    }

    pub fn setup_non_active_priority_after_blockers_declared_with_two_instants(&mut self) {
        prepare_priority_after_blockers_declared(
            self,
            "bdd-combat-priority-blockers-response-two-instants",
            vec![attacker_card()],
            vec![
                blocker_card(),
                support::instant_card("bdd-window-instant-a", 0),
                support::instant_card("bdd-window-instant-b", 0),
            ],
        );
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant-a"));
        self.tracked_second_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant-b"));
        self.pass_priority("Alice");
        self.reset_observations();
    }

    pub fn setup_non_active_priority_after_blockers_declared_with_controlled_blocking_spell(
        &mut self,
    ) {
        prepare_priority_after_blockers_declared(
            self,
            "bdd-combat-priority-blockers-controlled-blocker-spell",
            vec![attacker_card()],
            vec![
                blocker_card(),
                support::targeted_controlled_blocking_creature_damage_instant_card(
                    "bdd-guardians-rally",
                    0,
                    2,
                ),
            ],
        );
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-guardians-rally"));
        self.pass_priority("Alice");
        self.reset_observations();
    }

    pub fn setup_non_active_priority_after_blockers_declared_with_nonlethal_controlled_blocking_spell(
        &mut self,
    ) {
        prepare_priority_after_blockers_declared(
            self,
            "bdd-combat-priority-blockers-controlled-blocker-spell-nonlethal",
            vec![attacker_card()],
            vec![
                support::creature_card("bdd-blocker-priority", 0, 2, 3),
                support::targeted_controlled_blocking_creature_damage_instant_card(
                    "bdd-guardians-rally",
                    0,
                    1,
                ),
            ],
        );
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-guardians-rally"));
        self.pass_priority("Alice");
        self.reset_observations();
    }

    pub fn setup_non_active_priority_after_blockers_declared_with_opponents_attacking_spell(
        &mut self,
    ) {
        prepare_priority_after_blockers_declared(
            self,
            "bdd-combat-priority-blockers-opponent-attacker-spell",
            vec![attacker_card()],
            vec![
                blocker_card(),
                support::targeted_opponents_attacking_creature_damage_instant_card(
                    "bdd-punish-charge",
                    0,
                    2,
                ),
            ],
        );
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-punish-charge"));
        self.pass_priority("Alice");
        self.reset_observations();
    }

    pub fn setup_non_active_priority_after_blockers_declared_with_opponents_blocking_spell(
        &mut self,
    ) {
        prepare_priority_after_blockers_declared(
            self,
            "bdd-combat-priority-blockers-opponent-blocker-response",
            vec![attacker_card()],
            vec![
                blocker_card(),
                support::targeted_opponents_blocking_creature_damage_instant_card(
                    "bdd-punish-shield",
                    0,
                    2,
                ),
            ],
        );
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-punish-shield"));
        self.pass_priority("Alice");
        self.reset_observations();
    }
}
