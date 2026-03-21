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
pub struct LibraryCreature {
    pub power: u32,
    pub toughness: u32,
    pub flying: bool,
    pub reach: bool,
}

impl LibraryCreature {
    #[must_use]
    pub const fn new(power: u32, toughness: u32) -> Self {
        Self {
            power,
            toughness,
            flying: false,
            reach: false,
        }
    }

    #[must_use]
    pub const fn with_keywords(power: u32, toughness: u32, flying: bool, reach: bool) -> Self {
        Self {
            power,
            toughness,
            flying,
            reach,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LibraryCard {
    definition_id: CardDefinitionId,
    card_type: CardType,
    mana_cost: u32,
    creature: Option<LibraryCreature>,
}

impl LibraryCard {
    #[must_use]
    pub const fn new(definition_id: CardDefinitionId, card_type: CardType, mana_cost: u32) -> Self {
        Self {
            definition_id,
            card_type,
            mana_cost,
            creature: None,
        }
    }

    #[must_use]
    pub const fn creature(
        definition_id: CardDefinitionId,
        mana_cost: u32,
        power: u32,
        toughness: u32,
    ) -> Self {
        Self {
            definition_id,
            card_type: CardType::Creature,
            mana_cost,
            creature: Some(LibraryCreature::new(power, toughness)),
        }
    }

    #[must_use]
    pub const fn creature_with_keywords(
        definition_id: CardDefinitionId,
        mana_cost: u32,
        power: u32,
        toughness: u32,
        flying: bool,
        reach: bool,
    ) -> Self {
        Self {
            definition_id,
            card_type: CardType::Creature,
            mana_cost,
            creature: Some(LibraryCreature::with_keywords(
                power, toughness, flying, reach,
            )),
        }
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
    pub const fn mana_cost(&self) -> u32 {
        self.mana_cost
    }

    #[must_use]
    pub const fn creature_profile(&self) -> Option<&LibraryCreature> {
        self.creature.as_ref()
    }

    #[must_use]
    pub fn to_card_instance(&self, card_id: CardInstanceId) -> CardInstance {
        match self.creature {
            Some(creature) => CardInstance::new_creature_with_keywords(
                card_id,
                self.definition_id.clone(),
                self.mana_cost,
                creature.power,
                creature.toughness,
                creature.flying,
                creature.reach,
            ),
            None => CardInstance::new(
                card_id,
                self.definition_id.clone(),
                self.card_type.clone(),
                self.mana_cost,
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
