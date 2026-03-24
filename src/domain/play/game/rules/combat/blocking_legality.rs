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
                capabilities::attacker_requires_aerial_blocking_capability(card),
            )
        })
        .collect::<HashMap<_, _>>();
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

        let Some(attacker_requires_aerial_blocking) = declared_attackers.get(attacker_id) else {
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

        let card = players[defending_player_idx]
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

        if !capabilities::can_block_attacker_with_aerial_requirement(
            card,
            *attacker_requires_aerial_blocking,
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
        valid_blockers.push((blocker_id.clone(), attacker_id.clone()));
    }

    Ok(BlockersDeclared::new(
        game_id.clone(),
        cmd.player_id,
        valid_blockers,
    ))
}
