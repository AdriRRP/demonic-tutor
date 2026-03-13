pub mod application;
pub mod domain;
pub mod infrastructure;

pub use application::{EventBus, EventStore, GameService};

pub use domain::{
    cards::{CardInstance, CardType},
    commands::{
        AdvanceTurnCommand, DealOpeningHandsCommand, DrawCardCommand, MulliganCommand,
        PlayLandCommand, PlayerDeck, PlayerDeckContents, StartGameCommand,
    },
    errors::DomainError,
    events::{
        CardDrawn, DomainEvent, GameStarted, LandPlayed, MulliganTaken, OpeningHandDealt,
        TurnAdvanced,
    },
    game::{Game, Phase},
    ids::{CardDefinitionId, CardInstanceId, DeckId, GameId, PlayerId},
    zones::{Battlefield, Hand},
};
