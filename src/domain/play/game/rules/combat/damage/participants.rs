use crate::domain::play::{
    errors::{DomainError, GameError},
    game::model::Player,
    ids::CardInstanceId,
};

pub(super) fn collect_attackers(
    player: &Player,
) -> Result<Vec<(CardInstanceId, u32)>, DomainError> {
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

            Ok((card.id().clone(), power))
        })
        .collect()
}

pub(super) fn collect_blockers(
    player: &Player,
) -> Result<Vec<(CardInstanceId, CardInstanceId, u32)>, DomainError> {
    player
        .battlefield_cards()
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
