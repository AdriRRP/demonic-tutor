//! Supports game rules combat.

mod blocking_legality;
mod capabilities;
mod damage;
mod declaring;
mod progression;

use super::super::{
    invariants,
    model::{AggregateCardLocationIndex, Player},
    TerminalState,
};
use crate::domain::play::{
    commands::{DeclareAttackersCommand, DeclareBlockersCommand, ResolveCombatDamageCommand},
    errors::DomainError,
    events::{AttackersDeclared, BlockersDeclared},
    ids::GameId,
    phase::Phase,
};

pub use damage::ResolveCombatDamageOutcome;

/// Declares attackers for the active player in combat.
///
/// # Errors
/// Returns an error if the action is invalid.
pub fn declare_attackers(
    game_id: &GameId,
    players: &mut [Player],
    active_player_index: usize,
    phase: &Phase,
    cmd: DeclareAttackersCommand,
) -> Result<AttackersDeclared, DomainError> {
    invariants::require_active_player_index(players, active_player_index, &cmd.player_id)?;
    progression::require_attackers_step(*phase)?;
    declaring::declare_attackers(game_id, players, cmd)
}

/// Declares blockers for the defending player in combat.
///
/// # Errors
/// Returns an error if the action is invalid.
pub fn declare_blockers(
    game_id: &GameId,
    players: &mut [Player],
    active_player_index: usize,
    phase: &Phase,
    cmd: DeclareBlockersCommand,
) -> Result<BlockersDeclared, DomainError> {
    let active_player = players
        .get(active_player_index)
        .ok_or_else(|| {
            DomainError::Game(
                crate::domain::play::errors::GameError::InternalInvariantViolation(
                    "active player index should point to an existing player".to_string(),
                ),
            )
        })?
        .id();
    progression::require_defending_player(active_player, &cmd.player_id)?;
    progression::require_blockers_step(*phase)?;
    blocking_legality::declare_blockers(game_id, players, active_player_index, cmd)
}

/// Resolves combat damage between attacking and blocking creatures.
///
/// # Errors
/// Returns an error if the action is invalid.
pub fn resolve_combat_damage(
    game_id: &GameId,
    players: &mut [Player],
    card_locations: &AggregateCardLocationIndex,
    active_player_index: usize,
    phase: &Phase,
    terminal_state: &mut TerminalState,
    cmd: ResolveCombatDamageCommand,
) -> Result<ResolveCombatDamageOutcome, DomainError> {
    invariants::require_active_player_index(players, active_player_index, &cmd.player_id)?;
    progression::require_combat_damage_step(*phase)?;

    let player_idx = active_player_index;
    let defender_idx = progression::find_defending_player_index(players, active_player_index)?;

    damage::resolve_combat_damage(
        game_id,
        players,
        card_locations,
        terminal_state,
        cmd,
        player_idx,
        defender_idx,
    )
}
