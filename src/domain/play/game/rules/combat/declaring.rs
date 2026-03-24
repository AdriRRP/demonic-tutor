use super::super::super::{helpers, model::Player};
use crate::domain::play::{
    cards::CardType,
    commands::DeclareAttackersCommand,
    errors::{CardError, DomainError},
    events::AttackersDeclared,
    ids::{CardInstanceId, GameId},
};

pub fn declare_attackers(
    game_id: &GameId,
    players: &mut [Player],
    cmd: DeclareAttackersCommand,
) -> Result<AttackersDeclared, DomainError> {
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

        card.set_attacking(true);
        if !card.has_vigilance() {
            card.tap();
        }
        valid_attackers.push(attacker_id.clone());
    }

    Ok(AttackersDeclared::new(
        game_id.clone(),
        cmd.player_id,
        valid_attackers,
    ))
}
