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

#[must_use]
pub const fn accepts_target(targeting: SpellTargetingProfile, target: &SpellTarget) -> bool {
    targeting.allows_target_kind(target.kind())
}

#[must_use]
pub fn evaluate_target_legality(
    players: &[Player],
    targeting: SpellTargetingProfile,
    target: Option<&SpellTarget>,
) -> SpellTargetLegality {
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
