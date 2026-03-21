use super::super::super::model::Player;
use crate::domain::play::{
    errors::{DomainError, GameError, PhaseError},
    ids::PlayerId,
    phase::Phase,
};

pub fn require_defending_player(
    active_player: &PlayerId,
    requested_player: &PlayerId,
) -> Result<(), DomainError> {
    if active_player == requested_player {
        return Err(DomainError::Phase(PhaseError::NotDefendingPlayer {
            current: active_player.clone(),
            requested: requested_player.clone(),
        }));
    }

    Ok(())
}

pub const fn require_attackers_step(phase: Phase) -> Result<(), DomainError> {
    if !matches!(phase, Phase::DeclareAttackers) {
        return Err(DomainError::Phase(PhaseError::InvalidForCombat));
    }

    Ok(())
}

pub const fn require_blockers_step(phase: Phase) -> Result<(), DomainError> {
    if !matches!(phase, Phase::DeclareBlockers) {
        return Err(DomainError::Phase(PhaseError::InvalidForCombat));
    }

    Ok(())
}

pub const fn require_combat_damage_step(phase: Phase) -> Result<(), DomainError> {
    if !matches!(phase, Phase::CombatDamage) {
        return Err(DomainError::Phase(PhaseError::InvalidForCombat));
    }

    Ok(())
}

pub fn find_defending_player_index(
    players: &[Player],
    active_player: &PlayerId,
) -> Result<usize, DomainError> {
    players
        .iter()
        .position(|player| player.id() != active_player)
        .ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(
                "defending player should exist".to_string(),
            ))
        })
}
