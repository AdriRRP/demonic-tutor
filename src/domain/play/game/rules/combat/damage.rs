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
use std::collections::HashMap;

type CombatantPower = (CardInstanceId, u32);
type BlockAssignment = (CardInstanceId, CardInstanceId);
type BlockingCombatantPower = (CardInstanceId, CardInstanceId, u32);
type CombatAssignments = HashMap<CardInstanceId, Vec<CardInstanceId>>;

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

fn collect_attackers(player: &Player) -> Result<Vec<CombatantPower>, DomainError> {
    player
        .battlefield()
        .cards()
        .iter()
        .filter(|card| card.is_attacking())
        .map(|card| {
            let (power, _) = card.creature_stats().ok_or_else(|| {
                DomainError::Game(GameError::InternalInvariantViolation(format!(
                    "attacking creature {} must have power and toughness",
                    card.id()
                )))
            })?;

            Ok((card.id().clone(), power))
        })
        .collect()
}

fn collect_blockers(player: &Player) -> Result<Vec<BlockingCombatantPower>, DomainError> {
    player
        .battlefield()
        .cards()
        .iter()
        .filter(|card| card.is_blocking())
        .map(|card| {
            let (power, _) = card.creature_stats().ok_or_else(|| {
                DomainError::Game(GameError::InternalInvariantViolation(format!(
                    "blocking creature {} must have power and toughness",
                    card.id()
                )))
            })?;
            let attacker_id = card.blocking_target().cloned().ok_or_else(|| {
                DomainError::Game(GameError::InternalInvariantViolation(format!(
                    "blocking creature {} must have an assigned attacker",
                    card.id()
                )))
            })?;

            Ok((card.id().clone(), attacker_id, power))
        })
        .collect()
}

fn group_assignments_by_attacker(assignments: &[BlockAssignment]) -> CombatAssignments {
    let mut grouped = HashMap::new();

    for (blocker_id, attacker_id) in assignments {
        grouped
            .entry(attacker_id.clone())
            .or_insert_with(Vec::new)
            .push(blocker_id.clone());
    }

    grouped
}

fn group_assignments_by_blocker(assignments: &[BlockAssignment]) -> CombatAssignments {
    let mut grouped = HashMap::new();

    for (blocker_id, attacker_id) in assignments {
        grouped
            .entry(blocker_id.clone())
            .or_insert_with(Vec::new)
            .push(attacker_id.clone());
    }

    grouped
}

fn blocking_assignments(player: &Player) -> Vec<BlockAssignment> {
    player
        .battlefield()
        .cards()
        .iter()
        .filter_map(|card| {
            card.blocking_target()
                .map(|attacker_id| (card.id().clone(), attacker_id.clone()))
        })
        .collect()
}

fn apply_damage_and_clear_combat_state(
    players: &mut [Player],
    damage_received: &HashMap<CardInstanceId, u32>,
) {
    for player in players.iter_mut() {
        for card in player.battlefield_mut().iter_mut() {
            if let Some(damage) = damage_received.get(card.id()) {
                card.add_damage(*damage);
            }
            card.set_attacking(false);
            card.set_blocking(false);
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
    let assignments = blocking_assignments(&players[defender_idx]);
    let blockers_by_attacker = group_assignments_by_attacker(&assignments);
    let attackers_by_blocker = group_assignments_by_blocker(&assignments);

    let mut damage_events: Vec<DamageEvent> = Vec::new();
    let mut damage_received: HashMap<CardInstanceId, u32> = HashMap::new();
    let mut player_damage = 0;

    for (attacker_id, power) in &attackers {
        let blocking_for_attacker = blockers_by_attacker.get(attacker_id);

        if blocking_for_attacker.is_none_or(Vec::is_empty) {
            player_damage += *power;
            damage_events.push(DamageEvent {
                source: attacker_id.clone(),
                target: DamageTarget::Player(defender_player_id.clone()),
                damage_amount: *power,
            });
        } else {
            for blocker_id in blocking_for_attacker.into_iter().flatten() {
                *damage_received.entry(blocker_id.clone()).or_insert(0) += *power;
                damage_events.push(DamageEvent {
                    source: attacker_id.clone(),
                    target: DamageTarget::Creature(blocker_id.clone()),
                    damage_amount: *power,
                });
            }
        }
    }

    for (blocker_id, attacker_id, power) in &blockers {
        if attackers_by_blocker.contains_key(blocker_id) {
            *damage_received.entry(attacker_id.clone()).or_insert(0) += *power;
            damage_events.push(DamageEvent {
                source: blocker_id.clone(),
                target: DamageTarget::Creature(attacker_id.clone()),
                damage_amount: *power,
            });
        }
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
