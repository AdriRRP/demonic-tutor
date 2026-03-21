use super::super::support;
use super::super::GameplayWorld;
use demonictutor::{CardDefinitionId, CastSpellCommand, LibraryCard, Phase};

impl GameplayWorld {
    pub fn setup_priority_after_attackers_declared(&mut self) {
        self.reset_game_with_libraries(
            "bdd-combat-priority-attackers",
            support::filled_library(
                vec![LibraryCard::creature(
                    CardDefinitionId::new("bdd-attacker-priority"),
                    0,
                    2,
                    2,
                )],
                10,
            ),
            support::filled_library(Vec::new(), 10),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        let attacker_id = self.hand_card_by_definition("Alice", "bdd-attacker-priority");
        service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id("Alice"), attacker_id.clone()),
            )
            .expect("attacker cast should succeed");
        support::resolve_top_stack_with_passes(&service, self.game_mut());

        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-2",
        );
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        support::advance_turn_raw(&service, self.game_mut());
        support::close_empty_priority_window(&service, self.game_mut());
        support::advance_turn_raw(&service, self.game_mut());
        service
            .declare_attackers(
                self.game_mut(),
                demonictutor::DeclareAttackersCommand::new(
                    Self::player_id("Alice"),
                    vec![attacker_id.clone()],
                ),
            )
            .expect("declare attackers should succeed");

        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
    }

    pub fn setup_priority_after_attackers_declared_with_instant(&mut self) {
        self.reset_game_with_libraries(
            "bdd-combat-priority-attackers-instant",
            support::filled_library(
                vec![
                    LibraryCard::creature(CardDefinitionId::new("bdd-attacker-priority"), 0, 2, 2),
                    support::instant_card("bdd-window-instant", 0),
                ],
                10,
            ),
            support::filled_library(Vec::new(), 10),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        let attacker_id = self.hand_card_by_definition("Alice", "bdd-attacker-priority");
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-window-instant"));
        service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id("Alice"), attacker_id.clone()),
            )
            .expect("attacker cast should succeed");
        support::resolve_top_stack_with_passes(&service, self.game_mut());

        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-2",
        );
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        support::advance_turn_raw(&service, self.game_mut());
        support::close_empty_priority_window(&service, self.game_mut());
        support::advance_turn_raw(&service, self.game_mut());
        service
            .declare_attackers(
                self.game_mut(),
                demonictutor::DeclareAttackersCommand::new(
                    Self::player_id("Alice"),
                    vec![attacker_id.clone()],
                ),
            )
            .expect("declare attackers should succeed");

        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
    }

    pub fn setup_priority_after_attackers_declared_with_two_instants(&mut self) {
        self.reset_game_with_libraries(
            "bdd-combat-priority-attackers-two-instants",
            support::filled_library(
                vec![
                    LibraryCard::creature(CardDefinitionId::new("bdd-attacker-priority"), 0, 2, 2),
                    support::instant_card("bdd-window-instant-a", 0),
                    support::instant_card("bdd-window-instant-b", 0),
                ],
                10,
            ),
            support::filled_library(Vec::new(), 10),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        let attacker_id = self.hand_card_by_definition("Alice", "bdd-attacker-priority");
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-window-instant-a"));
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-window-instant-b"));
        service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id("Alice"), attacker_id.clone()),
            )
            .expect("attacker cast should succeed");
        support::resolve_top_stack_with_passes(&service, self.game_mut());

        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-2",
        );
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        support::advance_turn_raw(&service, self.game_mut());
        support::close_empty_priority_window(&service, self.game_mut());
        support::advance_turn_raw(&service, self.game_mut());
        service
            .declare_attackers(
                self.game_mut(),
                demonictutor::DeclareAttackersCommand::new(
                    Self::player_id("Alice"),
                    vec![attacker_id.clone()],
                ),
            )
            .expect("declare attackers should succeed");

        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::DeclareBlockers);
        assert_eq!(
            self.game()
                .priority()
                .expect("attackers declaration should open priority")
                .current_holder(),
            &Self::player_id("Alice")
        );
        assert!(self.game().stack().is_empty());
    }

    pub fn setup_non_active_priority_after_attackers_declared_with_instant(&mut self) {
        let mut bob_cards = support::filled_library(Vec::new(), 7);
        bob_cards.push(support::instant_card("bdd-window-instant", 0));
        self.reset_game_with_libraries(
            "bdd-combat-priority-attackers-response",
            support::filled_library(
                vec![LibraryCard::creature(
                    CardDefinitionId::new("bdd-attacker-priority"),
                    0,
                    2,
                    2,
                )],
                10,
            ),
            bob_cards,
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        let attacker_id = self.hand_card_by_definition("Alice", "bdd-attacker-priority");
        service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id("Alice"), attacker_id.clone()),
            )
            .expect("attacker cast should succeed");
        support::resolve_top_stack_with_passes(&service, self.game_mut());

        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-2",
        );

        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        support::advance_turn_raw(&service, self.game_mut());
        support::close_empty_priority_window(&service, self.game_mut());
        support::advance_turn_raw(&service, self.game_mut());
        service
            .declare_attackers(
                self.game_mut(),
                demonictutor::DeclareAttackersCommand::new(
                    Self::player_id("Alice"),
                    vec![attacker_id.clone()],
                ),
            )
            .expect("declare attackers should succeed");

        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant"));
        self.pass_priority("Alice");
        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
    }

    pub fn setup_non_active_priority_after_attackers_declared_with_two_instants(&mut self) {
        self.reset_game_with_libraries(
            "bdd-combat-priority-attackers-response-two-instants",
            support::filled_library(
                vec![LibraryCard::creature(
                    CardDefinitionId::new("bdd-attacker-priority"),
                    0,
                    2,
                    2,
                )],
                10,
            ),
            support::filled_library(
                vec![
                    support::instant_card("bdd-window-instant-a", 0),
                    support::instant_card("bdd-window-instant-b", 0),
                ],
                7,
            ),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        let attacker_id = self.hand_card_by_definition("Alice", "bdd-attacker-priority");
        service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id("Alice"), attacker_id.clone()),
            )
            .expect("attacker cast should succeed");
        support::resolve_top_stack_with_passes(&service, self.game_mut());

        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-2",
        );
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        support::advance_turn_raw(&service, self.game_mut());
        support::close_empty_priority_window(&service, self.game_mut());
        support::advance_turn_raw(&service, self.game_mut());
        service
            .declare_attackers(
                self.game_mut(),
                demonictutor::DeclareAttackersCommand::new(
                    Self::player_id("Alice"),
                    vec![attacker_id.clone()],
                ),
            )
            .expect("declare attackers should succeed");

        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant-a"));
        self.tracked_second_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant-b"));
        self.pass_priority("Alice");
        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
    }
}
