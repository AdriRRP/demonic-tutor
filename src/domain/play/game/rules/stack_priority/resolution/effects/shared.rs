//! Supports shared spell-resolution context and legality helpers.

use crate::domain::play::game::{
    model::StackZone,
    rules::{
        stack_priority::spell_effects::{
            evaluate_target_legality, SpellTargetLegality, TargetLegalityContext,
        },
        state_based_actions::{self, StateBasedActionsResult},
    },
    AggregateCardLocationIndex, Player, TerminalState,
};
use crate::domain::play::{
    cards::SupportedSpellRules,
    errors::{DomainError, GameError},
    events::{CardDiscarded, CardMovedZone, CreatureDied, GameEnded, LifeChanged},
    game::{model::StackSpellChoice, SpellTarget},
    ids::{CardInstanceId, GameId},
};

pub(super) type SpellResolutionSideEffects = (
    Option<CardDiscarded>,
    Vec<CardMovedZone>,
    Option<LifeChanged>,
    Vec<CreatureDied>,
    Vec<CardInstanceId>,
    Option<GameEnded>,
);

pub(in crate::domain::play::game::rules::stack_priority::resolution) struct ResolutionContext<'a> {
    pub(super) game_id: &'a GameId,
    pub(super) players: &'a mut [Player],
    pub(super) card_locations: &'a AggregateCardLocationIndex,
    pub(super) terminal_state: &'a mut TerminalState,
    pub(super) stack: &'a mut StackZone,
    pub(super) controller_index: usize,
    pub(super) supported_spell_rules: SupportedSpellRules,
    pub(super) target: Option<&'a SpellTarget>,
    pub(super) choice: Option<StackSpellChoice>,
}

impl<'a> ResolutionContext<'a> {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::domain::play::game::rules::stack_priority::resolution) const fn new(
        game_id: &'a GameId,
        players: &'a mut [Player],
        card_locations: &'a AggregateCardLocationIndex,
        terminal_state: &'a mut TerminalState,
        stack: &'a mut StackZone,
        controller_index: usize,
        supported_spell_rules: SupportedSpellRules,
        target: Option<&'a SpellTarget>,
        choice: Option<StackSpellChoice>,
    ) -> Self {
        Self {
            game_id,
            players,
            card_locations,
            terminal_state,
            stack,
            controller_index,
            supported_spell_rules,
            target,
            choice,
        }
    }
}

pub(super) struct EffectOutcomeSeed {
    pub(super) card_discarded: Option<CardDiscarded>,
    pub(super) zone_changes: Vec<CardMovedZone>,
    pub(super) life_changed: Option<LifeChanged>,
    pub(super) creatures_died: Vec<CreatureDied>,
    pub(super) moved_cards: Vec<CardInstanceId>,
}

pub(super) fn review_state_based_actions(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let StateBasedActionsResult {
        creatures_died,
        zone_changes,
        game_ended,
    } = state_based_actions::check_state_based_actions(game_id, players, terminal_state)?;
    Ok((
        None,
        zone_changes,
        None,
        creatures_died,
        Vec::new(),
        game_ended,
    ))
}

pub(super) fn resolve_target_legality_for_effect(
    players: &[Player],
    card_locations: &AggregateCardLocationIndex,
    stack: &StackZone,
    controller_index: usize,
    supported_spell_rules: SupportedSpellRules,
    target: Option<&SpellTarget>,
    missing_profile_message: &str,
) -> Result<Option<SpellTarget>, DomainError> {
    let legality = evaluate_target_legality(
        TargetLegalityContext::Resolution {
            players,
            card_locations,
            stack,
            actor_index: controller_index,
        },
        supported_spell_rules.targeting(),
        target,
    );

    match (legality, target) {
        (SpellTargetLegality::Legal, Some(target)) => Ok(Some((*target).clone())),
        (SpellTargetLegality::Legal, None) => {
            Err(DomainError::Game(GameError::InternalInvariantViolation(
                "legal targeted spell resolution requires an attached target".to_string(),
            )))
        }
        (SpellTargetLegality::MissingRequiredTarget, _) => {
            Err(DomainError::Game(GameError::InternalInvariantViolation(
                "targeted spell resolved without target".to_string(),
            )))
        }
        (SpellTargetLegality::NoTargetRequired, _) => Err(DomainError::Game(
            GameError::InternalInvariantViolation(missing_profile_message.to_string()),
        )),
        (
            SpellTargetLegality::IllegalTargetKind
            | SpellTargetLegality::IllegalTargetRule
            | SpellTargetLegality::MissingPlayer(_)
            | SpellTargetLegality::MissingCreature(_)
            | SpellTargetLegality::MissingPermanent(_)
            | SpellTargetLegality::MissingGraveyardCard(_)
            | SpellTargetLegality::MissingStackSpell(_),
            _,
        ) => Ok(None),
    }
}

pub(super) fn review_state_based_actions_after_effect(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
    seed: EffectOutcomeSeed,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let EffectOutcomeSeed {
        card_discarded,
        mut zone_changes,
        life_changed,
        mut creatures_died,
        moved_cards,
    } = seed;
    let StateBasedActionsResult {
        creatures_died: sba_creatures_died,
        zone_changes: sba_zone_changes,
        game_ended,
    } = state_based_actions::check_state_based_actions(game_id, players, terminal_state)?;
    creatures_died.extend(sba_creatures_died);
    zone_changes.extend(sba_zone_changes);
    Ok((
        card_discarded,
        zone_changes,
        life_changed,
        creatures_died,
        moved_cards,
        game_ended,
    ))
}
