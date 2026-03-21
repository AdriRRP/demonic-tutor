use super::super::super::{
    super::{Player, TerminalState},
    state_based_actions::{self, StateBasedActionsResult},
};
use crate::domain::play::{
    cards::{SpellResolutionProfile, SupportedSpellRules},
    errors::{DomainError, GameError},
    events::{CreatureDied, GameEnded, LifeChanged},
    game::SpellTarget,
    ids::{CardInstanceId, GameId},
};

type SpellResolutionSideEffects = (Option<LifeChanged>, Vec<CreatureDied>, Option<GameEnded>);

fn apply_damage_to_creature(players: &mut [Player], target_id: &CardInstanceId, damage: u32) {
    for player in players.iter_mut() {
        if let Some(card) = player.battlefield_mut().card_mut(target_id) {
            card.add_damage(damage);
            return;
        }
    }
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
    supported_spell_rules: SupportedSpellRules,
    target: Option<&SpellTarget>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    match supported_spell_rules.resolution() {
        SpellResolutionProfile::None => {
            review_state_based_actions(game_id, players, terminal_state)
        }
        SpellResolutionProfile::DealDamage { damage } => {
            let Some(target) = target else {
                return Err(DomainError::Game(GameError::InternalInvariantViolation(
                    "targeted spell resolved without target".to_string(),
                )));
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
    }
}
