//! Supports play commands setup.

use crate::domain::play::{
    cards::{
        ActivatedAbilityProfile, CardDefinition, CardInstance, CardType, CastingRule,
        KeywordAbilitySet, ManaColor, ManaCost, SupportedSpellRules,
    },
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
    pub const fn land(definition_id: CardDefinitionId, produced_mana: ManaColor) -> Self {
        Self {
            definition: CardDefinition::land(definition_id, produced_mana),
            card_type: CardType::Land,
            creature: None,
        }
    }

    #[must_use]
    pub const fn new(definition_id: CardDefinitionId, card_type: CardType, mana_cost: u32) -> Self {
        Self {
            definition: CardDefinition::for_card_type(definition_id, mana_cost, &card_type),
            card_type,
            creature: None,
        }
    }

    #[must_use]
    pub fn with_supported_spell_rules(
        mut self,
        supported_spell_rules: SupportedSpellRules,
    ) -> Self {
        self.definition = self
            .definition
            .with_supported_spell_rules(supported_spell_rules);
        self
    }

    #[must_use]
    pub fn with_casting_rule(mut self, casting_rule: CastingRule) -> Self {
        self.definition = self.definition.with_casting_rule(casting_rule);
        self
    }

    #[must_use]
    pub fn with_mana_cost(mut self, mana_cost: ManaCost) -> Self {
        self.definition = self.definition.with_mana_cost(mana_cost);
        self
    }

    #[must_use]
    pub fn with_activated_ability(mut self, activated_ability: ActivatedAbilityProfile) -> Self {
        self.definition = self.definition.with_activated_ability(activated_ability);
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
            definition: CardDefinition::for_card_type(
                definition_id,
                mana_cost,
                &CardType::Creature,
            ),
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
            definition: CardDefinition::for_card_type(
                definition_id,
                mana_cost,
                &CardType::Creature,
            ),
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
    pub fn mana_cost(&self) -> u32 {
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
            None => CardInstance::from_definition(card_id, self.definition.clone(), self.card_type),
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
