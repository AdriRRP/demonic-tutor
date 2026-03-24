use crate::domain::play::{
    errors::{DomainError, GameError},
    game::model::Player,
    ids::CardInstanceId,
};

#[derive(Debug, Clone)]
pub(super) struct AttackerParticipant {
    id: CardInstanceId,
    power: u32,
    has_trample: bool,
    has_first_strike: bool,
}

impl AttackerParticipant {
    #[must_use]
    pub const fn id(&self) -> &CardInstanceId {
        &self.id
    }

    #[must_use]
    pub const fn power(&self) -> u32 {
        self.power
    }

    #[must_use]
    pub const fn has_trample(&self) -> bool {
        self.has_trample
    }

    #[must_use]
    pub const fn has_first_strike(&self) -> bool {
        self.has_first_strike
    }
}

#[derive(Debug, Clone)]
pub(super) struct BlockerParticipant {
    id: CardInstanceId,
    blocked_attacker_id: CardInstanceId,
    power: u32,
    toughness: u32,
    marked_damage: u32,
    has_first_strike: bool,
}

impl BlockerParticipant {
    #[must_use]
    pub const fn id(&self) -> &CardInstanceId {
        &self.id
    }

    #[must_use]
    pub const fn blocked_attacker_id(&self) -> &CardInstanceId {
        &self.blocked_attacker_id
    }

    #[must_use]
    pub const fn power(&self) -> u32 {
        self.power
    }

    #[must_use]
    pub const fn lethal_damage_threshold(&self) -> u32 {
        self.toughness.saturating_sub(self.marked_damage)
    }

    #[must_use]
    pub const fn has_first_strike(&self) -> bool {
        self.has_first_strike
    }
}

pub(super) fn collect_attackers(player: &Player) -> Result<Vec<AttackerParticipant>, DomainError> {
    player
        .battlefield_cards()
        .filter(|card| card.is_attacking())
        .map(|card| {
            let (power, _) = card.creature_stats().ok_or_else(|| {
                DomainError::Game(GameError::InternalInvariantViolation(format!(
                    "attacking creature {} must have power and toughness",
                    card.id()
                )))
            })?;

            Ok(AttackerParticipant {
                id: card.id().clone(),
                power,
                has_trample: card.has_trample(),
                has_first_strike: card.has_first_strike(),
            })
        })
        .collect()
}

pub(super) fn collect_blockers(player: &Player) -> Result<Vec<BlockerParticipant>, DomainError> {
    player
        .battlefield_cards()
        .filter(|card| card.is_blocking())
        .map(|card| {
            let (power, toughness) = card.creature_stats().ok_or_else(|| {
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

            Ok(BlockerParticipant {
                id: card.id().clone(),
                blocked_attacker_id: attacker_id,
                power,
                toughness,
                marked_damage: card.damage(),
                has_first_strike: card.has_first_strike(),
            })
        })
        .collect()
}
