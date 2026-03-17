use crate::domain::play::ids::{CardDefinitionId, CardInstanceId};

const FLAG_TAPPED: u8 = 1 << 0;
const FLAG_SUMMONING_SICKNESS: u8 = 1 << 1;
const FLAG_ATTACKING: u8 = 1 << 2;
const FLAG_BLOCKING: u8 = 1 << 3;

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
    pub const fn is_spell_card(&self) -> bool {
        !self.is_land()
    }

    #[must_use]
    pub const fn is_creature(&self) -> bool {
        matches!(self, Self::Creature)
    }

    #[must_use]
    pub const fn is_instant(&self) -> bool {
        matches!(self, Self::Instant)
    }

    #[must_use]
    pub const fn is_permanent(&self) -> bool {
        matches!(
            self,
            Self::Land | Self::Creature | Self::Enchantment | Self::Artifact | Self::Planeswalker
        )
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
struct CreatureState {
    power: u32,
    toughness: u32,
    damage: u32,
    flags: u8,
    blocking_target: Option<CardInstanceId>,
}

impl CreatureState {
    const fn new(power: u32, toughness: u32) -> Self {
        Self {
            power,
            toughness,
            damage: 0,
            flags: FLAG_SUMMONING_SICKNESS,
            blocking_target: None,
        }
    }

    const fn has_flag(&self, flag: u8) -> bool {
        self.flags & flag != 0
    }

    const fn set_flag(&mut self, flag: u8, enabled: bool) {
        if enabled {
            self.flags |= flag;
        } else {
            self.flags &= !flag;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardInstance {
    id: CardInstanceId,
    definition_id: CardDefinitionId,
    card_type: CardType,
    mana_cost: u32,
    flags: u8,
    creature: Option<CreatureState>,
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
            mana_cost,
            flags: 0,
            creature: None,
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
            mana_cost,
            flags: 0,
            creature: Some(CreatureState::new(power, toughness)),
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
        self.flags & FLAG_TAPPED != 0
    }

    #[must_use]
    pub const fn mana_cost(&self) -> u32 {
        self.mana_cost
    }

    #[must_use]
    pub const fn power(&self) -> Option<u32> {
        match &self.creature {
            Some(creature) => Some(creature.power),
            None => None,
        }
    }

    #[must_use]
    pub const fn toughness(&self) -> Option<u32> {
        match &self.creature {
            Some(creature) => Some(creature.toughness),
            None => None,
        }
    }

    #[must_use]
    pub const fn creature_stats(&self) -> Option<(u32, u32)> {
        match (&self.card_type, &self.creature) {
            (CardType::Creature, Some(creature)) => Some((creature.power, creature.toughness)),
            _ => None,
        }
    }

    #[must_use]
    pub const fn has_summoning_sickness(&self) -> bool {
        match &self.creature {
            Some(creature) => creature.has_flag(FLAG_SUMMONING_SICKNESS),
            None => false,
        }
    }

    #[must_use]
    pub const fn is_attacking(&self) -> bool {
        match &self.creature {
            Some(creature) => creature.has_flag(FLAG_ATTACKING),
            None => false,
        }
    }

    #[must_use]
    pub const fn is_blocking(&self) -> bool {
        match &self.creature {
            Some(creature) => creature.has_flag(FLAG_BLOCKING),
            None => false,
        }
    }

    pub const fn tap(&mut self) {
        self.flags |= FLAG_TAPPED;
    }

    pub const fn untap(&mut self) {
        self.flags &= !FLAG_TAPPED;
    }

    pub const fn remove_summoning_sickness(&mut self) {
        if let Some(creature) = &mut self.creature {
            creature.set_flag(FLAG_SUMMONING_SICKNESS, false);
        }
    }

    pub const fn set_attacking(&mut self, attacking: bool) {
        if let Some(creature) = &mut self.creature {
            creature.set_flag(FLAG_ATTACKING, attacking);
        }
    }

    pub fn set_blocking(&mut self, blocking: bool) {
        if let Some(creature) = &mut self.creature {
            creature.set_flag(FLAG_BLOCKING, blocking);
            if !blocking {
                creature.blocking_target = None;
            }
        }
    }

    #[must_use]
    pub const fn blocking_target(&self) -> Option<&CardInstanceId> {
        match &self.creature {
            Some(creature) => creature.blocking_target.as_ref(),
            None => None,
        }
    }

    pub fn assign_blocking_target(&mut self, attacker_id: CardInstanceId) {
        if let Some(creature) = &mut self.creature {
            creature.set_flag(FLAG_BLOCKING, true);
            creature.blocking_target = Some(attacker_id);
        }
    }

    #[must_use]
    pub const fn damage(&self) -> u32 {
        match &self.creature {
            Some(creature) => creature.damage,
            None => 0,
        }
    }

    #[must_use]
    pub const fn has_lethal_damage(&self) -> bool {
        match &self.creature {
            Some(creature) => creature.damage >= creature.toughness,
            None => false,
        }
    }

    #[must_use]
    pub const fn has_zero_toughness(&self) -> bool {
        match &self.creature {
            Some(creature) => creature.toughness == 0,
            None => false,
        }
    }

    pub const fn add_damage(&mut self, damage: u32) {
        if let Some(creature) = &mut self.creature {
            creature.damage += damage;
        }
    }

    pub const fn clear_damage(&mut self) {
        if let Some(creature) = &mut self.creature {
            creature.damage = 0;
        }
    }
}
