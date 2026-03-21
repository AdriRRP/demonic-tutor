use super::super::super::{support, GameplayWorld};
use super::common::{attacker_card, blocker_card, prepare_priority_after_blockers_declared};

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
}
