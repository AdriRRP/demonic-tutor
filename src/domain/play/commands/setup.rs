use crate::domain::play::{
    cards::{CardInstance, CardType},
    ids::{CardDefinitionId, CardInstanceId, DeckId, PlayerId},
};

#[derive(Debug, Clone)]
pub struct PlayerDeck {
    pub player_id: PlayerId,
    pub deck_id: DeckId,
}

impl PlayerDeck {
    #[must_use]
    pub const fn new(player_id: PlayerId, deck_id: DeckId) -> Self {
        Self { player_id, deck_id }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NonCreatureCardType {
    Land,
    Instant,
    Sorcery,
    Enchantment,
    Artifact,
    Planeswalker,
}

impl NonCreatureCardType {
    #[must_use]
    pub const fn to_card_type(self) -> CardType {
        match self {
            Self::Land => CardType::Land,
            Self::Instant => CardType::Instant,
            Self::Sorcery => CardType::Sorcery,
            Self::Enchantment => CardType::Enchantment,
            Self::Artifact => CardType::Artifact,
            Self::Planeswalker => CardType::Planeswalker,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LibraryCard {
    NonCreature {
        definition_id: CardDefinitionId,
        card_type: NonCreatureCardType,
        mana_cost: u32,
    },
    Creature {
        definition_id: CardDefinitionId,
        mana_cost: u32,
        power: u32,
        toughness: u32,
    },
}

impl LibraryCard {
    #[must_use]
    pub const fn non_creature(
        definition_id: CardDefinitionId,
        card_type: NonCreatureCardType,
        mana_cost: u32,
    ) -> Self {
        Self::NonCreature {
            definition_id,
            card_type,
            mana_cost,
        }
    }

    #[must_use]
    pub const fn creature(
        definition_id: CardDefinitionId,
        mana_cost: u32,
        power: u32,
        toughness: u32,
    ) -> Self {
        Self::Creature {
            definition_id,
            mana_cost,
            power,
            toughness,
        }
    }

    #[must_use]
    pub fn to_card_instance(&self, card_id: CardInstanceId) -> CardInstance {
        match self {
            Self::Creature {
                definition_id,
                mana_cost,
                power,
                toughness,
            } => CardInstance::new_creature(
                card_id,
                definition_id.clone(),
                *mana_cost,
                *power,
                *toughness,
            ),
            Self::NonCreature {
                definition_id,
                card_type,
                mana_cost,
            } => CardInstance::new(
                card_id,
                definition_id.clone(),
                card_type.to_card_type(),
                *mana_cost,
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerLibrary {
    pub player_id: PlayerId,
    pub cards: Vec<LibraryCard>,
}

impl PlayerLibrary {
    #[must_use]
    pub const fn new(player_id: PlayerId, cards: Vec<LibraryCard>) -> Self {
        Self { player_id, cards }
    }
}
