use super::{
    super::{helpers, invariants, model::Player, TerminalState},
    game_effects, state_based_actions,
};
use crate::domain::play::{
    cards::CardType,
    commands::{DeclareAttackersCommand, DeclareBlockersCommand, ResolveCombatDamageCommand},
    errors::{CardError, DomainError, GameError, PhaseError},
    events::{
        AttackersDeclared, BlockersDeclared, CombatDamageResolved, CreatureDied, DamageEvent,
        DamageTarget, GameEnded, LifeChanged,
    },
    ids::{CardInstanceId, GameId, PlayerId},
    phase::Phase,
};
use std::collections::{HashMap, HashSet};

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

fn require_defending_player(
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

const fn require_attackers_step(phase: Phase) -> Result<(), DomainError> {
    if !matches!(phase, Phase::DeclareAttackers) {
        return Err(DomainError::Phase(PhaseError::InvalidForCombat));
    }

    Ok(())
}

const fn require_blockers_step(phase: Phase) -> Result<(), DomainError> {
    if !matches!(phase, Phase::DeclareBlockers) {
        return Err(DomainError::Phase(PhaseError::InvalidForCombat));
    }

    Ok(())
}

const fn require_combat_damage_step(phase: Phase) -> Result<(), DomainError> {
    if !matches!(phase, Phase::CombatDamage) {
        return Err(DomainError::Phase(PhaseError::InvalidForCombat));
    }

    Ok(())
}

fn find_defending_player_index(
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
    require_attackers_step(*phase)?;

    let player_idx = helpers::find_player_index(players, &cmd.player_id)?;
    let player = &mut players[player_idx];
    let battlefield = player.battlefield_mut();
    let mut valid_attackers: Vec<CardInstanceId> = Vec::new();

    for attacker_id in &cmd.attacker_ids {
        let card = battlefield.card_mut(attacker_id).ok_or_else(|| {
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
        game_id.clone(),
        cmd.player_id,
        valid_attackers,
    ))
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
    require_defending_player(active_player, &cmd.player_id)?;
    require_blockers_step(*phase)?;

    let defending_player_idx = find_defending_player_index(players, active_player)?;
    let attacker_player_idx = helpers::find_player_index(players, active_player)?;
    let declared_attackers = players[attacker_player_idx]
        .battlefield()
        .cards()
        .iter()
        .filter(|card| card.is_attacking())
        .map(|card| (card.id().clone(), card.has_flying()))
        .collect::<HashMap<_, _>>();
    let defender = &mut players[defending_player_idx];
    let battlefield = defender.battlefield_mut();
    let mut valid_blockers: Vec<(CardInstanceId, CardInstanceId)> = Vec::new();
    let mut seen_blockers = HashSet::new();
    let mut seen_attackers = HashSet::new();

    for (blocker_id, attacker_id) in &cmd.blocker_assignments {
        if !seen_blockers.insert(blocker_id.clone()) {
            return Err(DomainError::Game(GameError::DuplicateBlockerAssignment(
                blocker_id.clone(),
            )));
        }

        if !seen_attackers.insert(attacker_id.clone()) {
            return Err(DomainError::Game(
                GameError::MultipleBlockersPerAttackerNotSupported(attacker_id.clone()),
            ));
        }

        if !declared_attackers.contains_key(attacker_id) {
            return Err(DomainError::Card(CardError::NotAttacking(
                attacker_id.clone(),
            )));
        }

        let attacker_has_flying = declared_attackers
            .get(attacker_id)
            .copied()
            .unwrap_or(false);

        let card = battlefield.card_mut(blocker_id).ok_or_else(|| {
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

        if attacker_has_flying && !card.has_flying() && !card.has_reach() {
            return Err(DomainError::Card(
                CardError::CannotBlockFlyingWithoutFlyingOrReach {
                    player: cmd.player_id.clone(),
                    blocker: blocker_id.clone(),
                    attacker: attacker_id.clone(),
                },
            ));
        }

        card.assign_blocking_target(attacker_id.clone());
        valid_blockers.push((blocker_id.clone(), attacker_id.clone()));
    }

    Ok(BlockersDeclared::new(
        game_id.clone(),
        cmd.player_id,
        valid_blockers,
    ))
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
    require_combat_damage_step(*phase)?;

    let player_idx = helpers::find_player_index(players, &cmd.player_id)?;
    let defender_idx = find_defending_player_index(players, &cmd.player_id)?;

    let defender_player_id = players[defender_idx].id().clone();
    let attackers = collect_attackers(&players[player_idx])?;
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
    let state_based_actions =
        state_based_actions::check_state_based_actions(game_id, players, terminal_state)?;

    Ok(ResolveCombatDamageOutcome::new(
        CombatDamageResolved::new(game_id.clone(), cmd.player_id, damage_events),
        player_life_change,
        state_based_actions.creatures_died,
        state_based_actions.game_ended,
    ))
}
