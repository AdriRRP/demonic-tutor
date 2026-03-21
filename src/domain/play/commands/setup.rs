use crate::domain::play::{
    cards::{CardDefinition, CardInstance, CardType, KeywordAbilitySet, SpellEffectProfile},
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
    pub keyword_abilities: KeywordAbilitySet,
}

impl LibraryCreature {
    #[must_use]
    pub const fn new(power: u32, toughness: u32) -> Self {
        Self {
            power,
            toughness,
            keyword_abilities: KeywordAbilitySet::empty(),
        }
    }

    #[must_use]
    pub const fn with_keywords(
        power: u32,
        toughness: u32,
        keyword_abilities: KeywordAbilitySet,
    ) -> Self {
        Self {
            power,
            toughness,
            keyword_abilities,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LibraryCard {
    definition: CardDefinition,
    card_type: CardType,
    creature: Option<LibraryCreature>,
}

impl LibraryCard {
    #[must_use]
    pub const fn new(definition_id: CardDefinitionId, card_type: CardType, mana_cost: u32) -> Self {
        Self {
            definition: CardDefinition::new(definition_id, mana_cost),
            card_type,
            creature: None,
        }
    }

    #[must_use]
    pub fn with_spell_effect(mut self, spell_effect: SpellEffectProfile) -> Self {
        self.definition = self.definition.with_spell_effect(spell_effect);
        self
    }

    #[must_use]
    pub const fn creature(
        definition_id: CardDefinitionId,
        mana_cost: u32,
        power: u32,
        toughness: u32,
    ) -> Self {
        Self {
            definition: CardDefinition::new(definition_id, mana_cost),
            card_type: CardType::Creature,
            creature: Some(LibraryCreature::new(power, toughness)),
        }
    }

    #[must_use]
    pub const fn creature_with_keywords(
        definition_id: CardDefinitionId,
        mana_cost: u32,
        power: u32,
        toughness: u32,
        keyword_abilities: KeywordAbilitySet,
    ) -> Self {
        Self {
            definition: CardDefinition::new(definition_id, mana_cost),
            card_type: CardType::Creature,
            creature: Some(LibraryCreature::with_keywords(
                power,
                toughness,
                keyword_abilities,
            )),
        }
    }

    #[must_use]
    pub const fn definition_id(&self) -> &CardDefinitionId {
        self.definition.id()
    }

    #[must_use]
    pub const fn card_type(&self) -> &CardType {
        &self.card_type
    }

    #[must_use]
    pub const fn mana_cost(&self) -> u32 {
        self.definition.mana_cost()
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
                self.definition.clone(),
                creature.power,
                creature.toughness,
                creature.keyword_abilities,
            ),
            None => CardInstance::from_definition(
                card_id,
                self.definition.clone(),
                self.card_type.clone(),
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
