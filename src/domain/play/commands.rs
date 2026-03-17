use crate::domain::play::{
    cards::{CardInstance, CardType},
    ids::{CardDefinitionId, CardInstanceId, DeckId, GameId, PlayerId},
};

// Setup and deck-to-play translation

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
pub enum NonCreatureCardType {
    Land,
    Instant,
    Sorcery,
    Enchantment,
    Artifact,
    Planeswalker,
}

impl NonCreatureCardType {
    #[must_use]
    pub const fn to_card_type(self) -> CardType {
        match self {
            Self::Land => CardType::Land,
            Self::Instant => CardType::Instant,
            Self::Sorcery => CardType::Sorcery,
            Self::Enchantment => CardType::Enchantment,
            Self::Artifact => CardType::Artifact,
            Self::Planeswalker => CardType::Planeswalker,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LibraryCard {
    NonCreature {
        definition_id: CardDefinitionId,
        card_type: NonCreatureCardType,
        mana_cost: u32,
    },
    Creature {
        definition_id: CardDefinitionId,
        mana_cost: u32,
        power: u32,
        toughness: u32,
    },
}

impl LibraryCard {
    #[must_use]
    pub const fn non_creature(
        definition_id: CardDefinitionId,
        card_type: NonCreatureCardType,
        mana_cost: u32,
    ) -> Self {
        Self::NonCreature {
            definition_id,
            card_type,
            mana_cost,
        }
    }

    #[must_use]
    pub const fn creature(
        definition_id: CardDefinitionId,
        mana_cost: u32,
        power: u32,
        toughness: u32,
    ) -> Self {
        Self::Creature {
            definition_id,
            mana_cost,
            power,
            toughness,
        }
    }

    #[must_use]
    pub fn to_card_instance(&self, card_id: CardInstanceId) -> CardInstance {
        match self {
            Self::Creature {
                definition_id,
                mana_cost,
                power,
                toughness,
            } => CardInstance::new_creature(
                card_id,
                definition_id.clone(),
                *mana_cost,
                *power,
                *toughness,
            ),
            Self::NonCreature {
                definition_id,
                card_type,
                mana_cost,
            } => CardInstance::new(
                card_id,
                definition_id.clone(),
                card_type.to_card_type(),
                *mana_cost,
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

// Game lifecycle commands

#[derive(Debug, Clone)]
pub struct StartGameCommand {
    pub game_id: GameId,
    pub players: Vec<PlayerDeck>,
}

impl StartGameCommand {
    #[must_use]
    pub const fn new(game_id: GameId, players: Vec<PlayerDeck>) -> Self {
        Self { game_id, players }
    }
}

#[derive(Debug, Clone)]
pub struct DealOpeningHandsCommand {
    pub player_libraries: Vec<PlayerLibrary>,
}

impl DealOpeningHandsCommand {
    #[must_use]
    pub const fn new(player_libraries: Vec<PlayerLibrary>) -> Self {
        Self { player_libraries }
    }
}

#[derive(Debug, Clone)]
pub struct MulliganCommand {
    pub player_id: PlayerId,
}

impl MulliganCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId) -> Self {
        Self { player_id }
    }
}

// Turn flow commands

#[derive(Debug, Clone, Default)]
pub struct AdvanceTurnCommand;

impl AdvanceTurnCommand {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct DrawCardEffectCommand {
    pub player_id: PlayerId,
}

impl DrawCardEffectCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId) -> Self {
        Self { player_id }
    }
}

#[derive(Debug, Clone)]
pub struct DiscardForCleanupCommand {
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
}

impl DiscardForCleanupCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId, card_id: CardInstanceId) -> Self {
        Self { player_id, card_id }
    }
}

// Resource and battlefield commands

#[derive(Debug, Clone)]
pub struct PlayLandCommand {
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
}

impl PlayLandCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId, card_id: CardInstanceId) -> Self {
        Self { player_id, card_id }
    }
}

#[derive(Debug, Clone)]
pub struct AdjustLifeCommand {
    pub player_id: PlayerId,
    pub life_delta: i32,
}

impl AdjustLifeCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId, life_delta: i32) -> Self {
        Self {
            player_id,
            life_delta,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TapLandCommand {
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
}

impl TapLandCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId, card_id: CardInstanceId) -> Self {
        Self { player_id, card_id }
    }
}

#[derive(Debug, Clone)]
pub struct CastSpellCommand {
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
}

impl CastSpellCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId, card_id: CardInstanceId) -> Self {
        Self { player_id, card_id }
    }
}

// Combat commands

#[derive(Debug, Clone)]
pub struct DeclareAttackersCommand {
    pub player_id: PlayerId,
    pub attacker_ids: Vec<CardInstanceId>,
}

impl DeclareAttackersCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId, attacker_ids: Vec<CardInstanceId>) -> Self {
        Self {
            player_id,
            attacker_ids,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeclareBlockersCommand {
    pub player_id: PlayerId,
    pub blocker_assignments: Vec<(CardInstanceId, CardInstanceId)>,
}

impl DeclareBlockersCommand {
    #[must_use]
    pub const fn new(
        player_id: PlayerId,
        blocker_assignments: Vec<(CardInstanceId, CardInstanceId)>,
    ) -> Self {
        Self {
            player_id,
            blocker_assignments,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResolveCombatDamageCommand {
    pub player_id: PlayerId,
}

impl ResolveCombatDamageCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId) -> Self {
        Self { player_id }
    }
}
