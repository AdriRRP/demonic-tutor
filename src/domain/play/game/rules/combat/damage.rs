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
    super::{
        model::{Player, StackZone},
        AggregateCardLocationIndex, TerminalState,
    },
    game_effects, stack_priority, state_based_actions,
    state_based_actions::StateBasedActionsResult,
};
use crate::domain::play::{
    cards::TriggeredAbilityEvent,
    commands::ResolveCombatDamageCommand,
    errors::{DomainError, GameError},
    events::{
        CardMovedZone, CombatDamageResolved, CreatureDied, DamageEvent, DamageTarget, GameEnded,
        LifeChanged, TriggeredAbilityPutOnStack,
    },
    ids::{CardInstanceId, GameId},
};
use std::collections::HashMap;

struct CreatureDamageAssignment {
    target: CardInstanceId,
    damage: u32,
    source_has_deathtouch: bool,
}

type DamageResolution = (
    Vec<DamageEvent>,
    Vec<CreatureDamageAssignment>,
    u32,
    HashMap<usize, u32>,
);
type DamageStepOutcome = (
    Vec<DamageEvent>,
    Vec<LifeChanged>,
    Vec<CreatureDied>,
    Vec<CardMovedZone>,
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
    players[card_ref.player_index()]
        .card_by_handle(card_ref.handle())
        .map(|card| card.id().clone())
        .ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(
                missing_message.to_string(),
            ))
        })
}

fn add_damage(
    damage_index_by_target: &mut HashMap<CardInstanceId, usize>,
    damage_received: &mut Vec<CreatureDamageAssignment>,
    target: &CardInstanceId,
    damage: u32,
    source_has_deathtouch: bool,
) {
    if let Some(existing_index) = damage_index_by_target.get(target).copied() {
        let existing = &mut damage_received[existing_index];
        existing.damage += damage;
        existing.source_has_deathtouch |= source_has_deathtouch && damage > 0;
    } else {
        damage_index_by_target.insert(target.clone(), damage_received.len());
        damage_received.push(CreatureDamageAssignment {
            target: target.clone(),
            damage,
            source_has_deathtouch: source_has_deathtouch && damage > 0,
        });
    }
}

fn blockers_for_attacker<'a>(
    blockers_by_ref: &HashMap<
        (usize, crate::domain::play::ids::PlayerCardHandle),
        &'a BlockerParticipant,
    >,
    attacker: &AttackerParticipant,
) -> Vec<&'a BlockerParticipant> {
    attacker
        .blocked_by_refs()
        .iter()
        .filter_map(|blocker_ref| {
            blockers_by_ref
                .get(&(blocker_ref.player_index(), blocker_ref.handle()))
                .copied()
        })
        .collect()
}

fn merge_life_changed(aggregate: &mut Vec<LifeChanged>, mut step_life_changed: Vec<LifeChanged>) {
    aggregate.append(&mut step_life_changed);
}

fn record_lifelink_damage(
    lifelink_damage_by_controller: &mut HashMap<usize, u32>,
    controller_index: usize,
    damage: u32,
) {
    if damage > 0 {
        *lifelink_damage_by_controller
            .entry(controller_index)
            .or_insert(0) += damage;
    }
}

#[allow(clippy::too_many_arguments)]
fn resolve_attacker_damage(
    players: &[Player],
    attacker: &AttackerParticipant,
    blockers_by_ref: &HashMap<
        (usize, crate::domain::play::ids::PlayerCardHandle),
        &BlockerParticipant,
    >,
    defender_player_id: &crate::domain::play::ids::PlayerId,
    damage_events: &mut Vec<DamageEvent>,
    damage_index_by_target: &mut HashMap<CardInstanceId, usize>,
    damage_received: &mut Vec<CreatureDamageAssignment>,
    lifelink_damage_by_controller: &mut HashMap<usize, u32>,
) -> Result<u32, DomainError> {
    let attacker_id = resolve_combat_card_id(
        players,
        attacker.card_ref(),
        "combat attacker participant points to a missing battlefield card",
    )?;
    let ordered_blockers = blockers_for_attacker(blockers_by_ref, attacker);
    if ordered_blockers.is_empty() {
        if attacker.was_blocked() {
            return Ok(0);
        }

        if attacker.has_lifelink() {
            record_lifelink_damage(
                lifelink_damage_by_controller,
                attacker.card_ref().player_index(),
                attacker.power(),
            );
        }
        damage_events.push(DamageEvent {
            source: attacker_id,
            target: DamageTarget::Player(defender_player_id.clone()),
            damage_amount: attacker.power(),
        });
        return Ok(attacker.power());
    }

    let mut remaining_damage = attacker.power();
    for (index, blocker) in ordered_blockers.iter().enumerate() {
        let blocker_id = resolve_combat_card_id(
            players,
            blocker.card_ref(),
            "combat blocker participant points to a missing battlefield card",
        )?;
        let is_last = index + 1 == ordered_blockers.len();
        let lethal_to_blocker = if attacker.has_deathtouch() {
            u32::from(blocker.lethal_damage_threshold() > 0)
        } else {
            blocker.lethal_damage_threshold()
        };
        let blocker_damage = if attacker.has_trample() {
            remaining_damage.min(lethal_to_blocker)
        } else if is_last {
            remaining_damage
        } else {
            remaining_damage.min(lethal_to_blocker)
        };

        add_damage(
            damage_index_by_target,
            damage_received,
            &blocker_id,
            blocker_damage,
            attacker.has_deathtouch(),
        );
        if attacker.has_lifelink() {
            record_lifelink_damage(
                lifelink_damage_by_controller,
                attacker.card_ref().player_index(),
                blocker_damage,
            );
        }
        damage_events.push(DamageEvent {
            source: attacker_id.clone(),
            target: DamageTarget::Creature(blocker_id),
            damage_amount: blocker_damage,
        });
        remaining_damage = remaining_damage.saturating_sub(blocker_damage);
    }

    if attacker.has_trample() && remaining_damage > 0 {
        damage_events.push(DamageEvent {
            source: attacker_id,
            target: DamageTarget::Player(defender_player_id.clone()),
            damage_amount: remaining_damage,
        });
        return Ok(remaining_damage);
    }

    Ok(0)
}

fn resolve_blocker_damage(
    players: &[Player],
    blocker: &BlockerParticipant,
    damage_events: &mut Vec<DamageEvent>,
    damage_index_by_target: &mut HashMap<CardInstanceId, usize>,
    damage_received: &mut Vec<CreatureDamageAssignment>,
    lifelink_damage_by_controller: &mut HashMap<usize, u32>,
) -> Result<(), DomainError> {
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
    add_damage(
        damage_index_by_target,
        damage_received,
        &blocked_attacker_id,
        blocker.power(),
        blocker.has_deathtouch(),
    );
    if blocker.has_lifelink() {
        record_lifelink_damage(
            lifelink_damage_by_controller,
            blocker.card_ref().player_index(),
            blocker.power(),
        );
    }
    damage_events.push(DamageEvent {
        source: blocker_id,
        target: DamageTarget::Creature(blocked_attacker_id),
        damage_amount: blocker.power(),
    });
    Ok(())
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
    let mut damage_received: Vec<CreatureDamageAssignment> = Vec::new();
    let mut damage_index_by_target = HashMap::new();
    let mut player_damage = 0;
    let mut lifelink_damage_by_controller = HashMap::new();
    let blockers_by_ref = blockers
        .iter()
        .map(|blocker| {
            (
                (
                    blocker.card_ref().player_index(),
                    blocker.card_ref().handle(),
                ),
                blocker,
            )
        })
        .collect::<HashMap<_, _>>();

    for attacker in attackers
        .iter()
        .filter(|attacker| attackers_deal_damage(attacker))
    {
        player_damage += resolve_attacker_damage(
            players,
            attacker,
            &blockers_by_ref,
            defender_player_id,
            &mut damage_events,
            &mut damage_index_by_target,
            &mut damage_received,
            &mut lifelink_damage_by_controller,
        )?;
    }

    for blocker in blockers
        .iter()
        .filter(|blocker| blockers_deal_damage(blocker))
    {
        resolve_blocker_damage(
            players,
            blocker,
            &mut damage_events,
            &mut damage_index_by_target,
            &mut damage_received,
            &mut lifelink_damage_by_controller,
        )?;
    }

    Ok((
        damage_events,
        damage_received,
        player_damage,
        lifelink_damage_by_controller,
    ))
}

fn apply_player_combat_damage(
    game_id: &GameId,
    players: &mut [Player],
    defender_player_id: &crate::domain::play::ids::PlayerId,
    player_damage: u32,
) -> Result<Vec<LifeChanged>, DomainError> {
    if player_damage == 0 {
        return Ok(Vec::new());
    }

    let life_delta = i32::try_from(player_damage).map_err(|_| {
        DomainError::Game(GameError::InternalInvariantViolation(
            "combat damage should fit within i32 life adjustments".to_string(),
        ))
    })?;
    Ok(vec![game_effects::adjust_player_life(
        game_id,
        players,
        defender_player_id,
        -life_delta,
    )?])
}

fn apply_lifelink_gains(
    game_id: &GameId,
    players: &mut [Player],
    lifelink_damage_by_controller: HashMap<usize, u32>,
) -> Result<Vec<LifeChanged>, DomainError> {
    let mut events = Vec::new();

    for (controller_index, gained_life) in lifelink_damage_by_controller {
        if gained_life == 0 {
            continue;
        }
        let life_delta = i32::try_from(gained_life).map_err(|_| {
            DomainError::Game(GameError::InternalInvariantViolation(
                "lifelink life gain should fit within i32 life adjustments".to_string(),
            ))
        })?;
        events.push(game_effects::adjust_player_life_by_index(
            game_id,
            players,
            controller_index,
            life_delta,
        )?);
    }

    Ok(events)
}

fn resolve_combat_damage_step(
    ctx: &mut DamageStepContext<'_>,
    attackers: &[AttackerParticipant],
    blockers: &[BlockerParticipant],
    attackers_deal_damage: impl Fn(&AttackerParticipant) -> bool,
    blockers_deal_damage: impl Fn(&BlockerParticipant) -> bool,
) -> Result<DamageStepOutcome, DomainError> {
    let (damage_events, damage_received, player_damage, lifelink_damage_by_controller) =
        resolve_damage_step(
            ctx.players,
            attackers,
            blockers,
            ctx.defender_player_id,
            attackers_deal_damage,
            blockers_deal_damage,
        )?;
    apply_damage(ctx.players, ctx.card_locations, &damage_received);
    let mut life_changed = apply_player_combat_damage(
        ctx.game_id,
        ctx.players,
        ctx.defender_player_id,
        player_damage,
    )?;
    life_changed.extend(apply_lifelink_gains(
        ctx.game_id,
        ctx.players,
        lifelink_damage_by_controller,
    )?);
    let StateBasedActionsResult {
        creatures_died,
        zone_changes,
        game_ended,
    } = state_based_actions::check_state_based_actions(
        ctx.game_id,
        ctx.players,
        ctx.terminal_state,
    )?;

    Ok((damage_events, life_changed, creatures_died, zone_changes, game_ended))
}

#[derive(Debug, Clone)]
pub struct ResolveCombatDamageOutcome {
    pub combat_damage_resolved: CombatDamageResolved,
    pub life_changed: Vec<LifeChanged>,
    pub creatures_died: Vec<CreatureDied>,
    pub zone_changes: Vec<CardMovedZone>,
    pub triggered_abilities_put_on_stack: Vec<TriggeredAbilityPutOnStack>,
    pub game_ended: Option<GameEnded>,
}

impl ResolveCombatDamageOutcome {
    #[must_use]
    pub const fn new(
        combat_damage_resolved: CombatDamageResolved,
        life_changed: Vec<LifeChanged>,
        creatures_died: Vec<CreatureDied>,
        zone_changes: Vec<CardMovedZone>,
        triggered_abilities_put_on_stack: Vec<TriggeredAbilityPutOnStack>,
        game_ended: Option<GameEnded>,
    ) -> Self {
        Self {
            combat_damage_resolved,
            life_changed,
            creatures_died,
            zone_changes,
            triggered_abilities_put_on_stack,
            game_ended,
        }
    }
}

#[allow(clippy::too_many_lines, clippy::too_many_arguments)]
pub fn resolve_combat_damage(
    game_id: &GameId,
    players: &mut [Player],
    card_locations: &AggregateCardLocationIndex,
    stack: &mut StackZone,
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
    let has_first_strike_step = attackers
        .iter()
        .any(|attacker| attacker.has_first_strike() || attacker.has_double_strike())
        || blockers
            .iter()
            .any(|blocker| blocker.has_first_strike() || blocker.has_double_strike());

    let mut damage_events: Vec<DamageEvent> = Vec::new();
    let mut life_changed: Vec<LifeChanged> = Vec::new();
    let mut creatures_died: Vec<CreatureDied> = Vec::new();
    let mut zone_changes: Vec<CardMovedZone> = Vec::new();
    let mut game_ended: Option<GameEnded> = None;

    let (
        first_step_events,
        first_step_life_changed,
        first_step_creatures_died,
        first_step_zone_changes,
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
            |attacker| attacker.has_first_strike() || attacker.has_double_strike(),
            |blocker| blocker.has_first_strike() || blocker.has_double_strike(),
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
    zone_changes.extend(first_step_zone_changes);
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
            second_step_zone_changes,
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
            |attacker| !attacker.has_first_strike() || attacker.has_double_strike(),
            |blocker| !blocker.has_first_strike() || blocker.has_double_strike(),
        )?;
        damage_events.extend(second_step_events);
        merge_life_changed(&mut life_changed, second_step_life_changed);
        creatures_died.extend(second_step_creatures_died);
        zone_changes.extend(second_step_zone_changes);
        game_ended = second_step_game_ended;
    }

    clear_combat_state(players);
    let mut triggered_abilities_put_on_stack = Vec::new();
    if game_ended.is_none() {
        let mut trigger_sources = Vec::new();
        for damage_event in &damage_events {
            if let DamageTarget::Player(_) = &damage_event.target {
                if !trigger_sources
                    .iter()
                    .any(|source| source == &damage_event.source)
                {
                    trigger_sources.push(damage_event.source.clone());
                }
            }
        }

        for source_card_id in trigger_sources {
            for controller_index in 0..players.len() {
                let Some(handle) =
                    players[controller_index].resolve_public_card_handle(&source_card_id)
                else {
                    continue;
                };
                triggered_abilities_put_on_stack.extend(
                    stack_priority::triggers::enqueue_trigger_for_card_handle(
                        game_id,
                        players,
                        controller_index,
                        handle,
                        TriggeredAbilityEvent::DealsCombatDamageToPlayer,
                        stack,
                    )?,
                );
                break;
            }
        }
    }

    Ok(ResolveCombatDamageOutcome::new(
        CombatDamageResolved::new(game_id.clone(), cmd.player_id, damage_events),
        life_changed,
        creatures_died,
        zone_changes,
        triggered_abilities_put_on_stack,
        game_ended,
    ))
}
