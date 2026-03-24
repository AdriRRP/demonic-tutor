mod application;
mod participants;

use self::{
    application::{apply_damage, clear_combat_state},
    participants::{collect_attackers, collect_blockers, AttackerParticipant, BlockerParticipant},
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

fn blocker_for_attacker<'a>(
    blockers: &'a [BlockerParticipant],
    attacker_id: &CardInstanceId,
) -> Option<&'a BlockerParticipant> {
    blockers
        .iter()
        .find(|blocker| blocker.blocked_attacker_id() == attacker_id)
}

fn merge_life_changed(aggregate: &mut Option<LifeChanged>, step_life_changed: Option<LifeChanged>) {
    let Some(step_life_changed) = step_life_changed else {
        return;
    };

    match aggregate {
        Some(existing) => existing.to_life = step_life_changed.to_life,
        None => *aggregate = Some(step_life_changed),
    }
}

fn resolve_damage_step(
    attackers: &[AttackerParticipant],
    blockers: &[BlockerParticipant],
    defender_player_id: &crate::domain::play::ids::PlayerId,
    attackers_deal_damage: impl Fn(&AttackerParticipant) -> bool,
    blockers_deal_damage: impl Fn(&BlockerParticipant) -> bool,
) -> (Vec<DamageEvent>, Vec<(CardInstanceId, u32)>, u32) {
    let mut damage_events: Vec<DamageEvent> = Vec::new();
    let mut damage_received: Vec<(CardInstanceId, u32)> = Vec::new();
    let mut player_damage = 0;

    for attacker in attackers
        .iter()
        .filter(|attacker| attackers_deal_damage(attacker))
    {
        if let Some(blocker) = blocker_for_attacker(blockers, attacker.id()) {
            let lethal_to_blocker = blocker.lethal_damage_threshold();
            let blocker_damage = if attacker.has_trample() {
                attacker.power().min(lethal_to_blocker)
            } else {
                attacker.power()
            };
            let excess_to_player = if attacker.has_trample() {
                attacker.power().saturating_sub(blocker_damage)
            } else {
                0
            };

            add_damage(&mut damage_received, blocker.id(), blocker_damage);
            damage_events.push(DamageEvent {
                source: attacker.id().clone(),
                target: DamageTarget::Creature(blocker.id().clone()),
                damage_amount: blocker_damage,
            });
            if excess_to_player > 0 {
                player_damage += excess_to_player;
                damage_events.push(DamageEvent {
                    source: attacker.id().clone(),
                    target: DamageTarget::Player(defender_player_id.clone()),
                    damage_amount: excess_to_player,
                });
            }
        } else {
            player_damage += attacker.power();
            damage_events.push(DamageEvent {
                source: attacker.id().clone(),
                target: DamageTarget::Player(defender_player_id.clone()),
                damage_amount: attacker.power(),
            });
        }
    }

    for blocker in blockers
        .iter()
        .filter(|blocker| blockers_deal_damage(blocker))
    {
        add_damage(
            &mut damage_received,
            blocker.blocked_attacker_id(),
            blocker.power(),
        );
        damage_events.push(DamageEvent {
            source: blocker.id().clone(),
            target: DamageTarget::Creature(blocker.blocked_attacker_id().clone()),
            damage_amount: blocker.power(),
        });
    }

    (damage_events, damage_received, player_damage)
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
    let has_first_strike_step = attackers.iter().any(AttackerParticipant::has_first_strike)
        || blockers.iter().any(BlockerParticipant::has_first_strike);

    let mut damage_events: Vec<DamageEvent> = Vec::new();
    let mut life_changed: Option<LifeChanged> = None;
    let mut creatures_died: Vec<CreatureDied> = Vec::new();
    let mut game_ended: Option<GameEnded> = None;

    let (first_step_events, first_step_damage, first_step_player_damage) = if has_first_strike_step
    {
        resolve_damage_step(
            &attackers,
            &blockers,
            &defender_player_id,
            AttackerParticipant::has_first_strike,
            BlockerParticipant::has_first_strike,
        )
    } else {
        resolve_damage_step(
            &attackers,
            &blockers,
            &defender_player_id,
            |_| true,
            |_| true,
        )
    };

    damage_events.extend(first_step_events);
    apply_damage(players, &first_step_damage);
    let first_step_life_changed = if first_step_player_damage > 0 {
        let life_delta = i32::try_from(first_step_player_damage).map_err(|_| {
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
    merge_life_changed(&mut life_changed, first_step_life_changed);
    let StateBasedActionsResult {
        creatures_died: first_step_creatures_died,
        game_ended: first_step_game_ended,
    } = state_based_actions::check_state_based_actions(game_id, players, terminal_state)?;
    creatures_died.extend(first_step_creatures_died);
    if let Some(ended) = first_step_game_ended {
        game_ended = Some(ended);
    }

    if has_first_strike_step && game_ended.is_none() {
        let surviving_attackers = collect_attackers(&players[attacker_player_idx])?;
        let surviving_blockers = collect_blockers(&players[defender_idx])?;
        let (second_step_events, second_step_damage, second_step_player_damage) =
            resolve_damage_step(
                &surviving_attackers,
                &surviving_blockers,
                &defender_player_id,
                |attacker| !attacker.has_first_strike(),
                |blocker| !blocker.has_first_strike(),
            );
        damage_events.extend(second_step_events);
        apply_damage(players, &second_step_damage);
        let second_step_life_changed = if second_step_player_damage > 0 {
            let life_delta = i32::try_from(second_step_player_damage).map_err(|_| {
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
        merge_life_changed(&mut life_changed, second_step_life_changed);
        let StateBasedActionsResult {
            creatures_died: second_step_creatures_died,
            game_ended: second_step_game_ended,
        } = state_based_actions::check_state_based_actions(game_id, players, terminal_state)?;
        creatures_died.extend(second_step_creatures_died);
        game_ended = second_step_game_ended;
    }

    clear_combat_state(players);

    Ok(ResolveCombatDamageOutcome::new(
        CombatDamageResolved::new(game_id.clone(), cmd.player_id, damage_events),
        life_changed,
        creatures_died,
        game_ended,
    ))
}
