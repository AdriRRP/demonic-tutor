//! Supports play events zones.

//! `zone_owner_id` names the player whose visible zone receives or owns the card
//! for the movement being reported. It is intentionally not phrased as
//! controller, because the current engine already reports moves for cards whose
//! controller and owner may diverge in future slices.

use crate::domain::play::ids::{CardInstanceId, GameId, PlayerId};

#[derive(Debug, Clone)]
pub enum ZoneType {
    Created,
    Library,
    Hand,
    Battlefield,
    Graveyard,
    Exile,
    Stack,
}

impl ZoneType {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Library => "library",
            Self::Hand => "hand",
            Self::Battlefield => "battlefield",
            Self::Graveyard => "graveyard",
            Self::Exile => "exile",
            Self::Stack => "stack",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CardMovedZone {
    pub game_id: GameId,
    pub zone_owner_id: PlayerId,
    pub card_id: CardInstanceId,
    pub origin_zone: ZoneType,
    pub destination_zone: ZoneType,
}

impl CardMovedZone {
    #[must_use]
    pub const fn new(
        game_id: GameId,
        zone_owner_id: PlayerId,
        card_id: CardInstanceId,
        origin_zone: ZoneType,
        destination_zone: ZoneType,
    ) -> Self {
        Self {
            game_id,
            zone_owner_id,
            card_id,
            origin_zone,
            destination_zone,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CardExiled {
    pub game_id: GameId,
    pub zone_owner_id: PlayerId,
    pub card_id: CardInstanceId,
    pub origin_zone: ZoneType,
}

impl CardExiled {
    #[must_use]
    pub const fn new(
        game_id: GameId,
        zone_owner_id: PlayerId,
        card_id: CardInstanceId,
        origin_zone: ZoneType,
    ) -> Self {
        Self {
            game_id,
            zone_owner_id,
            card_id,
            origin_zone,
        }
    }
}
