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

    pub fn setup_non_active_priority_after_attackers_declared_with_opponents_attacking_spell(
        &mut self,
    ) {
        let attacker_id = prepare_priority_after_attackers_declared(
            self,
            "bdd-combat-priority-attackers-opponent-attacker-spell",
            vec![attacker_card()],
            vec![
                support::land_card("bdd-bob-buffer"),
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
        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
    }

    pub fn setup_non_active_priority_after_attackers_declared_with_nonlethal_opponents_attacking_spell(
        &mut self,
    ) {
        let attacker_id = prepare_priority_after_attackers_declared(
            self,
            "bdd-combat-priority-attackers-opponent-attacker-spell-nonlethal",
            vec![demonictutor::LibraryCard::creature(
                demonictutor::CardDefinitionId::new("bdd-attacker-priority"),
                0,
                2,
                3,
            )],
            vec![
                support::land_card("bdd-bob-buffer"),
                support::targeted_opponents_attacking_creature_damage_instant_card(
                    "bdd-punish-charge",
                    0,
                    1,
                ),
            ],
        );
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-punish-charge"));
        self.pass_priority("Alice");
        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
    }

    pub fn setup_non_active_priority_after_attackers_declared_with_controlled_attacking_spell(
        &mut self,
    ) {
        let attacker_id = prepare_priority_after_attackers_declared(
            self,
            "bdd-combat-priority-attackers-controlled-attacker-response",
            vec![attacker_card()],
            vec![
                support::land_card("bdd-bob-buffer"),
                support::targeted_controlled_attacking_creature_damage_instant_card(
                    "bdd-rally-shot",
                    0,
                    2,
                ),
            ],
        );
        self.tracked_response_card_id = Some(self.hand_card_by_definition("Bob", "bdd-rally-shot"));
        self.pass_priority("Alice");
        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
    }
}
