use crate::domain::ids::{CardDefinitionId, CardInstanceId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CardType {
    Land,
    Creature,
    Instant,
    Sorcery,
    Enchantment,
    Artifact,
    Planeswalker,
}

impl CardType {
    pub const fn is_land(&self) -> bool {
        matches!(self, CardType::Land)
    }

    pub const fn is_non_land(&self) -> bool {
        !self.is_land()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardInstance {
    id: CardInstanceId,
    definition_id: CardDefinitionId,
    card_type: CardType,
    tapped: bool,
}

impl CardInstance {
    #[must_use]
    pub const fn new(
        id: CardInstanceId,
        definition_id: CardDefinitionId,
        card_type: CardType,
    ) -> Self {
        Self {
            id,
            definition_id,
            card_type,
            tapped: false,
        }
    }

    #[must_use]
    pub const fn id(&self) -> &CardInstanceId {
        &self.id
    }

    #[must_use]
    pub const fn definition_id(&self) -> &CardDefinitionId {
        &self.definition_id
    }

    #[must_use]
    pub const fn card_type(&self) -> &CardType {
        &self.card_type
    }

    #[must_use]
    pub const fn is_tapped(&self) -> bool {
        self.tapped
    }

    pub const fn tap(&mut self) {
        self.tapped = true;
    }

    pub const fn untap(&mut self) {
        self.tapped = false;
    }
}
