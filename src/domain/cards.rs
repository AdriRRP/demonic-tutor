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
    #[must_use]
    pub const fn is_land(&self) -> bool {
        matches!(self, Self::Land)
    }

    #[must_use]
    pub const fn is_non_land(&self) -> bool {
        !self.is_land()
    }

    #[must_use]
    pub const fn is_creature(&self) -> bool {
        matches!(self, Self::Creature)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardDefinition {
    id: CardDefinitionId,
    mana_cost: u32,
}

impl CardDefinition {
    #[must_use]
    pub const fn new(id: CardDefinitionId, mana_cost: u32) -> Self {
        Self { id, mana_cost }
    }

    #[must_use]
    pub const fn id(&self) -> &CardDefinitionId {
        &self.id
    }

    #[must_use]
    pub const fn mana_cost(&self) -> u32 {
        self.mana_cost
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardInstance {
    id: CardInstanceId,
    definition_id: CardDefinitionId,
    card_type: CardType,
    tapped: bool,
    mana_cost: u32,
    power: Option<u32>,
    toughness: Option<u32>,
    has_summoning_sickness: bool,
    is_attacking: bool,
}

impl CardInstance {
    #[must_use]
    pub const fn new(
        id: CardInstanceId,
        definition_id: CardDefinitionId,
        card_type: CardType,
        mana_cost: u32,
    ) -> Self {
        Self {
            id,
            definition_id,
            card_type,
            tapped: false,
            mana_cost,
            power: None,
            toughness: None,
            has_summoning_sickness: false,
            is_attacking: false,
        }
    }

    #[must_use]
    pub const fn new_creature(
        id: CardInstanceId,
        definition_id: CardDefinitionId,
        mana_cost: u32,
        power: u32,
        toughness: u32,
    ) -> Self {
        Self {
            id,
            definition_id,
            card_type: CardType::Creature,
            tapped: false,
            mana_cost,
            power: Some(power),
            toughness: Some(toughness),
            has_summoning_sickness: true,
            is_attacking: false,
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

    #[must_use]
    pub const fn mana_cost(&self) -> u32 {
        self.mana_cost
    }

    #[must_use]
    pub const fn power(&self) -> Option<u32> {
        self.power
    }

    #[must_use]
    pub const fn toughness(&self) -> Option<u32> {
        self.toughness
    }

    #[must_use]
    pub const fn has_summoning_sickness(&self) -> bool {
        self.has_summoning_sickness
    }

    #[must_use]
    pub const fn is_attacking(&self) -> bool {
        self.is_attacking
    }

    pub const fn tap(&mut self) {
        self.tapped = true;
    }

    pub const fn untap(&mut self) {
        self.tapped = false;
    }

    pub const fn remove_summoning_sickness(&mut self) {
        self.has_summoning_sickness = false;
    }

    pub const fn set_attacking(&mut self, attacking: bool) {
        self.is_attacking = attacking;
    }
}
