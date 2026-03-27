//! Supports player-local spell cast preparation.

use super::{
    CardInstanceId, ManaCost, Player, PlayerCardZone, PrepareHandSpellCastError,
    PreparedHandSpellCast,
};
use crate::domain::play::cards::CastingRule;

#[allow(clippy::missing_const_for_fn)]
impl Player {
    /// Prepares a spell cast atomically from the player's hand.
    ///
    /// # Errors
    /// Returns `MissingCard` if the card is not still in hand, or
    /// `InsufficientMana` if the player cannot currently pay the provided cost.
    pub fn prepare_hand_spell_cast(
        &mut self,
        card_id: &CardInstanceId,
        mana_cost: u32,
        mana_cost_profile: ManaCost,
    ) -> Result<PreparedHandSpellCast, PrepareHandSpellCastError> {
        let handle = self
            .handle_in_zone(card_id, PlayerCardZone::Hand)
            .ok_or(PrepareHandSpellCastError::MissingCard)?;
        let available = self.mana();
        let mut next_mana = self.mana.clone();
        if !next_mana.spend(mana_cost_profile) {
            return Err(PrepareHandSpellCastError::InsufficientMana { available });
        }

        let owned = self
            .cards
            .begin_remove_by_handle(handle)
            .ok_or(PrepareHandSpellCastError::MissingCard)?;
        if self.hand.remove(handle).is_none() {
            let _ = self.cards.rollback_remove(handle, owned);
            return Err(PrepareHandSpellCastError::MissingCard);
        }
        self.cards.commit_removed(handle, owned.card.id());
        let payload = owned.card.into_spell_payload();
        self.mana = next_mana;

        Ok(PreparedHandSpellCast {
            mana_cost_paid: mana_cost,
            payload,
        })
    }

    /// Prepares a spell cast atomically from the player's graveyard when an
    /// explicit casting permission allows it.
    ///
    /// # Errors
    /// Returns `MissingCard` if the card is not still in graveyard, or
    /// `InsufficientMana` if the player cannot currently pay the provided cost.
    pub fn prepare_graveyard_spell_cast(
        &mut self,
        card_id: &CardInstanceId,
        mana_cost: u32,
        mana_cost_profile: ManaCost,
    ) -> Result<PreparedHandSpellCast, PrepareHandSpellCastError> {
        let handle = self
            .handle_in_zone(card_id, PlayerCardZone::Graveyard)
            .ok_or(PrepareHandSpellCastError::MissingCard)?;
        let available = self.mana();
        let mut next_mana = self.mana.clone();
        if !next_mana.spend(mana_cost_profile) {
            return Err(PrepareHandSpellCastError::InsufficientMana { available });
        }

        let owned = self
            .cards
            .begin_remove_by_handle(handle)
            .ok_or(PrepareHandSpellCastError::MissingCard)?;
        if self.graveyard.remove(handle).is_none() {
            let _ = self.cards.rollback_remove(handle, owned);
            return Err(PrepareHandSpellCastError::MissingCard);
        }
        let exile_on_resolution =
            owned
                .card
                .casting_permission_profile()
                .is_some_and(|permission| {
                    permission.supports(CastingRule::ExileOnResolutionWhenCastFromOwnGraveyard)
                });
        self.cards.commit_removed(handle, owned.card.id());
        let mut payload = owned.card.into_spell_payload();
        if exile_on_resolution {
            payload.mark_exile_on_resolution();
        }
        self.mana = next_mana;

        Ok(PreparedHandSpellCast {
            mana_cost_paid: mana_cost,
            payload,
        })
    }
}
