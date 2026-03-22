use super::super::support;
use super::super::GameplayWorld;
use demonictutor::{CardDefinitionId, CastSpellCommand, LibraryCard};

impl GameplayWorld {
    pub fn setup_cast_creature_spell(&mut self) {
        self.reset_game_with_libraries(
            "bdd-cast-creature",
            support::filled_library(
                vec![
                    LibraryCard::creature(CardDefinitionId::new("bdd-grizzly-bears"), 1, 2, 2),
                    support::land_card("bdd-forest"),
                ],
                10,
            ),
            support::filled_library(Vec::new(), 10),
        );

        support::advance_to_player_first_main_satisfying_cleanup(
            &support::create_service(),
            self.game_mut(),
            "player-1",
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-grizzly-bears"));
        self.tracked_blocker_id = Some(self.hand_card_by_definition("Alice", "bdd-forest"));
    }

    pub fn setup_targeted_player_spell(&mut self) {
        self.reset_game_with_libraries(
            "bdd-targeted-player-spell",
            support::filled_library(
                vec![support::targeted_damage_instant_card("bdd-shock", 0, 2)],
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
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-shock"));
    }

    pub fn setup_lethal_targeted_player_spell(&mut self) {
        self.setup_targeted_player_spell();
        let service = support::create_service();
        service
            .adjust_player_life_effect(
                self.game_mut(),
                demonictutor::AdjustPlayerLifeEffectCommand::new(
                    Self::player_id("Alice"),
                    Self::player_id("Bob"),
                    -18,
                ),
            )
            .expect("setup life adjustment should succeed");
        self.reset_observations();
    }

    pub fn setup_targeted_creature_spell(&mut self) {
        self.reset_game_with_libraries(
            "bdd-targeted-creature-spell",
            support::filled_library(
                vec![
                    support::land_card("bdd-mountain"),
                    support::targeted_damage_instant_card("bdd-shock", 0, 2),
                ],
                10,
            ),
            support::filled_library(vec![support::creature_card("bdd-bear", 0, 2, 2)], 10),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-2",
        );
        let creature_id = self.hand_card_by_definition("Bob", "bdd-bear");
        service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id("Bob"), creature_id.clone()),
            )
            .expect("setup creature spell cast should succeed");
        self.pass_priority("Bob");
        self.pass_priority("Alice");
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-shock"));
        self.tracked_blocker_id = Some(creature_id);
        self.reset_observations();
    }

    pub fn setup_targeted_opponent_player_spell(&mut self) {
        self.reset_game_with_libraries(
            "bdd-targeted-opponent-player-spell",
            support::filled_library(
                vec![support::targeted_opponent_damage_instant_card(
                    "bdd-lava-spike",
                    0,
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
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-lava-spike"));
        self.reset_observations();
    }

    pub fn setup_cast_green_instant_with_forest(&mut self) {
        self.reset_game_with_libraries(
            "bdd-cast-green-instant",
            support::filled_library(
                vec![
                    support::green_instant_card("bdd-giant-growth", 1),
                    support::forest_card("bdd-forest"),
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
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-giant-growth"));
        self.tracked_blocker_id = Some(self.hand_card_by_definition("Alice", "bdd-forest"));
        self.reset_observations();
    }

    pub fn setup_cast_green_instant_with_mountain(&mut self) {
        self.reset_game_with_libraries(
            "bdd-cast-green-instant-red-mana",
            support::filled_library(
                vec![
                    support::green_instant_card("bdd-giant-growth", 1),
                    support::mountain_card("bdd-mountain"),
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
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-giant-growth"));
        self.tracked_blocker_id = Some(self.hand_card_by_definition("Alice", "bdd-mountain"));
        self.reset_observations();
    }

    pub fn setup_targeted_controlled_creature_spell(&mut self) {
        self.reset_game_with_libraries(
            "bdd-targeted-controlled-creature-spell",
            support::filled_library(
                vec![
                    support::creature_card("bdd-alice-bear", 0, 2, 2),
                    support::targeted_controlled_creature_damage_instant_card(
                        "bdd-reckless-surge",
                        0,
                        2,
                    ),
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
        let creature_id = self.hand_card_by_definition("Alice", "bdd-alice-bear");
        service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id("Alice"), creature_id.clone()),
            )
            .expect("setup controlled creature spell cast should succeed");
        self.pass_priority("Alice");
        self.pass_priority("Bob");
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-reckless-surge"));
        self.tracked_blocker_id = Some(creature_id);
        self.reset_observations();
    }

    pub fn setup_targeted_controlled_creature_spell_with_opponents_creature(&mut self) {
        self.reset_game_with_libraries(
            "bdd-targeted-controlled-creature-spell-opponent",
            support::filled_library(
                vec![
                    support::land_card("bdd-mountain"),
                    support::targeted_controlled_creature_damage_instant_card(
                        "bdd-reckless-surge",
                        0,
                        2,
                    ),
                ],
                10,
            ),
            support::filled_library(vec![support::creature_card("bdd-bob-bear", 0, 2, 2)], 10),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-2",
        );
        let creature_id = self.hand_card_by_definition("Bob", "bdd-bob-bear");
        service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id("Bob"), creature_id.clone()),
            )
            .expect("setup opponent creature spell cast should succeed");
        self.pass_priority("Bob");
        self.pass_priority("Alice");
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-reckless-surge"));
        self.tracked_blocker_id = Some(creature_id);
        self.reset_observations();
    }

    pub fn setup_blocking_creature_player_target_spell(&mut self) {
        self.reset_game_with_libraries(
            "bdd-targeted-blocking-player-spell",
            support::filled_library(
                vec![
                    support::land_card("bdd-mountain"),
                    support::targeted_blocking_creature_damage_instant_card(
                        "bdd-hold-the-line",
                        0,
                        2,
                    ),
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
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-hold-the-line"));
        self.reset_observations();
    }

    pub fn setup_targeted_blocking_creature_spell(&mut self) {
        self.reset_game_with_libraries(
            "bdd-targeted-blocking-creature-spell",
            support::filled_library(
                vec![
                    support::land_card("bdd-mountain"),
                    support::targeted_blocking_creature_damage_instant_card(
                        "bdd-hold-the-line",
                        0,
                        2,
                    ),
                ],
                10,
            ),
            support::filled_library(vec![support::creature_card("bdd-bear", 0, 2, 2)], 10),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-2",
        );
        let creature_id = self.hand_card_by_definition("Bob", "bdd-bear");
        service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id("Bob"), creature_id.clone()),
            )
            .expect("setup creature spell cast should succeed");
        self.pass_priority("Bob");
        self.pass_priority("Alice");
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-hold-the-line"));
        self.tracked_blocker_id = Some(creature_id);
        self.reset_observations();
    }

    pub fn setup_spell_response_stack(&mut self) {
        self.reset_game_with_libraries(
            "bdd-spell-response",
            support::filled_library(
                vec![
                    LibraryCard::creature(CardDefinitionId::new("bdd-primary-creature"), 1, 2, 2),
                    support::land_card("bdd-forest"),
                ],
                10,
            ),
            support::filled_library(vec![support::instant_card("bdd-response-instant", 0)], 10),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-primary-creature"));
        self.tracked_blocker_id = Some(self.hand_card_by_definition("Alice", "bdd-forest"));
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-response-instant"));
    }

    pub fn setup_spell_response_stack_with_two_instants(&mut self) {
        self.reset_game_with_libraries(
            "bdd-spell-response-two-instants",
            support::filled_library(
                vec![
                    LibraryCard::creature(CardDefinitionId::new("bdd-primary-creature"), 1, 2, 2),
                    support::land_card("bdd-forest"),
                ],
                10,
            ),
            support::filled_library(
                vec![
                    support::instant_card("bdd-response-instant-a", 0),
                    support::instant_card("bdd-response-instant-b", 0),
                ],
                10,
            ),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );

        let original_spell = self.hand_card_by_definition("Alice", "bdd-primary-creature");
        self.tracked_card_id = Some(original_spell.clone());
        self.tracked_blocker_id = Some(self.hand_card_by_definition("Alice", "bdd-forest"));
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-response-instant-a"));
        self.tracked_second_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-response-instant-b"));

        self.ensure_tracked_land_provides_mana();
        let outcome = service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id("Alice"), original_spell),
            )
            .expect("primary spell cast should succeed");
        self.last_spell_put_on_stack = Some(outcome.spell_put_on_stack);
        self.pass_priority("Alice");
        self.reset_observations();
        assert_eq!(
            self.game()
                .priority()
                .expect("response window should be open")
                .current_holder(),
            &Self::player_id("Bob")
        );
        assert_eq!(self.game().stack().len(), 1);
    }

    pub fn setup_spell_response_stack_with_flash_artifact(&mut self) {
        self.reset_game_with_libraries(
            "bdd-spell-response-flash-artifact",
            support::filled_library(
                vec![
                    LibraryCard::creature(CardDefinitionId::new("bdd-primary-creature"), 1, 2, 2),
                    support::land_card("bdd-forest"),
                ],
                10,
            ),
            support::filled_library(
                vec![support::flash_artifact_card("bdd-response-artifact", 0)],
                10,
            ),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );

        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-primary-creature"));
        self.tracked_blocker_id = Some(self.hand_card_by_definition("Alice", "bdd-forest"));
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-response-artifact"));
    }

    pub fn setup_spell_response_stack_with_flash_enchantment(&mut self) {
        self.reset_game_with_libraries(
            "bdd-spell-response-flash-enchantment",
            support::filled_library(
                vec![
                    LibraryCard::creature(CardDefinitionId::new("bdd-primary-creature"), 1, 2, 2),
                    support::land_card("bdd-forest"),
                ],
                10,
            ),
            support::filled_library(
                vec![support::flash_enchantment_card(
                    "bdd-response-enchantment",
                    0,
                )],
                10,
            ),
        );

        support::advance_to_player_first_main_satisfying_cleanup(
            &support::create_service(),
            self.game_mut(),
            "player-1",
        );

        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-primary-creature"));
        self.tracked_blocker_id = Some(self.hand_card_by_definition("Alice", "bdd-forest"));
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-response-enchantment"));
    }

    pub fn setup_spell_response_stack_with_mana_paid_instant(&mut self) {
        let alice_cards = vec![support::instant_card("bdd-primary-instant", 0); 10];
        let bob_cards = vec![support::instant_card("bdd-response-paid-instant", 1); 5]
            .into_iter()
            .chain(vec![support::land_card("bdd-bob-mountain"); 5])
            .collect();

        self.reset_game_with_libraries("bdd-spell-response-paid-instant", alice_cards, bob_cards);

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-2",
        );

        let bob_land = self.hand_card_by_definition("Bob", "bdd-bob-mountain");
        service
            .play_land(
                self.game_mut(),
                demonictutor::PlayLandCommand::new(Self::player_id("Bob"), bob_land.clone()),
            )
            .expect("Bob should be able to play his setup land");

        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );

        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-primary-instant"));
        self.tracked_blocker_id = None;
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-response-paid-instant"));
        self.tracked_second_response_card_id = Some(bob_land);
    }

    pub fn setup_invalid_noninstant_response(&mut self) {
        self.reset_game_with_libraries(
            "bdd-invalid-response",
            support::filled_library(vec![support::instant_card("bdd-primary-instant", 0)], 10),
            support::filled_library(
                vec![LibraryCard::creature(
                    CardDefinitionId::new("bdd-response-creature"),
                    0,
                    2,
                    2,
                )],
                10,
            ),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-primary-instant"));
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-response-creature"));
    }

    pub fn setup_invalid_sorcery_response(&mut self) {
        self.reset_game_with_libraries(
            "bdd-invalid-sorcery-response",
            support::filled_library(vec![support::instant_card("bdd-primary-instant", 0)], 10),
            support::filled_library(vec![support::sorcery_card("bdd-response-sorcery", 0)], 10),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-primary-instant"));
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-response-sorcery"));
    }

    pub fn setup_invalid_planeswalker_response(&mut self) {
        self.reset_game_with_libraries(
            "bdd-invalid-planeswalker-response",
            support::filled_library(vec![support::instant_card("bdd-primary-instant", 0)], 10),
            support::filled_library(
                vec![support::planeswalker_card("bdd-response-planeswalker", 0)],
                10,
            ),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-primary-instant"));
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-response-planeswalker"));
    }

    pub fn setup_invalid_own_turn_artifact_response(&mut self) {
        self.reset_game_with_libraries(
            "bdd-invalid-own-turn-artifact-response",
            support::filled_library(vec![support::instant_card("bdd-primary-instant", 0)], 10),
            support::filled_library(
                vec![support::own_turn_priority_artifact_card(
                    "bdd-response-own-turn-artifact",
                    0,
                )],
                10,
            ),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-primary-instant"));
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-response-own-turn-artifact"));
    }

    pub fn setup_invalid_own_turn_enchantment_response(&mut self) {
        self.reset_game_with_libraries(
            "bdd-invalid-own-turn-enchantment-response",
            support::filled_library(vec![support::instant_card("bdd-primary-instant", 0)], 10),
            support::filled_library(
                vec![support::own_turn_priority_enchantment_card(
                    "bdd-response-own-turn-enchantment",
                    0,
                )],
                10,
            ),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-primary-instant"));
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-response-own-turn-enchantment"));
    }

    pub fn setup_cast_zero_toughness_creature_spell(&mut self) {
        self.reset_game_with_libraries(
            "bdd-cast-zero-toughness-creature",
            support::filled_library(
                vec![
                    LibraryCard::creature(
                        CardDefinitionId::new("bdd-zero-toughness-creature"),
                        1,
                        1,
                        0,
                    ),
                    support::land_card("bdd-forest"),
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
        self.tracked_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-zero-toughness-creature"));
        self.tracked_blocker_id = Some(self.hand_card_by_definition("Alice", "bdd-forest"));
    }

    pub fn setup_cast_land_as_spell(&mut self) {
        self.reset_game_with_libraries(
            "bdd-cast-land",
            support::filled_library(vec![support::land_card("bdd-plains")], 10),
            support::filled_library(Vec::new(), 10),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-plains"));
    }
}
