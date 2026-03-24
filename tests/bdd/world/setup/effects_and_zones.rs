//! BDD coverage for world setup effects and zones.

use {
    super::super::support,
    super::super::GameplayWorld,
    demonictutor::{AdjustPlayerLifeEffectCommand, Phase, PlayLandCommand},
};

impl GameplayWorld {
    pub fn setup_upkeep_with_land_on_battlefield(&mut self) {
        self.reset_game_with_libraries(
            "bdd-mana-clears-on-phase-advance",
            support::filled_library(vec![support::land_card("bdd-forest")], 40),
            support::filled_library(Vec::new(), 40),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );

        let land_id = self
            .player("Alice")
            .hand_card_at(0)
            .expect("Alice hand card should exist")
            .id()
            .clone();
        service
            .play_land(
                self.game_mut(),
                PlayLandCommand::new(Self::player_id("Alice"), land_id.clone()),
            )
            .expect("BDD setup land play should succeed");

        support::advance_to_player_phase_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
            Phase::Upkeep,
        );

        self.tracked_card_id = Some(land_id);
        self.reset_observations();
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
        self.tracked_card_id = Some(
            self.player("Alice")
                .hand_card_at(0)
                .expect("Alice hand card should exist")
                .id()
                .clone(),
        );
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

        let card_id = self
            .player(alias)
            .hand_card_at(0)
            .expect("hand card should exist")
            .id()
            .clone();
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

        let card_id = self
            .player(alias)
            .hand_card_at(0)
            .expect("hand card should exist")
            .id()
            .clone();
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
