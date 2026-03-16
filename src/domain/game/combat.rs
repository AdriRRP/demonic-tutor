use super::player::Player;
use super::Phase;
use crate::domain::{
    cards::CardType,
    commands::{DeclareAttackersCommand, DeclareBlockersCommand, ResolveCombatDamageCommand},
    errors::{CardError, DomainError, GameError, PhaseError},
    events::{
        AttackersDeclared, BlockersDeclared, CombatDamageResolved, DamageEvent, DamageTarget,
    },
    ids::{CardInstanceId, PlayerId},
};

/// Declares attackers for the active player in combat.
///
/// # Errors
/// Returns an error if:
/// - The player is not the active player
/// - The phase is not Main
/// - Any creature is not on the battlefield
/// - Any creature is tapped
/// - Any creature has summoning sickness
pub fn declare_attackers(
    players: &mut [Player],
    active_player: &PlayerId,
    phase: &Phase,
    cmd: DeclareAttackersCommand,
) -> Result<AttackersDeclared, DomainError> {
    if *active_player != cmd.player_id {
        return Err(DomainError::Game(GameError::NotYourTurn {
            current: active_player.clone(),
            requested: cmd.player_id,
        }));
    }

    if !matches!(phase, Phase::Combat) {
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

/// Declares blockers for the defending player in combat.
///
/// # Errors
/// Returns an error if:
/// - The player is the active player (not the defending player)
/// - The phase is not Main
/// - Any blocker is not on the battlefield
/// - Any blocker is tapped
pub fn declare_blockers(
    players: &mut [Player],
    active_player: &PlayerId,
    phase: &Phase,
    cmd: DeclareBlockersCommand,
) -> Result<BlockersDeclared, DomainError> {
    if *active_player == cmd.player_id {
        return Err(DomainError::Phase(PhaseError::NotDefendingPlayer {
            current: active_player.clone(),
            requested: cmd.player_id,
        }));
    }

    if !matches!(phase, Phase::Combat) {
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

/// Resolves combat damage between attacking and blocking creatures.
///
/// # Errors
/// Returns an error if:
/// - The player is not the active player
/// - The phase is not Combat
pub fn resolve_combat_damage(
    players: &mut [Player],
    active_player: &PlayerId,
    phase: &Phase,
    cmd: ResolveCombatDamageCommand,
) -> Result<CombatDamageResolved, DomainError> {
    if *active_player != cmd.player_id {
        return Err(DomainError::Game(GameError::NotYourTurn {
            current: active_player.clone(),
            requested: cmd.player_id,
        }));
    }

    if !matches!(phase, Phase::Combat) {
        return Err(DomainError::Phase(PhaseError::InvalidForCombat));
    }

    let player_idx = players
        .iter()
        .position(|p| p.id() == &cmd.player_id)
        .ok_or_else(|| DomainError::Game(GameError::PlayerNotFound(cmd.player_id.clone())))?;

    let defender_idx = players
        .iter()
        .position(|p| p.id() != &cmd.player_id)
        .ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(
                "defending player should exist".to_string(),
            ))
        })?;

    let defender_player_id = players[defender_idx].id().clone();

    let attackers: Vec<_> = players[player_idx]
        .battlefield()
        .cards()
        .iter()
        .filter(|c| c.is_attacking())
        .map(|c| (c.id().clone(), c.power().unwrap_or(0)))
        .collect();

    let blockers: Vec<_> = players[defender_idx]
        .battlefield()
        .cards()
        .iter()
        .filter(|c| c.is_blocking())
        .map(|c| (c.id().clone(), c.power().unwrap_or(0)))
        .collect();

    let mut damage_events: Vec<DamageEvent> = Vec::new();

    for (attacker_id, power) in &attackers {
        let blocking: Vec<_> = blockers.iter().map(|(id, _)| id.clone()).collect();

        if blocking.is_empty() {
            let player = &mut players[defender_idx];
            *player.life_mut() = player.life().saturating_sub(*power);
            damage_events.push(DamageEvent {
                source: attacker_id.clone(),
                target: DamageTarget::Player(defender_player_id.clone()),
                damage_amount: *power,
            });
        } else {
            for blocker_id in &blocking {
                damage_events.push(DamageEvent {
                    source: attacker_id.clone(),
                    target: DamageTarget::Creature(blocker_id.clone()),
                    damage_amount: *power,
                });
            }
        }
    }

    for (blocker_id, power) in &blockers {
        if !attackers.is_empty() {
            for (attacker_id, _) in &attackers {
                damage_events.push(DamageEvent {
                    source: blocker_id.clone(),
                    target: DamageTarget::Creature(attacker_id.clone()),
                    damage_amount: *power,
                });
            }
        }
    }

    for player in players.iter_mut() {
        for card in player.battlefield_mut().cards_mut().iter_mut() {
            if card.is_attacking() || card.is_blocking() {
                let power = card.power().unwrap_or(0);
                card.add_damage(power);
            }
            card.untap();
            card.set_attacking(false);
            card.set_blocking(false);
        }
    }

    Ok(CombatDamageResolved::new(
        super::Game::id_from_player_id(&cmd.player_id),
        cmd.player_id,
        damage_events,
    ))
}
