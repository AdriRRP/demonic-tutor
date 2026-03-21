use crate::domain::play::{
    cards::{CardInstance, SpellTargetingProfile, SupportedSpellRules},
    game::{Player, SpellTarget},
    ids::{CardInstanceId, PlayerId},
};

#[must_use]
pub const fn supported_spell_rules(card: &CardInstance) -> SupportedSpellRules {
    card.supported_spell_rules()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellTargetLegality {
    NoTargetRequired,
    MissingRequiredTarget,
    IllegalTargetKind,
    MissingPlayer(PlayerId),
    MissingCreature(CardInstanceId),
    Legal,
}

#[derive(Debug, Clone, Copy)]
pub enum TargetLegalityContext<'a> {
    Cast {
        players: &'a [Player],
        caster_id: &'a PlayerId,
    },
    Resolution {
        players: &'a [Player],
        controller_id: &'a PlayerId,
    },
}

impl<'a> TargetLegalityContext<'a> {
    #[must_use]
    pub const fn players(self) -> &'a [Player] {
        match self {
            Self::Cast { players, .. } | Self::Resolution { players, .. } => players,
        }
    }

    #[must_use]
    pub const fn actor_id(self) -> &'a PlayerId {
        match self {
            Self::Cast { caster_id, .. } => caster_id,
            Self::Resolution { controller_id, .. } => controller_id,
        }
    }
}

#[must_use]
pub const fn accepts_target(targeting: SpellTargetingProfile, target: &SpellTarget) -> bool {
    targeting.allows_target_kind(target.kind())
}

#[must_use]
pub fn evaluate_target_legality(
    context: TargetLegalityContext<'_>,
    targeting: SpellTargetingProfile,
    target: Option<&SpellTarget>,
) -> SpellTargetLegality {
    let players = context.players();
    let _actor_id = context.actor_id();
    match (targeting, target) {
        (SpellTargetingProfile::None, None) => SpellTargetLegality::NoTargetRequired,
        (SpellTargetingProfile::None, Some(_)) => SpellTargetLegality::IllegalTargetKind,
        (SpellTargetingProfile::ExactlyOne(_), None) => SpellTargetLegality::MissingRequiredTarget,
        (targeting, Some(target)) => {
            if !accepts_target(targeting, target) {
                return SpellTargetLegality::IllegalTargetKind;
            }

            match target {
                SpellTarget::Player(player_id) => {
                    if players.iter().any(|player| player.id() == player_id) {
                        SpellTargetLegality::Legal
                    } else {
                        SpellTargetLegality::MissingPlayer(player_id.clone())
                    }
                }
                SpellTarget::Creature(card_id) => {
                    let found = players.iter().any(|player| {
                        player
                            .battlefield()
                            .cards()
                            .iter()
                            .any(|card| card.id() == card_id)
                    });
                    if found {
                        SpellTargetLegality::Legal
                    } else {
                        SpellTargetLegality::MissingCreature(card_id.clone())
                    }
                }
            }
        }
    }
}
