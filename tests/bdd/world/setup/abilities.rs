//! BDD coverage for world setup abilities.

use {
    super::super::support,
    super::super::GameplayWorld,
    demonictutor::{CardDefinitionId, CastSpellCommand},
};

impl GameplayWorld {
    pub fn setup_activated_life_ability_in_first_main(&mut self) {
        self.reset_game_with_libraries(
            "bdd-activated-life-ability",
            support::filled_library(
                vec![support::life_gain_artifact_card("bdd-ivory-cup-lite", 0, 1)],
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

        let artifact_id = self.hand_card_by_definition("Alice", "bdd-ivory-cup-lite");
        service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id("Alice"), artifact_id),
            )
            .expect("setup artifact cast should succeed");
        self.pass_priority("Alice");
        self.pass_priority("Bob");

        self.tracked_card_id = Some(
            self.player("Alice")
                .battlefield_card_by_definition(&CardDefinitionId::new("bdd-ivory-cup-lite"))
                .expect("artifact should be on battlefield")
                .id()
                .clone(),
        );
        self.reset_observations();
    }
}
