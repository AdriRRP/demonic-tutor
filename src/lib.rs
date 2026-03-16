pub mod application;
pub mod domain;
pub mod infrastructure;

pub use application::{EventBus, EventStore, GameService};

pub use infrastructure::{GameLogProjection, InMemoryEventBus, InMemoryEventStore};

pub use domain::{
    cards::{CardInstance, CardType},
    commands::{
        AdvanceTurnCommand, CardWithCost, CastSpellCommand, DealOpeningHandsCommand,
        DeclareAttackersCommand, DeclareBlockersCommand, DrawCardCommand, MulliganCommand,
        PlayCreatureCommand, PlayLandCommand, PlayerDeck, PlayerDeckContents,
        ResolveCombatDamageCommand, SetLifeCommand, StartGameCommand, TapLandCommand,
    },
    errors::{CardError, DomainError, GameError, PhaseError, PlayerError},
    events::{
        AttackersDeclared, BlockersDeclared, CardDrawn, CombatDamageResolved,
        CreatureEnteredBattlefield, DamageEvent, DomainEvent, GameStarted, LandPlayed, LandTapped,
        LifeChanged, ManaAdded, MulliganTaken, OpeningHandDealt, PhaseChanged, SpellCast,
        TurnAdvanced, TurnNumberChanged,
    },
    game::{Game, Phase},
    ids::{CardDefinitionId, CardInstanceId, DeckId, GameId, PlayerId},
    zones::{Battlefield, Hand},
};
