pub mod application;
pub mod domain;
pub mod infrastructure;

pub use application::{EventBus, EventStore, GameService};

pub use infrastructure::{GameLogProjection, InMemoryEventBus, InMemoryEventStore};

pub use domain::play::{
    cards::{CardInstance, CardType},
    commands::{
        AdjustPlayerLifeEffectCommand, AdvanceTurnCommand, CastSpellCommand,
        DealOpeningHandsCommand, DeclareAttackersCommand, DeclareBlockersCommand,
        DiscardForCleanupCommand, DrawCardsEffectCommand, ExileCardCommand, LibraryCard,
        MulliganCommand, NonCreatureCardType, PassPriorityCommand, PlayLandCommand, PlayerDeck,
        PlayerLibrary, ResolveCombatDamageCommand, StartGameCommand, TapLandCommand,
    },
    errors::{CardError, DomainError, GameError, PhaseError, PlayerError},
    events::{
        AttackersDeclared, BlockersDeclared, CardDiscarded, CardDrawn, CardExiled,
        CombatDamageResolved, CreatureDied, DamageEvent, DiscardKind, DomainEvent, DrawKind,
        GameEndReason, GameEnded, GameStarted, LandPlayed, LandTapped, LifeChanged, ManaAdded,
        MulliganTaken, OpeningHandDealt, PriorityPassed, SpellCast, SpellCastOutcome,
        SpellPutOnStack, StackTopResolved, TurnProgressed,
    },
    game::{
        AdjustPlayerLifeEffectOutcome, AdvanceTurnOutcome, CastSpellOutcome,
        DrawCardsEffectOutcome, Game, PassPriorityOutcome, PriorityState, SpellOnStack,
        SpellTarget, StackObject, StackObjectKind, StackZone,
    },
    ids::{CardDefinitionId, CardInstanceId, DeckId, GameId, PlayerId, StackObjectId},
    phase::Phase,
    zones::{Battlefield, Graveyard, Hand, Library},
};
