//! Supports rules combat damage.

mod application;
mod participants;

use self::{
    application::{apply_damage, clear_combat_state},
    participants::{
        collect_attackers, collect_blockers, AttackerParticipant, BlockerParticipant, CombatCardRef,
    },
};
use super::super::{
    super::{model::Player, AggregateCardLocationIndex, TerminalState},
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

type DamageResolution = (Vec<DamageEvent>, Vec<(CardInstanceId, u32)>, u32);
type DamageStepOutcome = (
    Vec<DamageEvent>,
    Option<LifeChanged>,
    Vec<CreatureDied>,
    Option<GameEnded>,
);

struct DamageStepContext<'a> {
    game_id: &'a GameId,
    players: &'a mut [Player],
    card_locations: &'a AggregateCardLocationIndex,
    terminal_state: &'a mut TerminalState,
    defender_player_id: &'a crate::domain::play::ids::PlayerId,
}

fn resolve_combat_card_id(
    players: &[Player],
    card_ref: &CombatCardRef,
    missing_message: &str,
) -> Result<CardInstanceId, DomainError> {
    players[card_ref.owner_index()]
        .card_by_handle(card_ref.handle())
        .map(|card| card.id().clone())
        .ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(
                missing_message.to_string(),
            ))
        })
}

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

fn blockers_for_attacker<'a>(
    blockers: &'a [BlockerParticipant],
    attacker: &AttackerParticipant,
) -> Vec<&'a BlockerParticipant> {
    attacker
        .blocked_by_refs()
        .iter()
        .filter_map(|blocker_ref| {
            blockers.iter().find(|blocker| {
                blocker.card_ref().owner_index() == blocker_ref.owner_index()
                    && blocker.card_ref().handle() == blocker_ref.handle()
            })
        })
        .collect()
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
    players: &[Player],
    attackers: &[AttackerParticipant],
    blockers: &[BlockerParticipant],
    defender_player_id: &crate::domain::play::ids::PlayerId,
    attackers_deal_damage: impl Fn(&AttackerParticipant) -> bool,
    blockers_deal_damage: impl Fn(&BlockerParticipant) -> bool,
) -> Result<DamageResolution, DomainError> {
    let mut damage_events: Vec<DamageEvent> = Vec::new();
    let mut damage_received: Vec<(CardInstanceId, u32)> = Vec::new();
    let mut player_damage = 0;

    for attacker in attackers
        .iter()
        .filter(|attacker| attackers_deal_damage(attacker))
    {
        let attacker_id = resolve_combat_card_id(
            players,
            attacker.card_ref(),
            "combat attacker participant points to a missing battlefield card",
        )?;
        let ordered_blockers = blockers_for_attacker(blockers, attacker);
        if !ordered_blockers.is_empty() {
            let mut remaining_damage = attacker.power();
            for (index, blocker) in ordered_blockers.iter().enumerate() {
                let blocker_id = resolve_combat_card_id(
                    players,
                    blocker.card_ref(),
                    "combat blocker participant points to a missing battlefield card",
                )?;
                let is_last = index + 1 == ordered_blockers.len();
                let lethal_to_blocker = blocker.lethal_damage_threshold();
                let blocker_damage = if attacker.has_trample() {
                    remaining_damage.min(lethal_to_blocker)
                } else if is_last {
                    remaining_damage
                } else {
                    remaining_damage.min(lethal_to_blocker)
                };

                add_damage(&mut damage_received, &blocker_id, blocker_damage);
                damage_events.push(DamageEvent {
                    source: attacker_id.clone(),
                    target: DamageTarget::Creature(blocker_id),
                    damage_amount: blocker_damage,
                });
                remaining_damage = remaining_damage.saturating_sub(blocker_damage);
            }

            if attacker.has_trample() && remaining_damage > 0 {
                player_damage += remaining_damage;
                damage_events.push(DamageEvent {
                    source: attacker_id.clone(),
                    target: DamageTarget::Player(defender_player_id.clone()),
                    damage_amount: remaining_damage,
                });
            }
        } else {
            player_damage += attacker.power();
            damage_events.push(DamageEvent {
                source: attacker_id,
                target: DamageTarget::Player(defender_player_id.clone()),
                damage_amount: attacker.power(),
            });
        }
    }

    for blocker in blockers
        .iter()
        .filter(|blocker| blockers_deal_damage(blocker))
    {
        let blocker_id = resolve_combat_card_id(
            players,
            blocker.card_ref(),
            "combat blocker participant points to a missing battlefield card",
        )?;
        let blocked_attacker_id = resolve_combat_card_id(
            players,
            blocker.blocked_attacker_ref(),
            "combat blocker participant points to a missing blocked attacker",
        )?;
        add_damage(&mut damage_received, &blocked_attacker_id, blocker.power());
        damage_events.push(DamageEvent {
            source: blocker_id,
            target: DamageTarget::Creature(blocked_attacker_id),
            damage_amount: blocker.power(),
        });
    }

    Ok((damage_events, damage_received, player_damage))
}

fn apply_player_combat_damage(
    game_id: &GameId,
    players: &mut [Player],
    defender_player_id: &crate::domain::play::ids::PlayerId,
    player_damage: u32,
) -> Result<Option<LifeChanged>, DomainError> {
    if player_damage == 0 {
        return Ok(None);
    }

    let life_delta = i32::try_from(player_damage).map_err(|_| {
        DomainError::Game(GameError::InternalInvariantViolation(
            "combat damage should fit within i32 life adjustments".to_string(),
        ))
    })?;
    Ok(Some(game_effects::adjust_player_life(
        game_id,
        players,
        defender_player_id,
        -life_delta,
    )?))
}

fn resolve_combat_damage_step(
    ctx: &mut DamageStepContext<'_>,
    attackers: &[AttackerParticipant],
    blockers: &[BlockerParticipant],
    attackers_deal_damage: impl Fn(&AttackerParticipant) -> bool,
    blockers_deal_damage: impl Fn(&BlockerParticipant) -> bool,
) -> Result<DamageStepOutcome, DomainError> {
    let (damage_events, damage_received, player_damage) = resolve_damage_step(
        ctx.players,
        attackers,
        blockers,
        ctx.defender_player_id,
        attackers_deal_damage,
        blockers_deal_damage,
    )?;
    apply_damage(ctx.players, ctx.card_locations, &damage_received);
    let life_changed = apply_player_combat_damage(
        ctx.game_id,
        ctx.players,
        ctx.defender_player_id,
        player_damage,
    )?;
    let StateBasedActionsResult {
        creatures_died,
        game_ended,
    } = state_based_actions::check_state_based_actions(
        ctx.game_id,
        ctx.players,
        ctx.terminal_state,
    )?;

    Ok((damage_events, life_changed, creatures_died, game_ended))
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
    card_locations: &AggregateCardLocationIndex,
    terminal_state: &mut TerminalState,
    cmd: ResolveCombatDamageCommand,
    attacker_player_idx: usize,
    defender_idx: usize,
) -> Result<ResolveCombatDamageOutcome, DomainError> {
    let defender_player_id = players[defender_idx].id().clone();
    let attackers = collect_attackers(
        &players[attacker_player_idx],
        attacker_player_idx,
        defender_idx,
    )?;
    if attackers.is_empty() {
        return Err(DomainError::Game(GameError::NoAttackersDeclared));
    }
    let blockers = collect_blockers(
        &players[defender_idx],
        defender_idx,
        &players[attacker_player_idx],
        attacker_player_idx,
    )?;
    let has_first_strike_step = attackers.iter().any(AttackerParticipant::has_first_strike)
        || blockers.iter().any(BlockerParticipant::has_first_strike);

    let mut damage_events: Vec<DamageEvent> = Vec::new();
    let mut life_changed: Option<LifeChanged> = None;
    let mut creatures_died: Vec<CreatureDied> = Vec::new();
    let mut game_ended: Option<GameEnded> = None;

    let (
        first_step_events,
        first_step_life_changed,
        first_step_creatures_died,
        first_step_game_ended,
    ) = if has_first_strike_step {
        resolve_combat_damage_step(
            &mut DamageStepContext {
                game_id,
                players,
                card_locations,
                terminal_state,
                defender_player_id: &defender_player_id,
            },
            &attackers,
            &blockers,
            AttackerParticipant::has_first_strike,
            BlockerParticipant::has_first_strike,
        )
    } else {
        resolve_combat_damage_step(
            &mut DamageStepContext {
                game_id,
                players,
                card_locations,
                terminal_state,
                defender_player_id: &defender_player_id,
            },
            &attackers,
            &blockers,
            |_| true,
            |_| true,
        )
    }?;

    damage_events.extend(first_step_events);
    merge_life_changed(&mut life_changed, first_step_life_changed);
    creatures_died.extend(first_step_creatures_died);
    if let Some(ended) = first_step_game_ended {
        game_ended = Some(ended);
    }

    if has_first_strike_step && game_ended.is_none() {
        let surviving_attackers = collect_attackers(
            &players[attacker_player_idx],
            attacker_player_idx,
            defender_idx,
        )?;
        let surviving_blockers = collect_blockers(
            &players[defender_idx],
            defender_idx,
            &players[attacker_player_idx],
            attacker_player_idx,
        )?;
        let (
            second_step_events,
            second_step_life_changed,
            second_step_creatures_died,
            second_step_game_ended,
        ) = resolve_combat_damage_step(
            &mut DamageStepContext {
                game_id,
                players,
                card_locations,
                terminal_state,
                defender_player_id: &defender_player_id,
            },
            &surviving_attackers,
            &surviving_blockers,
            |attacker| !attacker.has_first_strike(),
            |blocker| !blocker.has_first_strike(),
        )?;
        damage_events.extend(second_step_events);
        merge_life_changed(&mut life_changed, second_step_life_changed);
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
