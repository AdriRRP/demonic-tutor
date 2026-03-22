use super::super::super::{
    super::{helpers, Player, TerminalState},
    state_based_actions::{self, StateBasedActionsResult},
};
use super::super::spell_effects::{
    evaluate_target_legality, SpellTargetLegality, TargetLegalityContext,
};
use crate::domain::play::{
    cards::{SpellResolutionProfile, SupportedSpellRules},
    errors::{DomainError, GameError},
    events::{CreatureDied, GameEnded, LifeChanged},
    game::SpellTarget,
    ids::{CardInstanceId, GameId, PlayerId},
};

type SpellResolutionSideEffects = (Option<LifeChanged>, Vec<CreatureDied>, Option<GameEnded>);

fn apply_damage_to_creature(players: &mut [Player], target_id: &CardInstanceId, damage: u32) {
    if let Some(card) = helpers::battlefield_card_mut(players, target_id) {
        card.add_damage(damage);
    }
}

fn destroy_creature(
    game_id: &GameId,
    players: &mut [Player],
    target_id: &CardInstanceId,
) -> Option<CreatureDied> {
    let target = helpers::battlefield_card_location(players, target_id)?;
    let owner_index = target.owner_index();
    let owner_id = target.owner_id().clone();
    players[owner_index].move_battlefield_card_to_graveyard(target_id)?;
    Some(CreatureDied::new(
        game_id.clone(),
        owner_id,
        target_id.clone(),
    ))
}

fn review_state_based_actions(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let StateBasedActionsResult {
        creatures_died,
        game_ended,
    } = state_based_actions::check_state_based_actions(game_id, players, terminal_state)?;
    Ok((None, creatures_died, game_ended))
}

pub(super) fn apply_supported_spell_rules(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
    controller_id: &PlayerId,
    supported_spell_rules: SupportedSpellRules,
    target: Option<&SpellTarget>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    match supported_spell_rules.resolution() {
        SpellResolutionProfile::None => {
            review_state_based_actions(game_id, players, terminal_state)
        }
        SpellResolutionProfile::DealDamage { damage } => {
            let legality = evaluate_target_legality(
                TargetLegalityContext::Resolution {
                    players,
                    controller_id,
                },
                supported_spell_rules.targeting(),
                target,
            );
            let target = match (legality, target) {
                (SpellTargetLegality::Legal, Some(target)) => target,
                (SpellTargetLegality::Legal, None) => {
                    return Err(DomainError::Game(GameError::InternalInvariantViolation(
                        "legal targeted spell resolution requires an attached target".to_string(),
                    )));
                }
                (SpellTargetLegality::MissingRequiredTarget, _) => {
                    return Err(DomainError::Game(GameError::InternalInvariantViolation(
                        "targeted spell resolved without target".to_string(),
                    )));
                }
                (SpellTargetLegality::NoTargetRequired, _) => {
                    return Err(DomainError::Game(GameError::InternalInvariantViolation(
                        "damage spell resolved without a targeting profile".to_string(),
                    )));
                }
                (
                    SpellTargetLegality::IllegalTargetKind
                    | SpellTargetLegality::IllegalTargetRule
                    | SpellTargetLegality::MissingPlayer(_)
                    | SpellTargetLegality::MissingCreature(_),
                    _,
                ) => {
                    return review_state_based_actions(game_id, players, terminal_state);
                }
            };

            let life_changed = match target {
                SpellTarget::Player(player_id) => {
                    Some(super::super::super::game_effects::adjust_player_life(
                        game_id,
                        players,
                        player_id,
                        -(damage).cast_signed(),
                    )?)
                }
                SpellTarget::Creature(card_id) => {
                    apply_damage_to_creature(players, card_id, damage);
                    None
                }
            };

            let StateBasedActionsResult {
                creatures_died,
                game_ended,
            } = state_based_actions::check_state_based_actions(game_id, players, terminal_state)?;
            Ok((life_changed, creatures_died, game_ended))
        }
        SpellResolutionProfile::DestroyTargetCreature => {
            let legality = evaluate_target_legality(
                TargetLegalityContext::Resolution {
                    players,
                    controller_id,
                },
                supported_spell_rules.targeting(),
                target,
            );
            let target = match (legality, target) {
                (SpellTargetLegality::Legal, Some(target)) => target,
                (SpellTargetLegality::Legal, None) => {
                    return Err(DomainError::Game(GameError::InternalInvariantViolation(
                        "legal targeted spell resolution requires an attached target".to_string(),
                    )));
                }
                (SpellTargetLegality::MissingRequiredTarget, _) => {
                    return Err(DomainError::Game(GameError::InternalInvariantViolation(
                        "targeted spell resolved without target".to_string(),
                    )));
                }
                (SpellTargetLegality::NoTargetRequired, _) => {
                    return Err(DomainError::Game(GameError::InternalInvariantViolation(
                        "destroy spell resolved without a targeting profile".to_string(),
                    )));
                }
                (
                    SpellTargetLegality::IllegalTargetKind
                    | SpellTargetLegality::IllegalTargetRule
                    | SpellTargetLegality::MissingPlayer(_)
                    | SpellTargetLegality::MissingCreature(_),
                    _,
                ) => {
                    return review_state_based_actions(game_id, players, terminal_state);
                }
            };

            let mut creatures_died = Vec::new();
            if let SpellTarget::Creature(card_id) = target {
                if let Some(creature_died) = destroy_creature(game_id, players, card_id) {
                    creatures_died.push(creature_died);
                }
            }

            let StateBasedActionsResult {
                creatures_died: sba_creatures_died,
                game_ended,
            } = state_based_actions::check_state_based_actions(game_id, players, terminal_state)?;
            creatures_died.extend(sba_creatures_died);
            Ok((None, creatures_died, game_ended))
        }
    }
}
