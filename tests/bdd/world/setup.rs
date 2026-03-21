use super::support;
use super::GameplayWorld;
use demonictutor::{
    AdjustPlayerLifeEffectCommand, CardDefinitionId, CastSpellCommand, DiscardForCleanupCommand,
    GameId, LibraryCard, Phase, StartGameCommand,
};

impl GameplayWorld {
    pub fn setup_started_game(&mut self, game_id: &str) {
        let service = support::create_service();
        let game = service
            .start_game(StartGameCommand::new(
                GameId::new(game_id),
                vec![
                    support::player_deck("player-1", "deck-1"),
                    support::player_deck("player-2", "deck-2"),
                ],
            ))
            .expect("game should start")
            .0;

        self.game = Some(game);
        self.reset_observations();
        self.reset_tracking();
    }

    pub(super) fn satisfy_cleanup_for_setup(&mut self) {
        let active_player = self.game().active_player().clone();
        let active_player_hand_size = self
            .game()
            .players()
            .iter()
            .find(|player| player.id() == &active_player)
            .expect("active player should exist")
            .hand()
            .cards()
            .len();

        if active_player_hand_size <= 7 {
            return;
        }

        let card_id = self
            .game()
            .players()
            .iter()
            .find(|player| player.id() == &active_player)
            .expect("active player should exist")
            .hand()
            .cards()[0]
            .id()
            .clone();
        self.game_mut()
            .discard_for_cleanup(DiscardForCleanupCommand::new(active_player, card_id))
            .expect("BDD setup cleanup discard should succeed");
    }

    pub fn setup_turn_state_satisfying_cleanup(
        &mut self,
        target_phase: Phase,
        target_player: &str,
        target_turn: u32,
    ) {
        self.reset_game_with_libraries(
            "bdd-turn-progression",
            support::filled_library(Vec::new(), 40),
            support::filled_library(Vec::new(), 40),
        );

        let target_player = Self::player_id(target_player);
        for _ in 0..64 {
            while self.game().phase() == &Phase::EndStep {
                let active_player = self.game().active_player().clone();
                let hand_size = self
                    .game()
                    .players()
                    .iter()
                    .find(|player| player.id() == &active_player)
                    .expect("active player should exist")
                    .hand()
                    .cards()
                    .len();
                if hand_size <= 7 {
                    break;
                }
                self.satisfy_cleanup_for_setup();
            }

            if self.game().phase() == &target_phase
                && self.game().active_player() == &target_player
                && self.game().turn_number() == target_turn
            {
                self.reset_observations();
                return;
            }

            self.advance_turn();
        }

        panic!(
            "failed to reach target state: phase={target_phase:?}, player={target_player}, turn={target_turn}"
        );
    }

    pub fn setup_upkeep_with_empty_library(&mut self) {
        self.reset_game_with_libraries(
            "bdd-empty-library-draw",
            support::filled_library(Vec::new(), 7),
            support::filled_library(Vec::new(), 7),
        );

        let service = support::create_service();
        support::advance_n_raw(&service, self.game_mut(), 2);
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::Upkeep);
        assert_eq!(self.player_library_size("Alice"), 0);
    }

    pub fn setup_first_main_with_library_size(&mut self, library_size: usize) {
        self.reset_game_with_libraries(
            "bdd-explicit-draw-effect",
            support::filled_library(Vec::new(), library_size + 8),
            support::filled_library(Vec::new(), 20),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::FirstMain);
        assert_eq!(self.player_library_size("Alice"), library_size);
    }

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

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
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

    pub fn setup_end_step_with_eight_cards_in_hand(&mut self) {
        self.reset_game_with_libraries(
            "bdd-cleanup-discard",
            support::creature_library(20),
            support::creature_library(20),
        );

        let service = support::create_service();
        support::advance_to_player_phase_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
            Phase::EndStep,
        );
        self.tracked_card_id = Some(self.player("Alice").hand().cards()[0].id().clone());
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::EndStep);
        assert_eq!(self.player_hand_size("Alice"), 8);
    }

    pub fn setup_player_at_life(&mut self, alias: &str, life: u32) {
        self.reset_game_with_libraries(
            "bdd-zero-life",
            support::filled_library(Vec::new(), 40),
            support::filled_library(Vec::new(), 40),
        );

        let current_life = self.player_life(alias);
        let delta = life.cast_signed() - current_life.cast_signed();
        let service = support::create_service();
        let outcome = service
            .adjust_player_life_effect(
                self.game_mut(),
                AdjustPlayerLifeEffectCommand::new(
                    Self::player_id("Alice"),
                    Self::player_id(alias),
                    delta,
                ),
            )
            .expect("BDD setup life adjustment should succeed");

        assert!(outcome.game_ended.is_none());
        self.reset_observations();
    }

    pub fn setup_creature_on_battlefield(&mut self, alias: &str) {
        self.reset_game_with_libraries(
            "bdd-exile-setup",
            support::filled_library(vec![support::creature_card("bdd-creature", 0, 2, 2)], 40),
            support::filled_library(Vec::new(), 40),
        );

        let service = support::create_service();
        let player_id = Self::player_id(alias);
        let player_label = if player_id == demonictutor::PlayerId::new("player-1") {
            "player-1"
        } else {
            "player-2"
        };
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            player_label,
        );

        let card_id = self.player(alias).hand().cards()[0].id().clone();
        service
            .cast_spell(
                self.game_mut(),
                demonictutor::CastSpellCommand::new(player_id, card_id.clone()),
            )
            .unwrap();
        support::resolve_top_stack_with_passes(&service, self.game_mut());

        self.tracked_card_id = Some(card_id);
        self.reset_observations();
    }

    pub fn setup_creature_in_graveyard(&mut self, alias: &str) {
        self.reset_game_with_libraries(
            "bdd-exile-graveyard-setup",
            support::filled_library(vec![support::creature_card("bdd-creature", 0, 2, 2)], 40),
            support::filled_library(Vec::new(), 40),
        );

        let service = support::create_service();
        let player_id = Self::player_id(alias);
        let player_label = if player_id == demonictutor::PlayerId::new("player-1") {
            "player-1"
        } else {
            "player-2"
        };
        support::advance_to_player_phase_satisfying_cleanup(
            &service,
            self.game_mut(),
            player_label,
            Phase::EndStep,
        );

        let card_id = self.player(alias).hand().cards()[0].id().clone();
        service
            .discard_for_cleanup(
                self.game_mut(),
                demonictutor::DiscardForCleanupCommand::new(player_id, card_id.clone()),
            )
            .unwrap();

        self.tracked_card_id = Some(card_id);
        self.reset_observations();
    }

    pub fn setup_creature_in_exile(&mut self, alias: &str) {
        self.setup_creature_on_battlefield(alias);
        let card_id = self.tracked_card_id.clone().unwrap();
        let service = support::create_service();
        service
            .exile_card(
                self.game_mut(),
                &demonictutor::ExileCardCommand::new(Self::player_id(alias), card_id, true),
            )
            .unwrap();
        self.reset_observations();
    }
}
