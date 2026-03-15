use super::player::Player;
use super::Phase;
use crate::domain::{
    cards::CardType,
    commands::{DeclareAttackersCommand, DeclareBlockersCommand},
    errors::{CardError, DomainError, GameError, PhaseError},
    events::{AttackersDeclared, BlockersDeclared},
    ids::{CardInstanceId, PlayerId},
};

pub fn declare_attackers(
    players: &mut [Player],
    active_player: &PlayerId,
    phase: &Phase,
    cmd: DeclareAttackersCommand,
) -> Result<AttackersDeclared, DomainError> {
    if *active_player != cmd.player_id {
        return Err(DomainError::Game(GameError::NotYourTurn {
            current: active_player.clone(),
            requested: cmd.player_id.clone(),
        }));
    }

    if !matches!(phase, Phase::Main) {
        return Err(DomainError::Phase(PhaseError::InvalidForCombat));
    }

    let player_idx = players
        .iter()
        .position(|p| p.id() == &cmd.player_id)
        .ok_or_else(|| DomainError::Game(GameError::PlayerNotFound(cmd.player_id.clone())))?;

    let player = &mut players[player_idx];
    let battlefield = player.battlefield_mut();

    let mut valid_attackers: Vec<CardInstanceId> = Vec::new();

    for attacker_id in &cmd.attacker_ids {
        let card = battlefield
            .cards_mut()
            .iter_mut()
            .find(|c| c.id() == attacker_id)
            .ok_or_else(|| {
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

        if card.has_summoning_sickness() {
            return Err(DomainError::Card(CardError::CreatureHasSummoningSickness {
                player: cmd.player_id.clone(),
                card: attacker_id.clone(),
            }));
        }

        card.set_attacking(true);
        card.tap();
        valid_attackers.push(attacker_id.clone());
    }

    Ok(AttackersDeclared::new(
        super::Game::id_from_player_id(&cmd.player_id),
        cmd.player_id,
        valid_attackers,
    ))
}

pub fn declare_blockers(
    players: &mut [Player],
    active_player: &PlayerId,
    phase: &Phase,
    cmd: DeclareBlockersCommand,
) -> Result<BlockersDeclared, DomainError> {
    if *active_player == cmd.player_id {
        return Err(DomainError::Phase(PhaseError::NotDefendingPlayer {
            current: active_player.clone(),
            requested: cmd.player_id.clone(),
        }));
    }

    if !matches!(phase, Phase::Main) {
        return Err(DomainError::Phase(PhaseError::InvalidForCombat));
    }

    let defending_player_idx = players
        .iter()
        .position(|p| p.id() != active_player)
        .ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(
                "defending player should exist".to_string(),
            ))
        })?;

    let defender = &mut players[defending_player_idx];
    let battlefield = defender.battlefield_mut();

    let mut valid_blockers: Vec<(CardInstanceId, CardInstanceId)> = Vec::new();

    for (blocker_id, attacker_id) in &cmd.blocker_assignments {
        let card = battlefield
            .cards_mut()
            .iter_mut()
            .find(|c| c.id() == blocker_id)
            .ok_or_else(|| {
                DomainError::Card(CardError::NotOnBattlefield {
                    player: cmd.player_id.clone(),
                    card: blocker_id.clone(),
                })
            })?;

        if !matches!(card.card_type(), CardType::Creature) {
            return Err(DomainError::Card(CardError::NotACreature(
                blocker_id.clone(),
            )));
        }

        if card.is_tapped() {
            return Err(DomainError::Card(CardError::AlreadyTapped {
                player: cmd.player_id.clone(),
                card: blocker_id.clone(),
            }));
        }

        card.set_blocking(true);
        valid_blockers.push((blocker_id.clone(), attacker_id.clone()));
    }

    Ok(BlockersDeclared::new(
        super::Game::id_from_player_id(&cmd.player_id),
        cmd.player_id,
        valid_blockers,
    ))
}
