mod application;
mod assignments;
mod participants;

use self::{
    application::apply_damage_and_clear_combat_state,
    assignments::blocker_by_attacker,
    participants::{collect_attackers, collect_blockers},
};
use super::super::{
    super::{model::Player, TerminalState},
    game_effects, state_based_actions,
    state_based_actions::StateBasedActionsResult,
};
use crate::domain::play::{
    commands::ResolveCombatDamageCommand,
    errors::{DomainError, GameError},
    events::{
        CombatDamageResolved, CreatureDied, DamageEvent, DamageTarget, GameEnded, LifeChanged,
    },
    ids::{CardInstanceId, GameId},
};

fn add_damage(
    damage_received: &mut Vec<(CardInstanceId, u32)>,
    target: &CardInstanceId,
    damage: u32,
) {
    if let Some((_, accumulated)) = damage_received
        .iter_mut()
        .find(|(card_id, _)| card_id == target)
    {
        *accumulated += damage;
    } else {
        damage_received.push((target.clone(), damage));
    }
}

#[derive(Debug, Clone)]
pub struct ResolveCombatDamageOutcome {
    pub combat_damage_resolved: CombatDamageResolved,
    pub life_changed: Option<LifeChanged>,
    pub creatures_died: Vec<CreatureDied>,
    pub game_ended: Option<GameEnded>,
}

impl ResolveCombatDamageOutcome {
    #[must_use]
    pub const fn new(
        combat_damage_resolved: CombatDamageResolved,
        life_changed: Option<LifeChanged>,
        creatures_died: Vec<CreatureDied>,
        game_ended: Option<GameEnded>,
    ) -> Self {
        Self {
            combat_damage_resolved,
            life_changed,
            creatures_died,
            game_ended,
        }
    }
}

pub fn resolve_combat_damage(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
    cmd: ResolveCombatDamageCommand,
    attacker_player_idx: usize,
    defender_idx: usize,
) -> Result<ResolveCombatDamageOutcome, DomainError> {
    let defender_player_id = players[defender_idx].id().clone();
    let attackers = collect_attackers(&players[attacker_player_idx])?;
    if attackers.is_empty() {
        return Err(DomainError::Game(GameError::NoAttackersDeclared));
    }
    let blockers = collect_blockers(&players[defender_idx])?;
    let blocker_by_attacker = blocker_by_attacker(&players[defender_idx]);

    let mut damage_events: Vec<DamageEvent> = Vec::new();
    let mut damage_received: Vec<(CardInstanceId, u32)> = Vec::new();
    let mut player_damage = 0;

    for (attacker_id, power) in &attackers {
        if let Some((_, blocker_id)) = blocker_by_attacker
            .iter()
            .find(|(blocked_attacker_id, _)| blocked_attacker_id == attacker_id)
        {
            add_damage(&mut damage_received, blocker_id, *power);
            damage_events.push(DamageEvent {
                source: attacker_id.clone(),
                target: DamageTarget::Creature(blocker_id.clone()),
                damage_amount: *power,
            });
        } else {
            player_damage += *power;
            damage_events.push(DamageEvent {
                source: attacker_id.clone(),
                target: DamageTarget::Player(defender_player_id.clone()),
                damage_amount: *power,
            });
        }
    }

    for (blocker_id, attacker_id, power) in &blockers {
        add_damage(&mut damage_received, attacker_id, *power);
        damage_events.push(DamageEvent {
            source: blocker_id.clone(),
            target: DamageTarget::Creature(attacker_id.clone()),
            damage_amount: *power,
        });
    }

    apply_damage_and_clear_combat_state(players, &damage_received);
    let player_life_change = if player_damage > 0 {
        let life_delta = i32::try_from(player_damage).map_err(|_| {
            DomainError::Game(GameError::InternalInvariantViolation(
                "combat damage should fit within i32 life adjustments".to_string(),
            ))
        })?;
        Some(game_effects::adjust_player_life(
            game_id,
            players,
            &defender_player_id,
            -life_delta,
        )?)
    } else {
        None
    };
    let StateBasedActionsResult {
        creatures_died,
        game_ended,
    } = state_based_actions::check_state_based_actions(game_id, players, terminal_state)?;

    Ok(ResolveCombatDamageOutcome::new(
        CombatDamageResolved::new(game_id.clone(), cmd.player_id, damage_events),
        player_life_change,
        creatures_died,
        game_ended,
    ))
}
