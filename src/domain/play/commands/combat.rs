//! Supports play commands combat.

use crate::domain::play::ids::{CardInstanceId, PlayerId};

#[derive(Debug, Clone)]
pub struct DeclareAttackersCommand {
    pub player_id: PlayerId,
    pub attacker_ids: Vec<CardInstanceId>,
}

impl DeclareAttackersCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId, attacker_ids: Vec<CardInstanceId>) -> Self {
        Self {
            player_id,
            attacker_ids,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeclareBlockersCommand {
    pub player_id: PlayerId,
    pub blocker_assignments: Vec<(CardInstanceId, CardInstanceId)>,
}

impl DeclareBlockersCommand {
    #[must_use]
    pub const fn new(
        player_id: PlayerId,
        blocker_assignments: Vec<(CardInstanceId, CardInstanceId)>,
    ) -> Self {
        Self {
            player_id,
            blocker_assignments,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResolveCombatDamageCommand {
    pub player_id: PlayerId,
}

impl ResolveCombatDamageCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId) -> Self {
        Self { player_id }
    }
}
