mod blocking_legality;
mod damage;
mod declaring;
mod progression;

use super::super::{helpers, invariants, model::Player, TerminalState};
use crate::domain::play::{
    commands::{DeclareAttackersCommand, DeclareBlockersCommand, ResolveCombatDamageCommand},
    errors::DomainError,
    events::{AttackersDeclared, BlockersDeclared},
    ids::{GameId, PlayerId},
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
    active_player: &PlayerId,
    phase: &Phase,
    cmd: DeclareAttackersCommand,
) -> Result<AttackersDeclared, DomainError> {
    invariants::require_active_player(active_player, &cmd.player_id)?;
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
    active_player: &PlayerId,
    phase: &Phase,
    cmd: DeclareBlockersCommand,
) -> Result<BlockersDeclared, DomainError> {
    progression::require_defending_player(active_player, &cmd.player_id)?;
    progression::require_blockers_step(*phase)?;
    blocking_legality::declare_blockers(game_id, players, active_player, cmd)
}

/// Resolves combat damage between attacking and blocking creatures.
///
/// # Errors
/// Returns an error if the action is invalid.
pub fn resolve_combat_damage(
    game_id: &GameId,
    players: &mut [Player],
    active_player: &PlayerId,
    phase: &Phase,
    terminal_state: &mut TerminalState,
    cmd: ResolveCombatDamageCommand,
) -> Result<ResolveCombatDamageOutcome, DomainError> {
    invariants::require_active_player(active_player, &cmd.player_id)?;
    progression::require_combat_damage_step(*phase)?;

    let player_idx = helpers::find_player_index(players, &cmd.player_id)?;
    let defender_idx = progression::find_defending_player_index(players, &cmd.player_id)?;

    damage::resolve_combat_damage(
        game_id,
        players,
        terminal_state,
        cmd,
        player_idx,
        defender_idx,
    )
}
