//! Executes public gameplay commands and assembles deterministic response envelopes.

use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

use crate::{
    application::{
        game_service::{
            combat::{
                domain_events_for_declare_attackers, domain_events_for_resolve_combat_damage,
            },
            resource_actions::domain_events_for_adjust_player_life_effect,
            stack::{
                domain_events_for_activate_ability, domain_events_for_cast_spell,
                domain_events_for_pass_priority, domain_events_for_resolve_optional_effect,
                domain_events_for_resolve_pending_hand_choice,
                domain_events_for_resolve_pending_scry, domain_events_for_resolve_pending_surveil,
            },
            turn_flow::{
                domain_events_for_advance_turn, domain_events_for_discard_for_cleanup,
                domain_events_for_draw_cards_effect,
            },
            GameService,
        },
        EventBus, EventStore,
    },
    domain::play::{
        commands::{DealOpeningHandsCommand, PlayerDeck, PlayerLibrary, StartGameCommand},
        errors::DomainError,
        events::DomainEvent,
        game::Game,
        ids::{GameId, PlayerId},
    },
};

use super::{
    public_event_log,
    surface::{public_events, public_surface_state},
    PublicCommandApplication, PublicCommandRejection, PublicCommandStatus, PublicEventLogEntry,
    PublicGameCommand, PublicGameSessionStart, PublicRematchCommand, PublicSeededGameSetup,
    PublicSeededPlayerSetup,
};

impl<E, B> GameService<E, B>
where
    E: EventStore,
    B: EventBus,
{
    /// Starts one public game from a deterministic seeded setup.
    ///
    /// # Errors
    ///
    /// Returns an error if the lifecycle commands are invalid.
    pub fn start_seeded_public_game(
        &self,
        setup: PublicSeededGameSetup,
        viewer_id: &PlayerId,
    ) -> Result<(Game, PublicGameSessionStart), DomainError> {
        let PublicSeededGameSetup {
            game_id,
            players,
            shuffle_seed,
        } = setup;
        let (player_decks, player_libraries) = seeded_start_inputs(players, shuffle_seed);
        let (game, game_started, opening_hands) = self.start_game_with_opening_hands(
            StartGameCommand::new(game_id, player_decks),
            &DealOpeningHandsCommand::new(player_libraries),
        )?;
        let emitted_events = std::iter::once(game_started.into())
            .chain(opening_hands.into_iter().map(Into::into))
            .collect();
        let session = public_game_session_start(&game, emitted_events, viewer_id);

        Ok((game, session))
    }

    /// Starts a seeded rematch using the same setup shape with a new game id.
    ///
    /// # Errors
    ///
    /// Returns an error if the lifecycle commands are invalid.
    pub fn rematch_seeded_public_game(
        &self,
        cmd: PublicRematchCommand,
        viewer_id: &PlayerId,
    ) -> Result<(Game, PublicGameSessionStart), DomainError> {
        self.start_seeded_public_game(cmd.original_setup.with_game_id(cmd.game_id), viewer_id)
    }

    /// Returns the persisted public event log for one game in deterministic sequence order.
    ///
    /// # Errors
    ///
    /// Returns an error if the backing event store cannot load the persisted stream.
    pub fn public_event_log(
        &self,
        game_id: &GameId,
    ) -> Result<Vec<PublicEventLogEntry>, DomainError> {
        let aggregate_id = game_id.to_string();
        let events = self.load_persisted_events(&aggregate_id)?;

        Ok(public_event_log(events))
    }

    /// Executes a public gameplay command and returns a UI-friendly deterministic envelope.
    pub fn execute_public_command(
        &self,
        game: &mut Game,
        command: PublicGameCommand,
    ) -> PublicCommandApplication {
        let result: Result<Vec<DomainEvent>, DomainError> = match command {
            PublicGameCommand::Concede(cmd) => {
                self.concede(game, cmd).map(|event| vec![event.into()])
            }
            PublicGameCommand::PlayLand(cmd) => {
                self.play_land(game, cmd).map(|event| vec![event.into()])
            }
            PublicGameCommand::TapLand(cmd) => self
                .tap_land(game, cmd)
                .map(|(land_tapped, mana_added)| vec![land_tapped.into(), mana_added.into()]),
            PublicGameCommand::CastSpell(cmd) => self
                .cast_spell(game, cmd)
                .map(|outcome| domain_events_for_cast_spell(&outcome)),
            PublicGameCommand::ActivateAbility(cmd) => self
                .activate_ability(game, cmd)
                .map(|outcome| domain_events_for_activate_ability(&outcome)),
            PublicGameCommand::PassPriority(cmd) => self
                .pass_priority(game, cmd)
                .map(|outcome| domain_events_for_pass_priority(&outcome)),
            PublicGameCommand::DeclareAttackers(cmd) => self
                .declare_attackers(game, cmd)
                .map(|outcome| domain_events_for_declare_attackers(&outcome)),
            PublicGameCommand::DeclareBlockers(cmd) => self
                .declare_blockers(game, cmd)
                .map(|event| vec![event.into()]),
            PublicGameCommand::ResolveCombatDamage(cmd) => self
                .resolve_combat_damage(game, cmd)
                .map(|outcome| domain_events_for_resolve_combat_damage(&outcome)),
            PublicGameCommand::AdvanceTurn(cmd) => self
                .advance_turn(game, cmd)
                .map(|outcome| domain_events_for_advance_turn(&outcome)),
            PublicGameCommand::DrawCardsEffect(cmd) => self
                .draw_cards_effect(game, &cmd)
                .map(|outcome| domain_events_for_draw_cards_effect(&outcome)),
            PublicGameCommand::DiscardForCleanup(cmd) => self
                .discard_for_cleanup(game, cmd)
                .map(|event| domain_events_for_discard_for_cleanup(&event)),
            PublicGameCommand::AdjustPlayerLifeEffect(cmd) => self
                .adjust_player_life_effect(game, cmd)
                .map(|outcome| domain_events_for_adjust_player_life_effect(&outcome)),
            PublicGameCommand::ExileCard(cmd) => {
                self.exile_card(game, &cmd).map(|event| vec![event.into()])
            }
            PublicGameCommand::ResolveOptionalEffect(cmd) => self
                .resolve_optional_effect(game, cmd)
                .map(|outcome| domain_events_for_resolve_optional_effect(&outcome)),
            PublicGameCommand::ResolvePendingHandChoice(cmd) => self
                .resolve_pending_hand_choice(game, cmd)
                .map(|outcome| domain_events_for_resolve_pending_hand_choice(&outcome)),
            PublicGameCommand::ResolvePendingScry(cmd) => self
                .resolve_pending_scry(game, cmd)
                .map(|outcome| domain_events_for_resolve_pending_scry(&outcome)),
            PublicGameCommand::ResolvePendingSurveil(cmd) => self
                .resolve_pending_surveil(game, cmd)
                .map(|outcome| domain_events_for_resolve_pending_surveil(&outcome)),
        };

        let status = match &result {
            Ok(_) => PublicCommandStatus::Applied,
            Err(err) => PublicCommandStatus::Rejected(PublicCommandRejection {
                message: err.to_string(),
            }),
        };
        let emitted_events = public_events(result.unwrap_or_default());

        PublicCommandApplication {
            status,
            emitted_events,
        }
    }
}

fn seeded_start_inputs(
    players: Vec<PublicSeededPlayerSetup>,
    shuffle_seed: u64,
) -> (Vec<PlayerDeck>, Vec<PlayerLibrary>) {
    let mut rng = StdRng::seed_from_u64(shuffle_seed);

    players
        .into_iter()
        .map(|player| {
            let PublicSeededPlayerSetup {
                player_id,
                deck_id,
                mut cards,
            } = player;
            cards.shuffle(&mut rng);
            (
                PlayerDeck::new(player_id.clone(), deck_id),
                PlayerLibrary::new(player_id, cards),
            )
        })
        .unzip()
}

fn public_game_session_start(
    game: &Game,
    emitted_events: Vec<DomainEvent>,
    viewer_id: &PlayerId,
) -> PublicGameSessionStart {
    let (legal_actions, choice_requests) = public_surface_state(game, viewer_id).into_parts();

    PublicGameSessionStart {
        emitted_events: public_events(emitted_events),
        game: super::game_view(game),
        legal_actions,
        choice_requests,
    }
}
