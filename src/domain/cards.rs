use crate::domain::ids::{CardDefinitionId, CardInstanceId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CardType {
    Land,
    NonLand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardInstance {
    id: CardInstanceId,
    definition_id: CardDefinitionId,
    card_type: CardType,
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
}
