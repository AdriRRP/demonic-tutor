use super::super::super::{support, GameplayWorld};
use super::common::{attacker_card, prepare_priority_after_attackers_declared};

impl GameplayWorld {
    pub fn setup_non_active_priority_after_attackers_declared_with_instant(&mut self) {
        let mut bob_cards = support::filled_library(Vec::new(), 7);
        bob_cards.push(support::instant_card("bdd-window-instant", 0));

        let attacker_id = prepare_priority_after_attackers_declared(
            self,
            "bdd-combat-priority-attackers-response",
            vec![attacker_card()],
            bob_cards,
        );
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant"));
        self.pass_priority("Alice");
        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
    }

    pub fn setup_non_active_priority_after_attackers_declared_with_two_instants(&mut self) {
        let attacker_id = prepare_priority_after_attackers_declared(
            self,
            "bdd-combat-priority-attackers-response-two-instants",
            vec![attacker_card()],
            vec![
                support::instant_card("bdd-window-instant-a", 0),
                support::instant_card("bdd-window-instant-b", 0),
            ],
        );
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant-a"));
        self.tracked_second_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant-b"));
        self.pass_priority("Alice");
        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
    }
}
