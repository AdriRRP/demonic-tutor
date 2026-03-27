//! Supports focused spell targeting and resolution profiles.

use super::{SingleTargetRule, SpellTargetKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellTargetingProfile {
    None,
    ExactlyOne(SingleTargetRule),
}

impl SpellTargetingProfile {
    #[must_use]
    pub const fn requires_target(&self) -> bool {
        !matches!(self, Self::None)
    }

    #[must_use]
    pub const fn allows_target_kind(self, kind: SpellTargetKind) -> bool {
        match self {
            Self::None => false,
            Self::ExactlyOne(rule) => rule.matches_target_kind(kind),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellResolutionProfile {
    None,
    AttachToTargetCreature,
    DealDamage { damage: u32 },
    GainLife { amount: u32 },
    LoseLife { amount: u32 },
    ChooseOneTargetPlayerGainOrLoseLife { gain_amount: u32, lose_amount: u32 },
    Scry { amount: u32 },
    LootDrawThenDiscard { draw_count: u32 },
    RummageDiscardThenDraw { draw_count: u32 },
    CreateVanillaCreatureToken { power: u32, toughness: u32 },
    PutPlusOnePlusOneCounterOnTargetCreature,
    ReturnTargetCreatureCardFromGraveyardToHand,
    ReanimateTargetCreatureCard,
    MillCards { amount: u32 },
    CounterTargetSpell,
    ReturnTargetPermanentToHand,
    DestroyTargetArtifactOrEnchantment,
    TargetPlayerDiscardsChosenCard,
    DestroyTargetCreature,
    ExileTargetCreature,
    ExileTargetCardFromGraveyard,
    PumpTargetCreatureUntilEndOfTurn { power: u32, toughness: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SupportedSpellRules {
    targeting: SpellTargetingProfile,
    resolution: SpellResolutionProfile,
}

impl SupportedSpellRules {
    #[must_use]
    pub const fn none() -> Self {
        Self {
            targeting: SpellTargetingProfile::None,
            resolution: SpellResolutionProfile::None,
        }
    }

    #[must_use]
    pub const fn deal_damage_to_any_target(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::any_player_or_creature_on_battlefield(),
            ),
            resolution: SpellResolutionProfile::DealDamage { damage },
        }
    }

    #[must_use]
    pub const fn attach_to_target_creature() -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::any_creature_on_battlefield(),
            ),
            resolution: SpellResolutionProfile::AttachToTargetCreature,
        }
    }

    #[must_use]
    pub const fn deal_damage_to_player(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(SingleTargetRule::any_player()),
            resolution: SpellResolutionProfile::DealDamage { damage },
        }
    }

    #[must_use]
    pub const fn gain_life_to_player(amount: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(SingleTargetRule::any_player()),
            resolution: SpellResolutionProfile::GainLife { amount },
        }
    }

    #[must_use]
    pub const fn lose_life_from_player(amount: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(SingleTargetRule::any_player()),
            resolution: SpellResolutionProfile::LoseLife { amount },
        }
    }

    #[must_use]
    pub const fn choose_one_target_player_gain_or_lose_life(
        gain_amount: u32,
        lose_amount: u32,
    ) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(SingleTargetRule::any_player()),
            resolution: SpellResolutionProfile::ChooseOneTargetPlayerGainOrLoseLife {
                gain_amount,
                lose_amount,
            },
        }
    }

    #[must_use]
    pub const fn scry(amount: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::None,
            resolution: SpellResolutionProfile::Scry { amount },
        }
    }

    #[must_use]
    pub const fn loot_draw_then_discard(draw_count: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::None,
            resolution: SpellResolutionProfile::LootDrawThenDiscard { draw_count },
        }
    }

    #[must_use]
    pub const fn rummage_discard_then_draw(draw_count: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::None,
            resolution: SpellResolutionProfile::RummageDiscardThenDraw { draw_count },
        }
    }

    #[must_use]
    pub const fn create_vanilla_creature_token(power: u32, toughness: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::None,
            resolution: SpellResolutionProfile::CreateVanillaCreatureToken { power, toughness },
        }
    }

    #[must_use]
    pub const fn put_plus_one_plus_one_counter_on_target_creature() -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::any_creature_on_battlefield(),
            ),
            resolution: SpellResolutionProfile::PutPlusOnePlusOneCounterOnTargetCreature,
        }
    }

    #[must_use]
    pub const fn return_target_creature_card_from_graveyard_to_hand() -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::any_card_in_a_graveyard(),
            ),
            resolution: SpellResolutionProfile::ReturnTargetCreatureCardFromGraveyardToHand,
        }
    }

    #[must_use]
    pub const fn reanimate_target_creature_card() -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::card_in_actors_graveyard(),
            ),
            resolution: SpellResolutionProfile::ReanimateTargetCreatureCard,
        }
    }

    #[must_use]
    pub const fn mill_target_player(amount: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(SingleTargetRule::any_player()),
            resolution: SpellResolutionProfile::MillCards { amount },
        }
    }

    #[must_use]
    pub const fn mill_self(amount: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::None,
            resolution: SpellResolutionProfile::MillCards { amount },
        }
    }

    #[must_use]
    pub const fn deal_damage_to_opponent(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(SingleTargetRule::opponent_of_actor()),
            resolution: SpellResolutionProfile::DealDamage { damage },
        }
    }

    #[must_use]
    pub const fn deal_damage_to_controlled_creature(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::creature_controlled_by_actor(),
            ),
            resolution: SpellResolutionProfile::DealDamage { damage },
        }
    }

    #[must_use]
    pub const fn deal_damage_to_opponents_creature(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(SingleTargetRule::opponents_creature()),
            resolution: SpellResolutionProfile::DealDamage { damage },
        }
    }

    #[must_use]
    pub const fn deal_damage_to_attacking_creature(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(SingleTargetRule::attacking_creature()),
            resolution: SpellResolutionProfile::DealDamage { damage },
        }
    }

    #[must_use]
    pub const fn deal_damage_to_blocking_creature(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(SingleTargetRule::blocking_creature()),
            resolution: SpellResolutionProfile::DealDamage { damage },
        }
    }

    #[must_use]
    pub const fn deal_damage_to_controlled_blocking_creature(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::controlled_blocking_creature(),
            ),
            resolution: SpellResolutionProfile::DealDamage { damage },
        }
    }

    #[must_use]
    pub const fn deal_damage_to_controlled_attacking_creature(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::controlled_attacking_creature(),
            ),
            resolution: SpellResolutionProfile::DealDamage { damage },
        }
    }

    #[must_use]
    pub const fn deal_damage_to_opponents_blocking_creature(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::opponents_blocking_creature(),
            ),
            resolution: SpellResolutionProfile::DealDamage { damage },
        }
    }

    #[must_use]
    pub const fn deal_damage_to_opponents_attacking_creature(damage: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::opponents_attacking_creature(),
            ),
            resolution: SpellResolutionProfile::DealDamage { damage },
        }
    }

    #[must_use]
    pub const fn destroy_target_creature() -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::any_creature_on_battlefield(),
            ),
            resolution: SpellResolutionProfile::DestroyTargetCreature,
        }
    }

    #[must_use]
    pub const fn counter_target_spell() -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(SingleTargetRule::any_spell_on_the_stack()),
            resolution: SpellResolutionProfile::CounterTargetSpell,
        }
    }

    #[must_use]
    pub const fn return_target_permanent_to_hand() -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::any_permanent_on_battlefield(),
            ),
            resolution: SpellResolutionProfile::ReturnTargetPermanentToHand,
        }
    }

    #[must_use]
    pub const fn destroy_target_artifact_or_enchantment() -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::artifact_or_enchantment_on_battlefield(),
            ),
            resolution: SpellResolutionProfile::DestroyTargetArtifactOrEnchantment,
        }
    }

    #[must_use]
    pub const fn target_player_discards_chosen_card() -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(SingleTargetRule::any_player()),
            resolution: SpellResolutionProfile::TargetPlayerDiscardsChosenCard,
        }
    }

    #[must_use]
    pub const fn exile_target_creature() -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::any_creature_on_battlefield(),
            ),
            resolution: SpellResolutionProfile::ExileTargetCreature,
        }
    }

    #[must_use]
    pub const fn exile_target_card_from_graveyard() -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::any_card_in_a_graveyard(),
            ),
            resolution: SpellResolutionProfile::ExileTargetCardFromGraveyard,
        }
    }

    #[must_use]
    pub const fn pump_target_creature_until_end_of_turn(power: u32, toughness: u32) -> Self {
        Self {
            targeting: SpellTargetingProfile::ExactlyOne(
                SingleTargetRule::any_creature_on_battlefield(),
            ),
            resolution: SpellResolutionProfile::PumpTargetCreatureUntilEndOfTurn {
                power,
                toughness,
            },
        }
    }

    #[must_use]
    pub const fn targeting(self) -> SpellTargetingProfile {
        self.targeting
    }

    #[must_use]
    pub const fn resolution(self) -> SpellResolutionProfile {
        self.resolution
    }

    #[must_use]
    pub const fn requires_explicit_hand_card_choice(self) -> bool {
        matches!(
            self.resolution,
            SpellResolutionProfile::TargetPlayerDiscardsChosenCard
        )
    }

    #[must_use]
    pub const fn requires_explicit_modal_choice(self) -> bool {
        matches!(
            self.resolution,
            SpellResolutionProfile::ChooseOneTargetPlayerGainOrLoseLife { .. }
        )
    }

    #[must_use]
    pub const fn requires_choice(self) -> bool {
        self.requires_explicit_hand_card_choice() || self.requires_explicit_modal_choice()
    }
}
