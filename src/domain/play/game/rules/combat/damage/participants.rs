//! Supports combat damage participants.

use crate::domain::play::{
    cards::{KeywordAbility, KeywordAbilitySet},
    errors::{DomainError, GameError},
    game::model::Player,
    ids::PlayerCardHandle,
};

#[derive(Debug, Clone)]
pub(super) struct CombatCardRef {
    owner_index: usize,
    handle: PlayerCardHandle,
}

impl CombatCardRef {
    #[must_use]
    pub const fn new(owner_index: usize, handle: PlayerCardHandle) -> Self {
        Self {
            owner_index,
            handle,
        }
    }

    #[must_use]
    pub const fn owner_index(&self) -> usize {
        self.owner_index
    }

    #[must_use]
    pub const fn handle(&self) -> PlayerCardHandle {
        self.handle
    }
}

#[derive(Debug, Clone)]
struct CombatKeywordProfile {
    keywords: KeywordAbilitySet,
}

impl CombatKeywordProfile {
    #[must_use]
    const fn new(keywords: KeywordAbilitySet) -> Self {
        Self { keywords }
    }

    #[must_use]
    const fn has_first_strike(&self) -> bool {
        self.keywords.contains(KeywordAbility::FirstStrike)
    }

    #[must_use]
    const fn has_deathtouch(&self) -> bool {
        self.keywords.contains(KeywordAbility::Deathtouch)
    }

    #[must_use]
    const fn has_double_strike(&self) -> bool {
        self.keywords.contains(KeywordAbility::DoubleStrike)
    }

    #[must_use]
    const fn has_lifelink(&self) -> bool {
        self.keywords.contains(KeywordAbility::Lifelink)
    }
}

#[derive(Debug, Clone)]
pub(super) struct AttackerParticipant {
    card_ref: CombatCardRef,
    blocked_by_refs: Vec<CombatCardRef>,
    power: u32,
    has_trample: bool,
    keywords: CombatKeywordProfile,
}

impl AttackerParticipant {
    #[must_use]
    pub const fn card_ref(&self) -> &CombatCardRef {
        &self.card_ref
    }

    #[must_use]
    pub fn blocked_by_refs(&self) -> &[CombatCardRef] {
        &self.blocked_by_refs
    }

    #[must_use]
    pub const fn was_blocked(&self) -> bool {
        !self.blocked_by_refs.is_empty()
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
        self.keywords.has_first_strike()
    }

    #[must_use]
    pub const fn has_deathtouch(&self) -> bool {
        self.keywords.has_deathtouch()
    }

    #[must_use]
    pub const fn has_double_strike(&self) -> bool {
        self.keywords.has_double_strike()
    }

    #[must_use]
    pub const fn has_lifelink(&self) -> bool {
        self.keywords.has_lifelink()
    }
}

#[derive(Debug, Clone)]
pub(super) struct BlockerParticipant {
    card_ref: CombatCardRef,
    blocked_attacker_ref: CombatCardRef,
    power: u32,
    toughness: u32,
    marked_damage: u32,
    keywords: CombatKeywordProfile,
}

impl BlockerParticipant {
    #[must_use]
    pub const fn card_ref(&self) -> &CombatCardRef {
        &self.card_ref
    }

    #[must_use]
    pub const fn blocked_attacker_ref(&self) -> &CombatCardRef {
        &self.blocked_attacker_ref
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
        self.keywords.has_first_strike()
    }

    #[must_use]
    pub const fn has_deathtouch(&self) -> bool {
        self.keywords.has_deathtouch()
    }

    #[must_use]
    pub const fn has_double_strike(&self) -> bool {
        self.keywords.has_double_strike()
    }

    #[must_use]
    pub const fn has_lifelink(&self) -> bool {
        self.keywords.has_lifelink()
    }
}

pub(super) fn collect_attackers(
    player: &Player,
    owner_index: usize,
    blocker_owner_index: usize,
) -> Result<Vec<AttackerParticipant>, DomainError> {
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
            let attacker_handle = player.battlefield_handle(card.id()).ok_or_else(|| {
                DomainError::Game(GameError::InternalInvariantViolation(format!(
                    "attacking creature {} must have a battlefield handle",
                    card.id()
                )))
            })?;
            let blocked_by_refs = card
                .blocked_by()
                .iter()
                .copied()
                .map(|handle| CombatCardRef::new(blocker_owner_index, handle))
                .collect();

            Ok(AttackerParticipant {
                card_ref: CombatCardRef::new(owner_index, attacker_handle),
                blocked_by_refs,
                power,
                has_trample: card.has_trample(),
                keywords: CombatKeywordProfile::new(
                    card.keyword_abilities()
                        .unwrap_or_else(KeywordAbilitySet::empty),
                ),
            })
        })
        .collect()
}

pub(super) fn collect_blockers(
    player: &Player,
    owner_index: usize,
    attacker_player: &Player,
    attacker_owner_index: usize,
) -> Result<Vec<BlockerParticipant>, DomainError> {
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
            let attacker_handle = card.blocking_target().ok_or_else(|| {
                DomainError::Game(GameError::InternalInvariantViolation(format!(
                    "blocking creature {} must have an assigned attacker",
                    card.id()
                )))
            })?;
            attacker_player
                .card_by_handle(attacker_handle)
                .ok_or_else(|| {
                    DomainError::Game(GameError::InternalInvariantViolation(format!(
                        "blocking creature {} points to a missing attacker handle",
                        card.id()
                    )))
                })?;
            let blocker_handle = player.battlefield_handle(card.id()).ok_or_else(|| {
                DomainError::Game(GameError::InternalInvariantViolation(format!(
                    "blocking creature {} must have a battlefield handle",
                    card.id()
                )))
            })?;

            Ok(BlockerParticipant {
                card_ref: CombatCardRef::new(owner_index, blocker_handle),
                blocked_attacker_ref: CombatCardRef::new(attacker_owner_index, attacker_handle),
                power,
                toughness,
                marked_damage: card.damage(),
                keywords: CombatKeywordProfile::new(
                    card.keyword_abilities()
                        .unwrap_or_else(KeywordAbilitySet::empty),
                ),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::similar_names)]

    use {
        super::{collect_attackers, collect_blockers},
        crate::domain::play::{
            cards::CardInstance,
            game::model::Player,
            ids::{CardDefinitionId, CardInstanceId, PlayerId},
        },
    };

    #[test]
    fn collect_blockers_keeps_internal_attacker_handle_reference() {
        let attacker_id = CardInstanceId::new("attacker");
        let blocker_id = CardInstanceId::new("blocker");
        let mut attacker_player = Player::new(PlayerId::new("attacker-player"));
        let mut blocker_player = Player::new(PlayerId::new("blocker-player"));

        let mut attacker = CardInstance::new_creature(
            attacker_id.clone(),
            CardDefinitionId::new("attacker-definition"),
            2,
            3,
            3,
        );
        attacker.remove_summoning_sickness();
        attacker.set_attacking(true);
        attacker_player.receive_battlefield_card(attacker);

        let attacker_handle = attacker_player.battlefield_handle(&attacker_id);
        assert!(attacker_handle.is_some());
        let Some(attacker_handle) = attacker_handle else {
            return;
        };

        let mut blocker = CardInstance::new_creature(
            blocker_id,
            CardDefinitionId::new("blocker-definition"),
            2,
            2,
            2,
        );
        blocker.remove_summoning_sickness();
        blocker.assign_blocking_target(attacker_handle);
        blocker_player.receive_battlefield_card(blocker);

        let blockers = collect_blockers(&blocker_player, 1, &attacker_player, 0);
        assert!(blockers.is_ok());
        let Ok(blockers) = blockers else {
            return;
        };

        assert_eq!(blockers.len(), 1);
        assert_eq!(blockers[0].card_ref().owner_index(), 1);
        assert_eq!(blockers[0].blocked_attacker_ref().owner_index(), 0);
        assert_eq!(blockers[0].blocked_attacker_ref().handle(), attacker_handle);
    }

    #[test]
    fn collect_attackers_keeps_ordered_blocker_handle_references() {
        let attacker_id = CardInstanceId::new("attacker");
        let blocker_a_id = CardInstanceId::new("blocker-a");
        let blocker_b_id = CardInstanceId::new("blocker-b");
        let mut attacker_player = Player::new(PlayerId::new("attacker-player"));
        let mut blocker_player = Player::new(PlayerId::new("blocker-player"));

        let mut attacker = CardInstance::new_creature(
            attacker_id,
            CardDefinitionId::new("attacker-definition"),
            2,
            4,
            4,
        );
        attacker.remove_summoning_sickness();
        attacker.set_attacking(true);
        attacker_player.receive_battlefield_card(attacker);

        let blocker_a = CardInstance::new_creature(
            blocker_a_id.clone(),
            CardDefinitionId::new("blocker-a-definition"),
            2,
            2,
            2,
        );
        blocker_player.receive_battlefield_card(blocker_a);
        let blocker_b = CardInstance::new_creature(
            blocker_b_id.clone(),
            CardDefinitionId::new("blocker-b-definition"),
            2,
            2,
            2,
        );
        blocker_player.receive_battlefield_card(blocker_b);

        let blocker_a_handle = blocker_player
            .battlefield_handle(&blocker_a_id)
            .expect("blocker a should have battlefield handle");
        let blocker_b_handle = blocker_player
            .battlefield_handle(&blocker_b_id)
            .expect("blocker b should have battlefield handle");

        attacker_player
            .battlefield_card_mut(&CardInstanceId::new("attacker"))
            .expect("attacker should be on battlefield")
            .add_blocker(blocker_a_handle);
        attacker_player
            .battlefield_card_mut(&CardInstanceId::new("attacker"))
            .expect("attacker should be on battlefield")
            .add_blocker(blocker_b_handle);

        let attackers =
            collect_attackers(&attacker_player, 0, 1).expect("attackers should collect");
        assert_eq!(attackers.len(), 1);
        assert_eq!(attackers[0].blocked_by_refs()[0].owner_index(), 1);
        assert_eq!(attackers[0].blocked_by_refs()[0].handle(), blocker_a_handle);
        assert_eq!(attackers[0].blocked_by_refs()[1].handle(), blocker_b_handle);
        assert!(attackers[0].was_blocked());
    }
}
