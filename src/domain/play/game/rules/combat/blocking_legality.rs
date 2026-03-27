//! Supports rules combat blocking legality.

use {
    super::{super::super::model::Player, capabilities, progression},
    crate::domain::play::{
        cards::CardType,
        commands::DeclareBlockersCommand,
        errors::{CardError, DomainError, GameError},
        events::BlockersDeclared,
        ids::{CardInstanceId, GameId},
    },
    std::collections::{HashMap, HashSet},
};

#[derive(Debug, Clone, Copy)]
struct AttackerBlockingRequirements {
    requires_aerial_blocking: bool,
    has_menace: bool,
}

#[allow(clippy::too_many_lines)]
pub fn declare_blockers(
    game_id: &GameId,
    players: &mut [Player],
    active_player_index: usize,
    cmd: DeclareBlockersCommand,
) -> Result<BlockersDeclared, DomainError> {
    let defending_player_idx =
        progression::find_defending_player_index(players, active_player_index)?;
    let attacker_player_idx = active_player_index;
    let declared_attackers = players[attacker_player_idx]
        .battlefield_cards()
        .filter(|card| card.is_attacking())
        .map(|card| {
            (
                card.id().clone(),
                AttackerBlockingRequirements {
                    requires_aerial_blocking:
                        capabilities::attacker_requires_aerial_blocking_capability(card),
                    has_menace: card.has_menace(),
                },
            )
        })
        .collect::<HashMap<_, _>>();
    let mut blocker_count_by_attacker = HashMap::new();
    for (_, attacker_id) in &cmd.blocker_assignments {
        *blocker_count_by_attacker
            .entry(attacker_id.clone())
            .or_insert(0usize) += 1;
    }
    for (attacker_id, requirements) in &declared_attackers {
        if requirements.has_menace && blocker_count_by_attacker.get(attacker_id).copied() == Some(1)
        {
            return Err(DomainError::Card(
                CardError::CannotBlockMenaceWithSingleBlocker {
                    player: cmd.player_id,
                    attacker: attacker_id.clone(),
                },
            ));
        }
    }
    let mut valid_blockers: Vec<(CardInstanceId, CardInstanceId)> = Vec::new();
    let mut seen_blockers = HashSet::new();

    for (blocker_id, attacker_id) in &cmd.blocker_assignments {
        if !seen_blockers.insert(blocker_id.clone()) {
            return Err(DomainError::Game(GameError::DuplicateBlockerAssignment(
                blocker_id.clone(),
            )));
        }

        let Some(attacker_requirements) = declared_attackers.get(attacker_id) else {
            return Err(DomainError::Card(CardError::NotAttacking(
                attacker_id.clone(),
            )));
        };

        let attacker_owner_id = players[attacker_player_idx].id().clone();
        let attacker_handle = players[attacker_player_idx]
            .battlefield_handle(attacker_id)
            .ok_or_else(|| {
                DomainError::Card(CardError::NotOnBattlefield {
                    player: attacker_owner_id,
                    card: attacker_id.clone(),
                })
            })?;
        let blocker_handle = players[defending_player_idx]
            .battlefield_handle(blocker_id)
            .ok_or_else(|| {
                DomainError::Card(CardError::NotOnBattlefield {
                    player: cmd.player_id.clone(),
                    card: blocker_id.clone(),
                })
            })?;

        if attacker_player_idx < defending_player_idx {
            let (left, right) = players.split_at_mut(defending_player_idx);
            let attacker_player = &mut left[attacker_player_idx];
            let defending_player = &mut right[0];
            let card = defending_player
                .battlefield_card_mut(blocker_id)
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

            if card.cannot_block() {
                return Err(DomainError::Card(CardError::CannotBlock {
                    player: cmd.player_id.clone(),
                    card: blocker_id.clone(),
                }));
            }

            if !capabilities::can_block_attacker_with_aerial_requirement(
                card,
                attacker_requirements.requires_aerial_blocking,
            ) {
                return Err(DomainError::Card(
                    CardError::CannotBlockFlyingWithoutFlyingOrReach {
                        player: cmd.player_id.clone(),
                        blocker: blocker_id.clone(),
                        attacker: attacker_id.clone(),
                    },
                ));
            }

            card.assign_blocking_target(attacker_handle);
            attacker_player
                .card_mut_by_handle(attacker_handle)
                .ok_or_else(|| {
                    DomainError::Game(GameError::InternalInvariantViolation(
                        "declared blocker points to an attacker handle missing from battlefield"
                            .to_string(),
                    ))
                })?
                .add_blocker(blocker_handle);
        } else {
            let (left, right) = players.split_at_mut(attacker_player_idx);
            let defending_player = &mut left[defending_player_idx];
            let attacker_player = &mut right[0];
            let card = defending_player
                .battlefield_card_mut(blocker_id)
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

            if card.cannot_block() {
                return Err(DomainError::Card(CardError::CannotBlock {
                    player: cmd.player_id.clone(),
                    card: blocker_id.clone(),
                }));
            }

            if !capabilities::can_block_attacker_with_aerial_requirement(
                card,
                attacker_requirements.requires_aerial_blocking,
            ) {
                return Err(DomainError::Card(
                    CardError::CannotBlockFlyingWithoutFlyingOrReach {
                        player: cmd.player_id.clone(),
                        blocker: blocker_id.clone(),
                        attacker: attacker_id.clone(),
                    },
                ));
            }

            card.assign_blocking_target(attacker_handle);
            attacker_player
                .card_mut_by_handle(attacker_handle)
                .ok_or_else(|| {
                    DomainError::Game(GameError::InternalInvariantViolation(
                        "declared blocker points to an attacker handle missing from battlefield"
                            .to_string(),
                    ))
                })?
                .add_blocker(blocker_handle);
        }
        valid_blockers.push((blocker_id.clone(), attacker_id.clone()));
    }

    Ok(BlockersDeclared::new(
        game_id.clone(),
        cmd.player_id,
        valid_blockers,
    ))
}

#[must_use]
pub fn can_block_attacker_candidate(
    players: &[Player],
    active_player_index: usize,
    defending_player_id: &crate::domain::play::ids::PlayerId,
    blocker_id: &CardInstanceId,
    attacker_id: &CardInstanceId,
) -> bool {
    let Ok(defending_player_idx) =
        progression::find_defending_player_index(players, active_player_index)
    else {
        return false;
    };
    if players
        .get(defending_player_idx)
        .is_none_or(|player| player.id() != defending_player_id)
    {
        return false;
    }

    let attacker_player = &players[active_player_index];
    let Some(attacker) = attacker_player.battlefield_card(attacker_id) else {
        return false;
    };
    if !attacker.is_attacking() || !matches!(attacker.card_type(), CardType::Creature) {
        return false;
    }

    let defending_player = &players[defending_player_idx];
    let Some(blocker) = defending_player.battlefield_card(blocker_id) else {
        return false;
    };
    if !matches!(blocker.card_type(), CardType::Creature) {
        return false;
    }
    if blocker.is_tapped() || blocker.cannot_block() {
        return false;
    }

    capabilities::can_block_attacker_with_aerial_requirement(
        blocker,
        capabilities::attacker_requires_aerial_blocking_capability(attacker),
    )
}
