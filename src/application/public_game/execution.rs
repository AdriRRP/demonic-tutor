//! Executes public gameplay commands and assembles deterministic response envelopes.

use crate::{
    application::{
        game_service::{
            combat::domain_events_for_resolve_combat_damage,
            resource_actions::domain_events_for_adjust_player_life_effect,
            stack::{
                domain_events_for_activate_ability, domain_events_for_cast_spell,
                domain_events_for_pass_priority, domain_events_for_resolve_optional_effect,
                domain_events_for_resolve_pending_hand_choice,
                domain_events_for_resolve_pending_scry, domain_events_for_resolve_pending_surveil,
            },
            turn_flow::{domain_events_for_advance_turn, domain_events_for_draw_cards_effect},
            GameService,
        },
        EventBus, EventStore,
    },
    domain::play::{errors::DomainError, events::DomainEvent, game::Game},
};

use super::{
    PublicCommandApplication, PublicCommandRejection, PublicCommandStatus, PublicGameCommand,
};

impl<E, B> GameService<E, B>
where
    E: EventStore,
    B: EventBus,
{
    /// Executes a public gameplay command and returns a UI-friendly deterministic envelope.
    pub fn execute_public_command(
        &self,
        game: &mut Game,
        command: PublicGameCommand,
    ) -> PublicCommandApplication {
        let result: Result<Vec<DomainEvent>, DomainError> = match command {
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
                .map(|event| vec![event.into()]),
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
                .map(|event| vec![event.into()]),
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
        let emitted_events = result.unwrap_or_default();

        PublicCommandApplication {
            status,
            emitted_events,
        }
    }
}
