pub mod application;
pub mod domain;

pub use application::GameService;

pub use domain::{
    cards::{CardInstance, CardType},
    commands::{
        AdvanceTurnCommand, DealOpeningHandsCommand, DrawCardCommand, PlayLandCommand, PlayerDeck,
        PlayerDeckContents, StartGameCommand,
    },
    errors::DomainError,
    events::{CardDrawn, GameStarted, LandPlayed, OpeningHandDealt, TurnAdvanced},
    game::{Game, Phase},
    ids::{CardDefinitionId, CardInstanceId, DeckId, GameId, PlayerId},
    zones::{Battlefield, Hand},
};
