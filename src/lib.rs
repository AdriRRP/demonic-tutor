pub mod application;
pub mod domain;
pub mod infrastructure;

pub use application::{EventBus, EventStore, GameService};

pub use infrastructure::{GameLogProjection, InMemoryEventBus, InMemoryEventStore};

pub use domain::play::{
    cards::{CardInstance, CardType},
    commands::{
        AdjustLifeCommand, AdvanceTurnCommand, CastSpellCommand, DealOpeningHandsCommand,
        DeclareAttackersCommand, DeclareBlockersCommand, DiscardForCleanupCommand,
        DrawCardsEffectCommand, LibraryCard, MulliganCommand, NonCreatureCardType, PlayLandCommand,
        PlayerDeck, PlayerLibrary, ResolveCombatDamageCommand, StartGameCommand, TapLandCommand,
    },
    errors::{CardError, DomainError, GameError, PhaseError, PlayerError},
    events::{
        AttackersDeclared, BlockersDeclared, CardDiscarded, CardDrawn, CombatDamageResolved,
        CreatureDied, DamageEvent, DiscardKind, DomainEvent, DrawKind, GameEndReason, GameEnded,
        GameStarted, LandPlayed, LandTapped, LifeChanged, ManaAdded, MulliganTaken,
        OpeningHandDealt, SpellCast, SpellCastOutcome, TurnProgressed,
    },
    game::{AdjustLifeOutcome, AdvanceTurnOutcome, CastSpellOutcome, DrawCardsEffectOutcome, Game},
    ids::{CardDefinitionId, CardInstanceId, DeckId, GameId, PlayerId},
    phase::Phase,
    zones::{Battlefield, Graveyard, Hand, Library},
};
