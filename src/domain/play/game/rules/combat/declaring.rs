//! Supports rules combat declaring.

use {
    super::super::super::{helpers, model::Player},
    crate::domain::play::{
        cards::{CardType, TriggeredAbilityEvent},
        commands::DeclareAttackersCommand,
        errors::{CardError, DomainError},
        events::{AttackersDeclared, TriggeredAbilityPutOnStack},
        game::model::StackZone,
        ids::{CardInstanceId, GameId},
    },
};

#[derive(Debug, Clone)]
pub struct DeclareAttackersOutcome {
    pub attackers_declared: AttackersDeclared,
    pub triggered_abilities_put_on_stack: Vec<TriggeredAbilityPutOnStack>,
}

#[allow(clippy::redundant_pub_crate)]
pub(crate) fn can_attack_with_candidate(
    players: &[Player],
    active_player_index: usize,
    phase: crate::domain::play::phase::Phase,
    player_id: &crate::domain::play::ids::PlayerId,
    attacker_id: &CardInstanceId,
) -> bool {
    if super::progression::require_attackers_step(phase).is_err() {
        return false;
    }
    let Ok(player_idx) = helpers::find_player_index(players, player_id) else {
        return false;
    };
    if player_idx != active_player_index {
        return false;
    }

    let Some(card) = players[player_idx].battlefield_card(attacker_id) else {
        return false;
    };

    matches!(card.card_type(), CardType::Creature)
        && !card.is_tapped()
        && (!card.has_summoning_sickness() || card.has_haste())
        && !card.cannot_attack()
}

pub fn declare_attackers(
    game_id: &GameId,
    players: &mut [Player],
    stack: &mut StackZone,
    cmd: DeclareAttackersCommand,
) -> Result<DeclareAttackersOutcome, DomainError> {
    let player_idx = helpers::find_player_index(players, &cmd.player_id)?;
    let player = &mut players[player_idx];
    let mut valid_attackers: Vec<CardInstanceId> = Vec::new();

    for attacker_id in &cmd.attacker_ids {
        let card = player.battlefield_card_mut(attacker_id).ok_or_else(|| {
            DomainError::Card(CardError::NotOnBattlefield {
                player: cmd.player_id.clone(),
                card: attacker_id.clone(),
            })
        })?;

        if !matches!(card.card_type(), CardType::Creature) {
            return Err(DomainError::Card(CardError::NotACreature(
                attacker_id.clone(),
            )));
        }

        if card.is_tapped() {
            return Err(DomainError::Card(CardError::AlreadyTapped {
                player: cmd.player_id.clone(),
                card: attacker_id.clone(),
            }));
        }

        if card.has_summoning_sickness() && !card.has_haste() {
            return Err(DomainError::Card(CardError::CreatureHasSummoningSickness {
                player: cmd.player_id.clone(),
                card: attacker_id.clone(),
            }));
        }

        if card.cannot_attack() {
            return Err(DomainError::Card(CardError::CannotAttack {
                player: cmd.player_id.clone(),
                card: attacker_id.clone(),
            }));
        }

        card.set_attacking(true);
        if !card.has_vigilance() {
            card.tap();
        }
        valid_attackers.push(attacker_id.clone());
    }

    let attackers_declared =
        AttackersDeclared::new(game_id.clone(), cmd.player_id, valid_attackers);
    let trigger_handles = attackers_declared
        .attackers
        .iter()
        .filter_map(|card_id| players[player_idx].resolve_public_card_handle(card_id))
        .collect::<Vec<_>>();
    let mut triggered_abilities_put_on_stack = Vec::new();
    for handle in trigger_handles {
        triggered_abilities_put_on_stack.extend(
            crate::domain::play::game::rules::stack_priority::triggers::enqueue_trigger_for_card_handle(
                game_id,
                players,
                player_idx,
                handle,
                TriggeredAbilityEvent::Attacks,
                stack,
            )?,
        );
    }

    Ok(DeclareAttackersOutcome {
        attackers_declared,
        triggered_abilities_put_on_stack,
    })
}
