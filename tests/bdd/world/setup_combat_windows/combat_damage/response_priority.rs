//! BDD coverage for setup combat windows combat damage response priority.

use {
    super::super::super::{support, GameplayWorld},
    super::common::{attacker_card, prepare_priority_after_combat_damage},
    demonictutor::{LibraryCard, Phase},
};

impl GameplayWorld {
    pub fn setup_non_active_priority_after_combat_damage_with_instant(&mut self) {
        let mut bob_cards = support::filled_library(Vec::new(), 7);
        bob_cards.push(support::instant_card("bdd-window-instant", 0));

        let attacker_id = prepare_priority_after_combat_damage(
            self,
            "bdd-post-combat-damage-response",
            vec![attacker_card()],
            bob_cards,
        );
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant"));
        self.pass_priority("Alice");
        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::EndOfCombat);
        assert_eq!(
            self.game()
                .priority()
                .expect("combat damage should reopen priority")
                .current_holder(),
            &Self::player_id("Bob")
        );
    }

    pub fn setup_non_active_priority_in_end_of_combat_with_two_instants(&mut self) {
        let attacker_id = prepare_priority_after_combat_damage(
            self,
            "bdd-post-combat-damage-response-two-instants",
            vec![attacker_card()],
            vec![
                LibraryCard::creature(
                    demonictutor::CardDefinitionId::new("bdd-bob-buffer"),
                    0,
                    2,
                    2,
                ),
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
        assert_eq!(self.game().phase(), &Phase::EndOfCombat);
        assert_eq!(
            self.game()
                .priority()
                .expect("combat damage should reopen priority")
                .current_holder(),
            &Self::player_id("Bob")
        );
    }
}
