#[path = "../unit/support/mod.rs"]
pub mod support;

use demonictutor::{
    AdjustLifeCommand, AdvanceTurnCommand, AdvanceTurnOutcome, CardDefinitionId, CardDiscarded,
    CardDrawn, CardInstance, CardInstanceId, CastSpellCommand, CombatDamageResolved, CreatureDied,
    DealOpeningHandsCommand, DiscardForCleanupCommand, Game, GameEnded, GameId, LibraryCard, Phase,
    PlayLandCommand, PlayerId, ResolveCombatDamageCommand, SpellCast, StartGameCommand,
    TapLandCommand, TurnProgressed,
};

#[derive(Debug, Default, cucumber::World)]
pub struct GameplayWorld {
    game: Option<Game>,
    pub last_turn_progressed: Option<TurnProgressed>,
    pub last_game_ended: Option<GameEnded>,
    pub last_card_drawn: Option<CardDrawn>,
    pub last_card_discarded: Option<CardDiscarded>,
    pub last_spell_cast: Option<SpellCast>,
    pub last_combat_damage: Option<CombatDamageResolved>,
    pub last_creature_died: Vec<CreatureDied>,
    pub last_error: Option<String>,
    pub pre_advance_hand_size: Option<usize>,
    pub post_advance_hand_size: Option<usize>,
    pub tracked_card_id: Option<CardInstanceId>,
    pub tracked_attacker_id: Option<CardInstanceId>,
    pub tracked_blocker_id: Option<CardInstanceId>,
    pub blocker_assignments: Vec<(CardInstanceId, CardInstanceId)>,
}

impl GameplayWorld {
    pub fn is_initialized(&self) -> bool {
        self.game.is_some()
    }

    pub fn game(&self) -> &Game {
        self.game
            .as_ref()
            .expect("world game should be initialized")
    }

    pub fn game_mut(&mut self) -> &mut Game {
        self.game
            .as_mut()
            .expect("world game should be initialized")
    }

    pub fn player_id(alias: &str) -> PlayerId {
        match alias {
            "Alice" => PlayerId::new("player-1"),
            "Bob" => PlayerId::new("player-2"),
            _ => panic!("unknown player alias: {alias}"),
        }
    }

    pub fn phase_from_name(name: &str) -> Phase {
        match name {
            "Untap" => Phase::Untap,
            "Draw" => Phase::Draw,
            "FirstMain" => Phase::FirstMain,
            "EndStep" => Phase::EndStep,
            other => panic!("unsupported phase in BDD suite: {other}"),
        }
    }

    pub fn player(&self, alias: &str) -> &demonictutor::domain::play::game::Player {
        let player_id = Self::player_id(alias);
        self.game()
            .players()
            .iter()
            .find(|player| player.id() == &player_id)
            .unwrap_or_else(|| panic!("player should exist: {player_id}"))
    }

    pub fn hand_contains(&self, alias: &str, card_id: &CardInstanceId) -> bool {
        self.player(alias)
            .hand()
            .cards()
            .iter()
            .any(|card: &CardInstance| card.id() == card_id)
    }

    pub fn battlefield_contains(&self, alias: &str, card_id: &CardInstanceId) -> bool {
        self.player(alias)
            .battlefield()
            .cards()
            .iter()
            .any(|card: &CardInstance| card.id() == card_id)
    }

    pub fn graveyard_contains(&self, alias: &str, card_id: &CardInstanceId) -> bool {
        self.player(alias)
            .graveyard()
            .cards()
            .iter()
            .any(|card: &CardInstance| card.id() == card_id)
    }

    pub fn battlefield_card(&self, alias: &str, card_id: &CardInstanceId) -> &CardInstance {
        self.player(alias)
            .battlefield()
            .cards()
            .iter()
            .find(|card: &&CardInstance| card.id() == card_id)
            .unwrap_or_else(|| panic!("battlefield card should exist: {card_id}"))
    }

    pub fn hand_card_by_definition(&self, alias: &str, definition_id: &str) -> CardInstanceId {
        let definition_id = CardDefinitionId::new(definition_id);
        self.player(alias)
            .hand()
            .cards()
            .iter()
            .find(|card: &&CardInstance| card.definition_id() == &definition_id)
            .unwrap_or_else(|| panic!("hand card should exist: {definition_id}"))
            .id()
            .clone()
    }

    pub fn player_hand_size(&self, alias: &str) -> usize {
        self.player(alias).hand().cards().len()
    }

    pub fn player_library_size(&self, alias: &str) -> usize {
        self.player(alias).library().len()
    }

    pub fn player_life(&self, alias: &str) -> u32 {
        self.player(alias).life()
    }

    pub fn reset_observations(&mut self) {
        self.last_turn_progressed = None;
        self.last_game_ended = None;
        self.last_card_drawn = None;
        self.last_card_discarded = None;
        self.last_spell_cast = None;
        self.last_combat_damage = None;
        self.last_creature_died.clear();
        self.last_error = None;
        self.pre_advance_hand_size = None;
        self.post_advance_hand_size = None;
    }

    pub fn reset_tracking(&mut self) {
        self.tracked_card_id = None;
        self.tracked_attacker_id = None;
        self.tracked_blocker_id = None;
        self.blocker_assignments.clear();
    }

    fn reset_game_with_libraries(
        &mut self,
        game_id: &str,
        alice_cards: Vec<LibraryCard>,
        bob_cards: Vec<LibraryCard>,
    ) {
        let service = support::create_service();
        let mut game = service
            .start_game(StartGameCommand::new(
                GameId::new(game_id),
                vec![
                    support::player_deck("player-1", "deck-1"),
                    support::player_deck("player-2", "deck-2"),
                ],
            ))
            .expect("game should start")
            .0;

        service
            .deal_opening_hands(
                &mut game,
                &DealOpeningHandsCommand::new(vec![
                    support::player_library("player-1", alice_cards),
                    support::player_library("player-2", bob_cards),
                ]),
            )
            .expect("opening hands should be dealt");

        self.game = Some(game);
        self.reset_observations();
        self.reset_tracking();
    }

    fn satisfy_cleanup_for_setup(&mut self) {
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

    pub fn setup_draw_phase_with_empty_library(&mut self) {
        self.reset_game_with_libraries(
            "bdd-empty-library-draw",
            support::filled_library(Vec::new(), 7),
            support::filled_library(Vec::new(), 7),
        );

        let service = support::create_service();
        support::advance_n_raw(&service, self.game_mut(), 3);
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::Draw);
        assert_eq!(self.player_library_size("Alice"), 0);
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

    pub fn ensure_tracked_land_provides_mana(&mut self) {
        let service = support::create_service();
        let land_id = self
            .tracked_blocker_id
            .clone()
            .expect("tracked land card should exist");

        service
            .play_land(
                self.game_mut(),
                PlayLandCommand::new(Self::player_id("Alice"), land_id.clone()),
            )
            .expect("playing land should succeed");

        service
            .tap_land(
                self.game_mut(),
                TapLandCommand::new(Self::player_id("Alice"), land_id),
            )
            .expect("tapping land should succeed");
    }

    fn setup_combat(
        &mut self,
        game_id: &str,
        attacker_definition: &str,
        attacker_card: LibraryCard,
        blocker_definition: Option<&str>,
        blocker_card: Option<LibraryCard>,
    ) {
        self.reset_game_with_libraries(
            game_id,
            support::filled_library(vec![attacker_card], 10),
            support::filled_library(blocker_card.into_iter().collect(), 10),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        let attacker_id = self.hand_card_by_definition("Alice", attacker_definition);
        service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id("Alice"), attacker_id.clone()),
            )
            .expect("attacker cast should succeed");

        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-2",
        );

        if let Some(blocker_definition) = blocker_definition {
            let blocker_id = self.hand_card_by_definition("Bob", blocker_definition);
            service
                .cast_spell(
                    self.game_mut(),
                    CastSpellCommand::new(Self::player_id("Bob"), blocker_id.clone()),
                )
                .expect("blocker cast should succeed");
            self.tracked_blocker_id = Some(blocker_id);
        }

        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        service
            .advance_turn(self.game_mut(), AdvanceTurnCommand::new())
            .expect("advance to combat should succeed");

        service
            .declare_attackers(
                self.game_mut(),
                demonictutor::DeclareAttackersCommand::new(
                    Self::player_id("Alice"),
                    vec![attacker_id.clone()],
                ),
            )
            .expect("declare attackers should succeed");

        self.tracked_attacker_id = Some(attacker_id.clone());

        if let Some(blocker_id) = self.tracked_blocker_id.clone() {
            self.blocker_assignments = vec![(blocker_id, attacker_id)];
            let assignments = self.blocker_assignments.clone();
            service
                .declare_blockers(
                    self.game_mut(),
                    demonictutor::DeclareBlockersCommand::new(Self::player_id("Bob"), assignments),
                )
                .expect("declare blockers should succeed");
        } else {
            self.blocker_assignments.clear();
        }

        self.reset_observations();
    }

    pub fn setup_blocked_damage_marking(&mut self) {
        self.setup_combat(
            "bdd-blocked-combat",
            "bdd-attacker-marking",
            LibraryCard::creature(CardDefinitionId::new("bdd-attacker-marking"), 0, 2, 4),
            Some("bdd-blocker-marking"),
            Some(LibraryCard::creature(
                CardDefinitionId::new("bdd-blocker-marking"),
                0,
                3,
                4,
            )),
        );
    }

    pub fn setup_unblocked_combat(&mut self) {
        self.setup_combat(
            "bdd-unblocked-combat",
            "bdd-attacker-unblocked",
            LibraryCard::creature(CardDefinitionId::new("bdd-attacker-unblocked"), 0, 3, 3),
            None,
            None,
        );
    }

    pub fn setup_lethal_damage_combat(&mut self) {
        self.setup_combat(
            "bdd-lethal-combat",
            "bdd-doomed-attacker",
            LibraryCard::creature(CardDefinitionId::new("bdd-doomed-attacker"), 0, 2, 2),
            Some("bdd-lethal-blocker"),
            Some(LibraryCard::creature(
                CardDefinitionId::new("bdd-lethal-blocker"),
                0,
                3,
                3,
            )),
        );
        self.tracked_card_id = self.tracked_attacker_id.clone();
    }

    pub fn setup_nonlethal_damage_combat(&mut self) {
        self.setup_combat(
            "bdd-nonlethal-combat",
            "bdd-sturdy-attacker",
            LibraryCard::creature(CardDefinitionId::new("bdd-sturdy-attacker"), 0, 2, 4),
            Some("bdd-sturdy-blocker"),
            Some(LibraryCard::creature(
                CardDefinitionId::new("bdd-sturdy-blocker"),
                0,
                2,
                4,
            )),
        );
        self.tracked_card_id = self.tracked_attacker_id.clone();
    }

    pub fn setup_end_step_with_surviving_damage(&mut self) {
        self.setup_combat(
            "bdd-cleanup-damage",
            "bdd-survivor",
            LibraryCard::creature(CardDefinitionId::new("bdd-survivor"), 0, 3, 3),
            Some("bdd-trader"),
            Some(LibraryCard::creature(
                CardDefinitionId::new("bdd-trader"),
                0,
                2,
                2,
            )),
        );
        self.resolve_combat_damage();
        let service = support::create_service();
        support::advance_n_satisfying_cleanup(&service, self.game_mut(), 2);
        while self.player_hand_size("Alice") > 7 {
            let card_id = self.player("Alice").hand().cards()[0].id().clone();
            service
                .discard_for_cleanup(
                    self.game_mut(),
                    DiscardForCleanupCommand::new(Self::player_id("Alice"), card_id),
                )
                .expect("BDD cleanup discard setup should succeed");
        }
        self.tracked_card_id = self.tracked_attacker_id.clone();
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::EndStep);
    }

    pub fn setup_end_step_with_eight_cards_in_hand(&mut self) {
        self.reset_game_with_libraries(
            "bdd-cleanup-discard",
            support::creature_library(20),
            support::creature_library(20),
        );

        let service = support::create_service();
        support::advance_n_raw(&service, self.game_mut(), 7);
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
            .adjust_life(
                self.game_mut(),
                AdjustLifeCommand::new(Self::player_id(alias), delta),
            )
            .expect("BDD setup life adjustment should succeed");

        assert!(outcome.game_ended.is_none());
        self.reset_observations();
    }

    pub fn advance_turn(&mut self) {
        self.pre_advance_hand_size = Some(self.player_hand_size("Alice"));

        match self.game_mut().advance_turn(AdvanceTurnCommand::new()) {
            Ok(AdvanceTurnOutcome::Progressed {
                turn_progressed,
                card_drawn,
            }) => {
                self.last_turn_progressed = Some(turn_progressed);
                self.last_game_ended = None;
                self.last_card_drawn = card_drawn;
                self.last_error = None;
            }
            Ok(AdvanceTurnOutcome::GameEnded(game_ended)) => {
                self.last_turn_progressed = None;
                self.last_game_ended = Some(game_ended);
                self.last_card_drawn = None;
                self.last_error = None;
            }
            Err(error) => {
                self.last_error = Some(error.to_string());
                self.last_turn_progressed = None;
                self.last_game_ended = None;
                self.last_card_drawn = None;
            }
        }

        self.post_advance_hand_size = Some(self.player_hand_size("Alice"));
    }

    pub fn cast_tracked_spell(&mut self, alias: &str) {
        let service = support::create_service();
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked spell card should exist");

        match service.cast_spell(
            self.game_mut(),
            CastSpellCommand::new(Self::player_id(alias), card_id),
        ) {
            Ok(outcome) => {
                self.last_spell_cast = Some(outcome.spell_cast);
                self.last_creature_died = outcome.creatures_died;
                self.last_error = None;
            }
            Err(error) => {
                self.last_spell_cast = None;
                self.last_creature_died.clear();
                self.last_error = Some(error.to_string());
            }
        }
    }

    pub fn adjust_life(&mut self, alias: &str, delta: i32) {
        let service = support::create_service();

        match service.adjust_life(
            self.game_mut(),
            AdjustLifeCommand::new(Self::player_id(alias), delta),
        ) {
            Ok(outcome) => {
                self.last_game_ended = outcome.game_ended;
                self.last_error = None;
            }
            Err(error) => {
                self.last_game_ended = None;
                self.last_error = Some(error.to_string());
            }
        }
    }

    pub fn discard_tracked_card_for_cleanup(&mut self, alias: &str) {
        let service = support::create_service();
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked discard card should exist");

        match service.discard_for_cleanup(
            self.game_mut(),
            DiscardForCleanupCommand::new(Self::player_id(alias), card_id),
        ) {
            Ok(event) => {
                self.last_card_discarded = Some(event);
                self.last_error = None;
            }
            Err(error) => {
                self.last_card_discarded = None;
                self.last_error = Some(error.to_string());
            }
        }
    }

    pub fn resolve_combat_damage(&mut self) {
        let service = support::create_service();
        let assignments = self.blocker_assignments.clone();
        match service.resolve_combat_damage(
            self.game_mut(),
            ResolveCombatDamageCommand::new(Self::player_id("Alice"), assignments),
        ) {
            Ok((damage_event, died_events)) => {
                self.last_combat_damage = Some(damage_event);
                self.last_creature_died = died_events;
                self.last_error = None;
            }
            Err(error) => {
                self.last_combat_damage = None;
                self.last_creature_died.clear();
                self.last_error = Some(error.to_string());
            }
        }
    }

    pub fn tracked_card(&self, alias: &str) -> &CardInstance {
        let card_id = self
            .tracked_card_id
            .as_ref()
            .expect("tracked card should exist");
        self.battlefield_card(alias, card_id)
    }

    pub fn tracked_attacker(&self) -> &CardInstance {
        let attacker_id = self
            .tracked_attacker_id
            .as_ref()
            .expect("tracked attacker should exist");
        self.battlefield_card("Alice", attacker_id)
    }

    pub fn tracked_blocker(&self) -> &CardInstance {
        let blocker_id = self
            .tracked_blocker_id
            .as_ref()
            .expect("tracked blocker should exist");
        self.battlefield_card("Bob", blocker_id)
    }
}
