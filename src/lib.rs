pub mod application;
pub mod domain;
pub mod infrastructure;

pub use application::{EventBus, EventStore, GameService};

pub use infrastructure::{GameLogProjection, InMemoryEventBus, InMemoryEventStore};

pub use domain::{
    cards::{CardInstance, CardType},
    commands::{
        AdvanceTurnCommand, DealOpeningHandsCommand, DrawCardCommand, MulliganCommand,
        PlayLandCommand, PlayerDeck, PlayerDeckContents, SetLifeCommand, StartGameCommand,
        TapLandCommand,
    },
    errors::DomainError,
    events::{
        CardDrawn, DomainEvent, GameStarted, LandPlayed, LandTapped, LifeChanged, ManaAdded,
        MulliganTaken, OpeningHandDealt, PhaseChanged, TurnAdvanced, TurnNumberChanged,
    },
    game::{Game, Phase},
    ids::{CardDefinitionId, CardInstanceId, DeckId, GameId, PlayerId},
    zones::{Battlefield, Hand},
};
