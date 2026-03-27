//! Supports card instance constructors and runtime behavior.

use super::{
    ActivatedAbilityProfile, ActivatedManaAbilityProfile, Arc, AttachedCombatRestrictionProfile,
    AttachedStatBoostProfile, AttachmentProfile, CardDefinition, CardDefinitionId, CardFace,
    CardInstance, CardInstanceId, CardRuntime, CardRuntimeKind, CardType, CastingPermissionProfile,
    ControllerStaticEffectProfile, CreatureRuntime, KeywordAbility, KeywordAbilitySet, ManaCost,
    PlayerCardHandle, PlayerId, SupportedSpellRules, TriggeredAbilityProfile,
};

impl CardInstance {
    #[must_use]
    pub const fn id(&self) -> &CardInstanceId {
        &self.id
    }

    #[must_use]
    pub(crate) fn from_definition(
        id: CardInstanceId,
        definition: CardDefinition,
        _card_type: CardType,
    ) -> Self {
        let loyalty = definition.initial_loyalty().unwrap_or(0);
        Self {
            id,
            owner_id: None,
            face: CardFace {
                definition: Arc::new(definition),
            },
            runtime: CardRuntime {
                tapped: false,
                loyalty,
                loyalty_ability_activated_this_turn: false,
                is_token: false,
                attached_to: None,
                kind: CardRuntimeKind::NonCreature,
            },
        }
    }

    #[must_use]
    pub fn new(
        id: CardInstanceId,
        definition_id: CardDefinitionId,
        card_type: CardType,
        mana_cost: u32,
    ) -> Self {
        Self::from_definition(
            id,
            CardDefinition::for_card_type(definition_id, mana_cost, &card_type),
            card_type,
        )
    }

    #[must_use]
    pub fn new_creature(
        id: CardInstanceId,
        definition_id: CardDefinitionId,
        mana_cost: u32,
        power: u32,
        toughness: u32,
    ) -> Self {
        Self {
            id,
            owner_id: None,
            face: CardFace {
                definition: Arc::new(CardDefinition::for_card_type(
                    definition_id,
                    mana_cost,
                    &CardType::Creature,
                )),
            },
            runtime: CardRuntime {
                tapped: false,
                loyalty: 0,
                loyalty_ability_activated_this_turn: false,
                is_token: false,
                attached_to: None,
                kind: CardRuntimeKind::Creature(CreatureRuntime::new(power, toughness)),
            },
        }
    }

    #[must_use]
    pub fn new_creature_with_keywords(
        id: CardInstanceId,
        definition: CardDefinition,
        power: u32,
        toughness: u32,
        keywords: KeywordAbilitySet,
    ) -> Self {
        Self {
            id,
            owner_id: None,
            face: CardFace {
                definition: Arc::new(definition),
            },
            runtime: CardRuntime {
                tapped: false,
                loyalty: 0,
                loyalty_ability_activated_this_turn: false,
                is_token: false,
                attached_to: None,
                kind: CardRuntimeKind::Creature(CreatureRuntime::new_with_keywords(
                    power, toughness, keywords,
                )),
            },
        }
    }

    #[must_use]
    pub fn new_vanilla_creature_token(
        id: CardInstanceId,
        definition_id: CardDefinitionId,
        power: u32,
        toughness: u32,
    ) -> Self {
        Self {
            id,
            owner_id: None,
            face: CardFace {
                definition: Arc::new(CardDefinition::for_card_type(
                    definition_id,
                    0,
                    &CardType::Creature,
                )),
            },
            runtime: CardRuntime {
                tapped: false,
                loyalty: 0,
                loyalty_ability_activated_this_turn: false,
                is_token: true,
                attached_to: None,
                kind: CardRuntimeKind::Creature(CreatureRuntime::new(power, toughness)),
            },
        }
    }

    #[must_use]
    pub fn new_keyworded_creature_token(
        id: CardInstanceId,
        definition_id: CardDefinitionId,
        power: u32,
        toughness: u32,
        keywords: KeywordAbilitySet,
    ) -> Self {
        Self {
            id,
            owner_id: None,
            face: CardFace {
                definition: Arc::new(CardDefinition::for_card_type(
                    definition_id,
                    0,
                    &CardType::Creature,
                )),
            },
            runtime: CardRuntime {
                tapped: false,
                loyalty: 0,
                loyalty_ability_activated_this_turn: false,
                is_token: true,
                attached_to: None,
                kind: CardRuntimeKind::Creature(CreatureRuntime::new_with_keywords(
                    power, toughness, keywords,
                )),
            },
        }
    }

    pub(crate) fn ensure_owner(&mut self, owner_id: &PlayerId) {
        if self.owner_id.is_none() {
            self.owner_id = Some(owner_id.clone());
        }
    }

    #[must_use]
    pub const fn owner_id(&self) -> Option<&PlayerId> {
        self.owner_id.as_ref()
    }

    #[must_use]
    pub fn definition_id(&self) -> &CardDefinitionId {
        self.face.definition.id()
    }

    #[must_use]
    pub fn card_type(&self) -> &CardType {
        self.face.definition.card_type()
    }

    #[must_use]
    pub const fn is_tapped(&self) -> bool {
        self.runtime.tapped
    }

    #[must_use]
    pub fn loyalty(&self) -> Option<u32> {
        matches!(self.card_type(), CardType::Planeswalker).then_some(self.runtime.loyalty)
    }

    #[must_use]
    pub fn loyalty_ability_activated_this_turn(&self) -> bool {
        matches!(self.card_type(), CardType::Planeswalker)
            && self.runtime.loyalty_ability_activated_this_turn
    }

    #[must_use]
    pub const fn is_token(&self) -> bool {
        self.runtime.is_token
    }

    #[must_use]
    pub fn mana_cost(&self) -> u32 {
        self.face.definition.mana_cost()
    }

    #[must_use]
    pub fn mana_cost_profile(&self) -> ManaCost {
        self.face.definition.mana_cost_profile()
    }

    #[must_use]
    pub fn casting_permission_profile(&self) -> Option<CastingPermissionProfile> {
        self.face.definition.casting_permission()
    }

    #[must_use]
    pub fn supported_spell_rules(&self) -> SupportedSpellRules {
        self.face.definition.supported_spell_rules()
    }

    #[must_use]
    pub fn activated_ability(&self) -> Option<ActivatedAbilityProfile> {
        self.face.definition.activated_ability()
    }

    #[must_use]
    pub fn triggered_ability(&self) -> Option<TriggeredAbilityProfile> {
        self.face.definition.triggered_ability()
    }

    #[must_use]
    pub fn attachment_profile(&self) -> Option<AttachmentProfile> {
        self.face.definition.attachment_profile()
    }

    #[must_use]
    pub fn attached_stat_boost(&self) -> Option<AttachedStatBoostProfile> {
        self.face.definition.attached_stat_boost()
    }

    #[must_use]
    pub fn attached_combat_restriction(&self) -> Option<AttachedCombatRestrictionProfile> {
        self.face.definition.attached_combat_restriction()
    }

    #[must_use]
    pub fn controller_static_effect(&self) -> Option<ControllerStaticEffectProfile> {
        self.face.definition.controller_static_effect()
    }

    #[must_use]
    pub const fn attached_to(&self) -> Option<&CardInstanceId> {
        self.runtime.attached_to.as_ref()
    }

    #[must_use]
    pub fn activated_mana_ability(&self) -> Option<ActivatedManaAbilityProfile> {
        self.face.definition.activated_mana_ability()
    }

    #[must_use]
    pub const fn power(&self) -> Option<u32> {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => Some(
                creature.power
                    + creature.plus_one_plus_one_counters
                    + creature.temporary_power
                    + creature.attached_power_bonus
                    + creature.controller_static_power_bonus,
            ),
            CardRuntimeKind::NonCreature => None,
        }
    }

    #[must_use]
    pub const fn toughness(&self) -> Option<u32> {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => Some(
                creature.toughness
                    + creature.plus_one_plus_one_counters
                    + creature.temporary_toughness
                    + creature.attached_toughness_bonus
                    + creature.controller_static_toughness_bonus,
            ),
            CardRuntimeKind::NonCreature => None,
        }
    }

    #[must_use]
    pub const fn creature_stats(&self) -> Option<(u32, u32)> {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => Some((
                creature.power
                    + creature.plus_one_plus_one_counters
                    + creature.temporary_power
                    + creature.attached_power_bonus
                    + creature.controller_static_power_bonus,
                creature.toughness
                    + creature.plus_one_plus_one_counters
                    + creature.temporary_toughness
                    + creature.attached_toughness_bonus
                    + creature.controller_static_toughness_bonus,
            )),
            CardRuntimeKind::NonCreature => None,
        }
    }

    #[must_use]
    pub const fn has_summoning_sickness(&self) -> bool {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => creature.has_summoning_sickness(),
            CardRuntimeKind::NonCreature => false,
        }
    }

    #[must_use]
    pub const fn is_attacking(&self) -> bool {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => creature.is_attacking(),
            CardRuntimeKind::NonCreature => false,
        }
    }

    #[must_use]
    pub const fn is_blocking(&self) -> bool {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => creature.is_blocking(),
            CardRuntimeKind::NonCreature => false,
        }
    }

    pub const fn tap(&mut self) {
        self.runtime.tapped = true;
    }

    pub const fn untap(&mut self) {
        self.runtime.tapped = false;
    }

    pub fn adjust_loyalty(&mut self, delta: i32) -> bool {
        if !matches!(self.card_type(), CardType::Planeswalker) {
            return false;
        }
        if delta.is_negative() {
            let amount = delta.unsigned_abs();
            if self.runtime.loyalty < amount {
                return false;
            }
            self.runtime.loyalty -= amount;
        } else {
            self.runtime.loyalty = self.runtime.loyalty.saturating_add(delta.cast_unsigned());
        }
        true
    }

    pub fn mark_loyalty_ability_activated(&mut self) -> bool {
        if !matches!(self.card_type(), CardType::Planeswalker) {
            return false;
        }
        if self.runtime.loyalty_ability_activated_this_turn {
            return false;
        }
        self.runtime.loyalty_ability_activated_this_turn = true;
        true
    }

    pub fn reset_loyalty_activation_for_new_turn(&mut self) {
        if matches!(self.card_type(), CardType::Planeswalker) {
            self.runtime.loyalty_ability_activated_this_turn = false;
        }
    }

    pub fn attach_to(&mut self, target_id: CardInstanceId) {
        self.runtime.attached_to = Some(target_id);
    }

    pub fn clear_attachment(&mut self) {
        self.runtime.attached_to = None;
    }

    pub const fn remove_summoning_sickness(&mut self) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.remove_summoning_sickness();
        }
    }

    pub fn set_attacking(&mut self, attacking: bool) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.set_attacking(attacking);
        }
    }

    pub fn set_blocking(&mut self, blocking: bool) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.set_blocking(blocking);
        }
    }

    #[must_use]
    pub const fn blocking_target(&self) -> Option<PlayerCardHandle> {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => creature.blocking_target,
            CardRuntimeKind::NonCreature => None,
        }
    }

    pub fn assign_blocking_target(&mut self, attacker_handle: PlayerCardHandle) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.set_blocking(true);
            creature.blocking_target = Some(attacker_handle);
        }
    }

    pub fn add_blocker(&mut self, blocker_handle: PlayerCardHandle) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            if !creature.blocked_by.contains(&blocker_handle) {
                creature.blocked_by.push(blocker_handle);
            }
        }
    }

    #[must_use]
    pub const fn blocked_by(&self) -> &[PlayerCardHandle] {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => creature.blocked_by.as_slice(),
            CardRuntimeKind::NonCreature => &[],
        }
    }

    #[must_use]
    pub const fn damage(&self) -> u32 {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => creature.damage,
            CardRuntimeKind::NonCreature => 0,
        }
    }

    #[must_use]
    pub const fn has_lethal_damage(&self) -> bool {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => {
                creature.damage
                    >= creature.toughness
                        + creature.plus_one_plus_one_counters
                        + creature.temporary_toughness
                        + creature.attached_toughness_bonus
                        + creature.controller_static_toughness_bonus
                    || (creature.deathtouch_damage && creature.damage > 0)
            }
            CardRuntimeKind::NonCreature => false,
        }
    }

    #[must_use]
    pub const fn has_zero_toughness(&self) -> bool {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => {
                creature.toughness
                    + creature.plus_one_plus_one_counters
                    + creature.temporary_toughness
                    + creature.attached_toughness_bonus
                    + creature.controller_static_toughness_bonus
                    == 0
            }
            CardRuntimeKind::NonCreature => false,
        }
    }

    pub const fn add_plus_one_plus_one_counters(&mut self, amount: u32) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.plus_one_plus_one_counters =
                creature.plus_one_plus_one_counters.saturating_add(amount);
        }
    }

    pub const fn add_damage(&mut self, damage: u32) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.damage += damage;
        }
    }

    pub const fn add_deathtouch_damage(&mut self, damage: u32) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.damage += damage;
            if damage > 0 {
                creature.deathtouch_damage = true;
            }
        }
    }

    pub const fn clear_damage(&mut self) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.damage = 0;
            creature.deathtouch_damage = false;
        }
    }

    pub const fn apply_temporary_stat_bonus(&mut self, power: u32, toughness: u32) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.temporary_power += power;
            creature.temporary_toughness += toughness;
        }
    }

    pub const fn add_attached_stat_bonus(&mut self, power: u32, toughness: u32) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.attached_power_bonus = creature.attached_power_bonus.saturating_add(power);
            creature.attached_toughness_bonus =
                creature.attached_toughness_bonus.saturating_add(toughness);
        }
    }

    pub const fn remove_attached_stat_bonus(&mut self, power: u32, toughness: u32) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.attached_power_bonus = creature.attached_power_bonus.saturating_sub(power);
            creature.attached_toughness_bonus =
                creature.attached_toughness_bonus.saturating_sub(toughness);
        }
    }

    pub const fn add_controller_static_stat_bonus(&mut self, power: u32, toughness: u32) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.controller_static_power_bonus =
                creature.controller_static_power_bonus.saturating_add(power);
            creature.controller_static_toughness_bonus = creature
                .controller_static_toughness_bonus
                .saturating_add(toughness);
        }
    }

    pub const fn remove_controller_static_stat_bonus(&mut self, power: u32, toughness: u32) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.controller_static_power_bonus =
                creature.controller_static_power_bonus.saturating_sub(power);
            creature.controller_static_toughness_bonus = creature
                .controller_static_toughness_bonus
                .saturating_sub(toughness);
        }
    }

    pub const fn add_attached_cant_attack_or_block(&mut self) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.attached_cant_attack_count =
                creature.attached_cant_attack_count.saturating_add(1);
            creature.attached_cant_block_count =
                creature.attached_cant_block_count.saturating_add(1);
        }
    }

    pub const fn remove_attached_cant_attack_or_block(&mut self) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.attached_cant_attack_count =
                creature.attached_cant_attack_count.saturating_sub(1);
            creature.attached_cant_block_count =
                creature.attached_cant_block_count.saturating_sub(1);
        }
    }

    #[must_use]
    pub const fn cannot_attack(&self) -> bool {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => {
                creature.attached_cant_attack_count > 0
                    || creature.keywords.contains(KeywordAbility::Defender)
            }
            CardRuntimeKind::NonCreature => false,
        }
    }

    #[must_use]
    pub const fn cannot_block(&self) -> bool {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => {
                creature.attached_cant_block_count > 0 || creature.temporary_cant_block_count > 0
            }
            CardRuntimeKind::NonCreature => false,
        }
    }

    pub const fn add_temporary_cant_block(&mut self) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.temporary_cant_block_count =
                creature.temporary_cant_block_count.saturating_add(1);
        }
    }

    pub const fn clear_temporary_stat_bonuses(&mut self) {
        if let CardRuntimeKind::Creature(creature) = &mut self.runtime.kind {
            creature.temporary_power = 0;
            creature.temporary_toughness = 0;
            creature.temporary_cant_block_count = 0;
        }
    }

    #[must_use]
    pub const fn has_keyword(&self, ability: KeywordAbility) -> bool {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => creature.keywords.contains(ability),
            CardRuntimeKind::NonCreature => false,
        }
    }

    #[must_use]
    pub const fn has_flying(&self) -> bool {
        self.has_keyword(KeywordAbility::Flying)
    }

    #[must_use]
    pub const fn has_reach(&self) -> bool {
        self.has_keyword(KeywordAbility::Reach)
    }

    #[must_use]
    pub const fn has_haste(&self) -> bool {
        self.has_keyword(KeywordAbility::Haste)
    }

    #[must_use]
    pub const fn has_vigilance(&self) -> bool {
        self.has_keyword(KeywordAbility::Vigilance)
    }

    #[must_use]
    pub const fn has_trample(&self) -> bool {
        self.has_keyword(KeywordAbility::Trample)
    }

    #[must_use]
    pub const fn has_first_strike(&self) -> bool {
        self.has_keyword(KeywordAbility::FirstStrike)
    }

    #[must_use]
    pub const fn has_deathtouch(&self) -> bool {
        self.has_keyword(KeywordAbility::Deathtouch)
    }

    #[must_use]
    pub const fn has_double_strike(&self) -> bool {
        self.has_keyword(KeywordAbility::DoubleStrike)
    }

    #[must_use]
    pub const fn has_lifelink(&self) -> bool {
        self.has_keyword(KeywordAbility::Lifelink)
    }

    #[must_use]
    pub const fn has_menace(&self) -> bool {
        self.has_keyword(KeywordAbility::Menace)
    }

    #[must_use]
    pub const fn has_hexproof(&self) -> bool {
        self.has_keyword(KeywordAbility::Hexproof)
    }

    #[must_use]
    pub const fn has_indestructible(&self) -> bool {
        self.has_keyword(KeywordAbility::Indestructible)
    }

    #[must_use]
    pub const fn has_defender(&self) -> bool {
        self.has_keyword(KeywordAbility::Defender)
    }

    #[must_use]
    pub const fn keyword_abilities(&self) -> Option<KeywordAbilitySet> {
        match &self.runtime.kind {
            CardRuntimeKind::Creature(creature) => Some(creature.keywords),
            CardRuntimeKind::NonCreature => None,
        }
    }
}
