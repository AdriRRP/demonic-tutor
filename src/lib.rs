//! Supports the `DemonicTutor` library crate.

pub mod application;
pub mod domain;
pub mod infrastructure;

pub use application::{
    choice_requests, game_view, legal_actions, public_command_result, PublicActivatableCard,
    PublicBattlefieldCardView, PublicBinaryChoice, PublicCardView, PublicCastableCard,
    PublicChoiceCandidate, PublicChoiceRequest, PublicCommandApplication, PublicCommandRejection,
    PublicCommandResult, PublicCommandStatus, PublicGameCommand, PublicGameView, PublicLegalAction,
    PublicModalSpellChoice, PublicPlayerView, PublicPriorityView, PublicScryChoice,
    PublicStackObjectView, PublicStackTargetView,
};
pub use application::{EventBus, EventStore, GameService};

pub use infrastructure::{GameLogProjection, InMemoryEventBus, InMemoryEventStore};

pub use domain::play::{
    cards::{
        ActivatedAbilityEffect, ActivatedAbilityProfile, ActivatedAbilitySacrificeCost,
        ActivatedManaAbilityProfile, AttachedCombatRestrictionProfile, AttachedStatBoostProfile,
        AttachmentProfile, CardInstance, CardType, CastingPermissionProfile, CastingRule,
        ControllerStaticEffectProfile, CreatureTargetRule, KeywordAbility, KeywordAbilitySet,
        ManaColor, ManaCost, PlayerTargetRule, SingleTargetRule, SpellResolutionProfile,
        SpellTargetKind, SpellTargetingProfile, SupportedSpellRules, TriggeredAbilityEffect,
        TriggeredAbilityEvent, TriggeredAbilityProfile,
    },
    commands::{
        ActivateAbilityCommand, AdjustPlayerLifeEffectCommand, AdvanceTurnCommand,
        CastSpellCommand, DealOpeningHandsCommand, DeclareAttackersCommand, DeclareBlockersCommand,
        DiscardForCleanupCommand, DrawCardsEffectCommand, ExileCardCommand, LibraryCard,
        LibraryCreature, ModalSpellMode, MulliganCommand, PassPriorityCommand, PlayLandCommand,
        PlayerDeck, PlayerLibrary, ResolveCombatDamageCommand, ResolveOptionalEffectCommand,
        ResolvePendingHandChoiceCommand, ResolvePendingScryCommand, SpellChoice, StartGameCommand,
        TapLandCommand,
    },
    errors::{CardError, DomainError, GameError, PhaseError, PlayerError},
    events::{
        ActivatedAbilityPutOnStack, AttackersDeclared, BlockersDeclared, CardDiscarded, CardDrawn,
        CardExiled, CombatDamageResolved, CreatureDied, DamageEvent, DiscardKind, DomainEvent,
        DrawKind, GameEndReason, GameEnded, GameStarted, LandPlayed, LandTapped, LifeChanged,
        ManaAdded, MulliganTaken, OpeningHandDealt, PriorityPassed, SpellCast, SpellCastOutcome,
        SpellPutOnStack, StackTopResolved, TriggeredAbilityPutOnStack, TurnProgressed,
    },
    game::{
        ActivateAbilityOutcome, ActivatedAbilityOnStack, AdjustPlayerLifeEffectOutcome,
        AdvanceTurnOutcome, CastSpellOutcome, DrawCardsEffectOutcome, Game, PassPriorityOutcome,
        PendingDecision, PendingHandChoiceKind, PriorityState, ResolveOptionalEffectOutcome,
        ResolvePendingHandChoiceOutcome, ResolvePendingScryOutcome, SpellOnStack, SpellTarget,
        StackObject, StackObjectKind, StackZone, TriggeredAbilityOnStack,
    },
    ids::{CardDefinitionId, CardInstanceId, DeckId, GameId, PlayerId, StackObjectId},
    phase::Phase,
    zones::{Battlefield, Graveyard, Hand, Library},
};
