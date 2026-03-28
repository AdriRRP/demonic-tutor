//! Defines the public gameplay contract consumed by clients.

use crate::domain::play::{
    cards::{CardType, KeywordAbility},
    commands::{
        ActivateAbilityCommand, AdjustPlayerLifeEffectCommand, AdvanceTurnCommand,
        CastSpellCommand, ConcedeCommand, DeclareAttackersCommand, DeclareBlockersCommand,
        DiscardForCleanupCommand, DrawCardsEffectCommand, ExileCardCommand, LibraryCard,
        ModalSpellMode, PassPriorityCommand, PlayLandCommand, PlayerDeck,
        ResolveCombatDamageCommand, ResolveOptionalEffectCommand, ResolvePendingHandChoiceCommand,
        ResolvePendingScryCommand, ResolvePendingSurveilCommand, TapLandCommand,
    },
    events::{
        ActivatedAbilityPutOnStack, AttackersDeclared, BlockersDeclared, CardDiscarded,
        CardMovedZone, CombatDamageResolved, CreatureDied, DrawKind, GameEndReason, GameEnded,
        GameStarted, LandPlayed, LandTapped, LifeChanged, ManaAdded, MulliganTaken, PriorityPassed,
        SpellCast, SpellPutOnStack, StackTopResolved, TriggeredAbilityPutOnStack, TurnProgressed,
    },
    ids::{CardDefinitionId, CardInstanceId, DeckId, GameId, PlayerId, StackObjectId},
    phase::Phase,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicPriorityView {
    pub current_holder: PlayerId,
    pub has_pending_pass: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicCardView {
    pub card_id: CardInstanceId,
    pub definition_id: CardDefinitionId,
    pub card_type: CardType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicBattlefieldCardView {
    pub card_id: CardInstanceId,
    pub definition_id: CardDefinitionId,
    pub card_type: CardType,
    pub permanent_state: PublicPermanentStateView,
    pub attached_to: Option<CardInstanceId>,
    pub power: Option<u32>,
    pub toughness: Option<u32>,
    pub loyalty: Option<u32>,
    pub combat_state: PublicCombatStateView,
    pub keywords: Vec<KeywordAbility>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicPermanentStateView {
    pub tapped: bool,
    pub token: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicCombatStateView {
    pub summoning_sickness: bool,
    pub attacking: bool,
    pub blocking: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicPlayerView {
    pub player_id: PlayerId,
    pub is_active: bool,
    pub life: u32,
    pub mana_total: u32,
    pub hand_count: usize,
    pub library_count: usize,
    pub battlefield: Vec<PublicBattlefieldCardView>,
    pub graveyard: Vec<PublicCardView>,
    pub exile: Vec<PublicCardView>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicStackTargetView {
    Unavailable,
    Player(PlayerId),
    Card(CardInstanceId),
    StackSpell(StackObjectId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicPlayableSubsetVersion {
    V1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicStackObjectView {
    Unavailable {
        number: u32,
    },
    Spell {
        number: u32,
        controller_id: PlayerId,
        source_card_id: CardInstanceId,
        card_type: CardType,
        target: Option<PublicStackTargetView>,
        requires_choice: bool,
    },
    ActivatedAbility {
        number: u32,
        controller_id: PlayerId,
        source_card_id: CardInstanceId,
        target: Option<PublicStackTargetView>,
    },
    TriggeredAbility {
        number: u32,
        controller_id: PlayerId,
        source_card_id: CardInstanceId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicGameView {
    pub game_id: GameId,
    pub playable_subset_version: PublicPlayableSubsetVersion,
    pub active_player_id: PlayerId,
    pub phase: Phase,
    pub turn_number: u32,
    pub priority: Option<PublicPriorityView>,
    pub is_over: bool,
    pub winner_id: Option<PlayerId>,
    pub loser_id: Option<PlayerId>,
    pub end_reason: Option<GameEndReason>,
    pub players: Vec<PublicPlayerView>,
    pub stack: Vec<PublicStackObjectView>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicCastableCard {
    pub card_id: CardInstanceId,
    pub definition_id: CardDefinitionId,
    pub card_type: CardType,
    pub requires_target: bool,
    pub requires_choice: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicActivatableCard {
    pub card_id: CardInstanceId,
    pub definition_id: CardDefinitionId,
    pub requires_target: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicBlockerOption {
    pub blocker_id: CardInstanceId,
    pub attacker_ids: Vec<CardInstanceId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicLegalAction {
    Concede {
        player_id: PlayerId,
    },
    ResolvePendingScry {
        player_id: PlayerId,
    },
    ResolvePendingSurveil {
        player_id: PlayerId,
    },
    ResolvePendingHandChoice {
        player_id: PlayerId,
    },
    ResolveOptionalEffect {
        player_id: PlayerId,
    },
    PassPriority {
        player_id: PlayerId,
    },
    PlayLand {
        player_id: PlayerId,
        playable_land_ids: Vec<CardInstanceId>,
    },
    TapManaSource {
        player_id: PlayerId,
        mana_source_ids: Vec<CardInstanceId>,
    },
    CastSpell {
        player_id: PlayerId,
        castable_cards: Vec<PublicCastableCard>,
    },
    ActivateAbility {
        player_id: PlayerId,
        activatable_cards: Vec<PublicActivatableCard>,
    },
    DeclareAttackers {
        player_id: PlayerId,
        attacker_ids: Vec<CardInstanceId>,
    },
    DeclareBlockers {
        player_id: PlayerId,
        attacker_ids: Vec<CardInstanceId>,
        blocker_options: Vec<PublicBlockerOption>,
    },
    ResolveCombatDamage {
        player_id: PlayerId,
    },
    AdvanceTurn {
        player_id: PlayerId,
    },
    DiscardForCleanup {
        player_id: PlayerId,
        card_ids: Vec<CardInstanceId>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicChoiceCandidate {
    Player(PlayerId),
    Card(CardInstanceId),
    StackSpell(StackObjectId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicChoiceRequest {
    PhaseUnavailable {
        player_id: PlayerId,
        phase: Phase,
    },
    PriorityUnavailable {
        player_id: PlayerId,
    },
    PendingDecisionUnavailable {
        player_id: PlayerId,
        decision: PublicPendingDecisionKind,
    },
    PendingScry {
        player_id: PlayerId,
        source_card_id: CardInstanceId,
        looked_at_card_ids: Vec<CardInstanceId>,
        options: Vec<PublicScryChoice>,
    },
    PendingSurveil {
        player_id: PlayerId,
        source_card_id: CardInstanceId,
        looked_at_card_ids: Vec<CardInstanceId>,
        options: Vec<PublicSurveilChoice>,
    },
    PendingHandChoice {
        player_id: PlayerId,
        source_card_id: CardInstanceId,
        hand_card_ids: Vec<CardInstanceId>,
    },
    OptionalEffectDecision {
        player_id: PlayerId,
        source_card_id: CardInstanceId,
        options: Vec<PublicBinaryChoice>,
    },
    SpellTarget {
        player_id: PlayerId,
        source_card_id: CardInstanceId,
        candidates: Vec<PublicChoiceCandidate>,
    },
    SpellChoiceUnavailable {
        player_id: PlayerId,
        source_card_id: CardInstanceId,
    },
    SpellChoice {
        player_id: PlayerId,
        source_card_id: CardInstanceId,
        hand_card_ids: Vec<CardInstanceId>,
    },
    SpellSecondaryCreatureChoice {
        player_id: PlayerId,
        source_card_id: CardInstanceId,
        creature_ids: Vec<CardInstanceId>,
        allows_skipping: bool,
    },
    SpellModalChoice {
        player_id: PlayerId,
        source_card_id: CardInstanceId,
        modes: Vec<PublicModalSpellChoice>,
    },
    AbilityTarget {
        player_id: PlayerId,
        source_card_id: CardInstanceId,
        candidates: Vec<PublicChoiceCandidate>,
    },
    CleanupDiscard {
        player_id: PlayerId,
        hand_card_ids: Vec<CardInstanceId>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicPendingDecisionKind {
    Scry,
    Surveil,
    HandChoice,
    OptionalEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicModalSpellChoice {
    TargetPlayerGainLife,
    TargetPlayerLoseLife,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicBinaryChoice {
    Yes,
    No,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicScryChoice {
    KeepOnTop,
    MoveToBottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicSurveilChoice {
    KeepOnTop,
    MoveToGraveyard,
}

#[derive(Debug, Clone)]
pub enum PublicGameCommand {
    Concede(ConcedeCommand),
    PlayLand(PlayLandCommand),
    TapLand(TapLandCommand),
    CastSpell(CastSpellCommand),
    ActivateAbility(ActivateAbilityCommand),
    PassPriority(PassPriorityCommand),
    DeclareAttackers(DeclareAttackersCommand),
    DeclareBlockers(DeclareBlockersCommand),
    ResolveCombatDamage(ResolveCombatDamageCommand),
    AdvanceTurn(AdvanceTurnCommand),
    DrawCardsEffect(DrawCardsEffectCommand),
    DiscardForCleanup(DiscardForCleanupCommand),
    AdjustPlayerLifeEffect(AdjustPlayerLifeEffectCommand),
    ExileCard(ExileCardCommand),
    ResolveOptionalEffect(ResolveOptionalEffectCommand),
    ResolvePendingHandChoice(ResolvePendingHandChoiceCommand),
    ResolvePendingScry(ResolvePendingScryCommand),
    ResolvePendingSurveil(ResolvePendingSurveilCommand),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicCommandRejection {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicCommandStatus {
    Applied,
    Rejected(PublicCommandRejection),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicOpeningHandDealt {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub card_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicCardDrawn {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub draw_kind: DrawKind,
}

#[derive(Debug, Clone)]
pub enum PublicEvent {
    GameStarted(GameStarted),
    OpeningHandDealt(PublicOpeningHandDealt),
    GameEnded(GameEnded),
    LandPlayed(LandPlayed),
    TurnProgressed(TurnProgressed),
    CardDrawn(PublicCardDrawn),
    CardDiscarded(CardDiscarded),
    MulliganTaken(MulliganTaken),
    LifeChanged(LifeChanged),
    LandTapped(LandTapped),
    ManaAdded(ManaAdded),
    ActivatedAbilityPutOnStack(ActivatedAbilityPutOnStack),
    TriggeredAbilityPutOnStack(TriggeredAbilityPutOnStack),
    SpellPutOnStack(SpellPutOnStack),
    PriorityPassed(PriorityPassed),
    StackTopResolved(StackTopResolved),
    SpellCast(SpellCast),
    AttackersDeclared(AttackersDeclared),
    BlockersDeclared(BlockersDeclared),
    CombatDamageResolved(CombatDamageResolved),
    CreatureDied(CreatureDied),
    CardMovedZone(CardMovedZone),
}

#[derive(Debug, Clone)]
pub struct PublicCommandApplication {
    pub status: PublicCommandStatus,
    pub emitted_events: Vec<PublicEvent>,
}

#[derive(Debug, Clone)]
pub struct PublicCommandResult {
    pub status: PublicCommandStatus,
    pub emitted_events: Vec<PublicEvent>,
    pub game: PublicGameView,
    pub legal_actions: Vec<PublicLegalAction>,
    pub choice_requests: Vec<PublicChoiceRequest>,
}

#[derive(Debug, Clone)]
pub struct PublicEventLogEntry {
    pub sequence: u64,
    pub event: PublicEvent,
}

#[derive(Debug, Clone)]
pub struct PublicSeededPlayerSetup {
    pub player_id: PlayerId,
    pub deck_id: DeckId,
    pub cards: Vec<LibraryCard>,
}

impl PublicSeededPlayerSetup {
    #[must_use]
    pub const fn new(player_id: PlayerId, deck_id: DeckId, cards: Vec<LibraryCard>) -> Self {
        Self {
            player_id,
            deck_id,
            cards,
        }
    }

    #[must_use]
    pub fn player_deck(&self) -> PlayerDeck {
        PlayerDeck::new(self.player_id.clone(), self.deck_id.clone())
    }
}

#[derive(Debug, Clone)]
pub struct PublicSeededGameSetup {
    pub game_id: GameId,
    pub players: Vec<PublicSeededPlayerSetup>,
    pub shuffle_seed: u64,
}

impl PublicSeededGameSetup {
    #[must_use]
    pub const fn new(
        game_id: GameId,
        players: Vec<PublicSeededPlayerSetup>,
        shuffle_seed: u64,
    ) -> Self {
        Self {
            game_id,
            players,
            shuffle_seed,
        }
    }

    #[must_use]
    pub fn with_game_id(mut self, game_id: GameId) -> Self {
        self.game_id = game_id;
        self
    }
}

#[derive(Debug, Clone)]
pub struct PublicRematchCommand {
    pub game_id: GameId,
    pub original_setup: PublicSeededGameSetup,
}

impl PublicRematchCommand {
    #[must_use]
    pub const fn new(game_id: GameId, original_setup: PublicSeededGameSetup) -> Self {
        Self {
            game_id,
            original_setup,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PublicGameSessionStart {
    pub emitted_events: Vec<PublicEvent>,
    pub game: PublicGameView,
    pub legal_actions: Vec<PublicLegalAction>,
    pub choice_requests: Vec<PublicChoiceRequest>,
}

impl From<ModalSpellMode> for PublicModalSpellChoice {
    fn from(value: ModalSpellMode) -> Self {
        match value {
            ModalSpellMode::TargetPlayerGainLife => Self::TargetPlayerGainLife,
            ModalSpellMode::TargetPlayerLoseLife => Self::TargetPlayerLoseLife,
        }
    }
}
