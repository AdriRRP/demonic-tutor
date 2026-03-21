mod combat;
mod common;
mod resource_actions;
mod stack;
mod turn_flow;

use crate::{
    application::{EventBus, EventStore},
    domain::play::{
        commands::{
            AdjustPlayerLifeEffectCommand, AdvanceTurnCommand, CastSpellCommand,
            DealOpeningHandsCommand, DeclareAttackersCommand, DeclareBlockersCommand,
            DiscardForCleanupCommand, DrawCardsEffectCommand, ExileCardCommand, MulliganCommand,
            PassPriorityCommand, PlayLandCommand, ResolveCombatDamageCommand, StartGameCommand,
            TapLandCommand,
        },
        errors::{DomainError, GameError},
        events::{
            AttackersDeclared, BlockersDeclared, CardDiscarded, CardExiled, DomainEvent,
            GameStarted, LandPlayed, LandTapped, ManaAdded, MulliganTaken, OpeningHandDealt,
        },
        game::{
            AdjustPlayerLifeEffectOutcome, AdvanceTurnOutcome, CastSpellOutcome,
            DrawCardsEffectOutcome, Game, PassPriorityOutcome, ResolveCombatDamageOutcome,
        },
    },
};

use self::{
    combat::domain_events_for_resolve_combat_damage,
    common::DomainEvents,
    resource_actions::domain_events_for_adjust_player_life_effect,
    stack::{domain_events_for_cast_spell, domain_events_for_pass_priority},
    turn_flow::{domain_events_for_advance_turn, domain_events_for_draw_cards_effect},
};

pub struct GameService<E, B>
where
    E: EventStore,
    B: EventBus,
{
    event_store: E,
    event_bus: B,
}

impl<E, B> GameService<E, B>
where
    E: EventStore,
    B: EventBus,
{
    #[must_use]
    pub const fn new(event_store: E, event_bus: B) -> Self {
        Self {
            event_store,
            event_bus,
        }
    }

    fn persist_and_publish_events(
        &self,
        game_id: &str,
        events: &[DomainEvent],
    ) -> Result<(), DomainError> {
        if !events.is_empty() {
            self.event_store.append(game_id, events).map_err(|err| {
                DomainError::Game(GameError::InternalInvariantViolation(format!(
                    "failed to persist domain events for aggregate {game_id}: {err}"
                )))
            })?;
            for event in events {
                self.event_bus.publish(event);
            }
        }

        Ok(())
    }

    fn persist_and_publish_event<T>(&self, game_id: &str, event: &T) -> Result<(), DomainError>
    where
        T: Clone + Into<DomainEvent>,
    {
        self.persist_and_publish_events(game_id, &[event.clone().into()])
    }

    fn persist_and_publish_event_batch<T>(
        &self,
        game_id: &str,
        events: &[T],
    ) -> Result<(), DomainError>
    where
        T: Clone + Into<DomainEvent>,
    {
        if events.is_empty() {
            return Ok(());
        }

        let domain_events = events.iter().cloned().map(Into::into).collect::<Vec<_>>();
        self.persist_and_publish_events(game_id, &domain_events)
    }

    /// Starts a new game.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn start_game(&self, cmd: StartGameCommand) -> Result<(Game, GameStarted), DomainError> {
        let (game, event) = Game::start(cmd)?;
        self.persist_and_publish_event(game.id().as_str(), &event)?;

        Ok((game, event))
    }

    /// Deals opening hands to all players.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn deal_opening_hands(
        &self,
        game: &mut Game,
        cmd: &DealOpeningHandsCommand,
    ) -> Result<Vec<OpeningHandDealt>, DomainError> {
        let events = game.deal_opening_hands(cmd)?;
        self.persist_and_publish_event_batch(game.id().as_str(), &events)?;

        Ok(events)
    }

    /// Plays a land card.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn play_land(
        &self,
        game: &mut Game,
        cmd: PlayLandCommand,
    ) -> Result<LandPlayed, DomainError> {
        let event = game.play_land(cmd)?;
        self.persist_and_publish_event(game.id().as_str(), &event)?;

        Ok(event)
    }

    /// Advances the turn to the next player.
    ///
    /// # Errors
    ///
    /// Returns an error if the active player cannot be found or auto-draw fails.
    pub fn advance_turn(
        &self,
        game: &mut Game,
        cmd: AdvanceTurnCommand,
    ) -> Result<AdvanceTurnOutcome, DomainError> {
        let outcome = game.advance_turn(cmd)?;
        let domain_events = domain_events_for_advance_turn(&outcome);
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok(outcome)
    }

    /// Resolves an explicit draw effect from the active player onto a target player.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn draw_cards_effect(
        &self,
        game: &mut Game,
        cmd: &DrawCardsEffectCommand,
    ) -> Result<DrawCardsEffectOutcome, DomainError> {
        let outcome = game.draw_cards_effect(cmd)?;
        let domain_events = domain_events_for_draw_cards_effect(&outcome);
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok(outcome)
    }

    /// Discards one card from hand during cleanup-related turn flow.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn discard_for_cleanup(
        &self,
        game: &mut Game,
        cmd: DiscardForCleanupCommand,
    ) -> Result<CardDiscarded, DomainError> {
        let event = game.discard_for_cleanup(cmd)?;
        self.persist_and_publish_event(game.id().as_str(), &event)?;

        Ok(event)
    }

    /// Exiles a card.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn exile_card(
        &self,
        game: &mut Game,
        cmd: &ExileCardCommand,
    ) -> Result<CardExiled, DomainError> {
        let event = game.exile_card(cmd)?;
        self.persist_and_publish_event(game.id().as_str(), &event)?;

        Ok(event)
    }

    /// Performs a mulligan for a player.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn mulligan(
        &self,
        game: &mut Game,
        cmd: MulliganCommand,
    ) -> Result<MulliganTaken, DomainError> {
        let event = game.mulligan(cmd)?;
        self.persist_and_publish_event(game.id().as_str(), &event)?;

        Ok(event)
    }

    /// Resolves an explicit life effect from a caster onto a target player.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn adjust_player_life_effect(
        &self,
        game: &mut Game,
        cmd: AdjustPlayerLifeEffectCommand,
    ) -> Result<AdjustPlayerLifeEffectOutcome, DomainError> {
        let outcome = game.adjust_player_life_effect(cmd)?;
        let domain_events = domain_events_for_adjust_player_life_effect(&outcome);
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok(outcome)
    }

    /// Taps a land to add mana.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn tap_land(
        &self,
        game: &mut Game,
        cmd: TapLandCommand,
    ) -> Result<(LandTapped, ManaAdded), DomainError> {
        let (land_event, mana_event) = game.tap_land(cmd)?;
        let mut domain_events = DomainEvents::with(land_event.clone());
        domain_events.push(mana_event.clone());
        let domain_events = domain_events.into_vec();
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok((land_event, mana_event))
    }

    /// Casts a spell.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn cast_spell(
        &self,
        game: &mut Game,
        cmd: CastSpellCommand,
    ) -> Result<CastSpellOutcome, DomainError> {
        let outcome = game.cast_spell(cmd)?;
        let domain_events = domain_events_for_cast_spell(&outcome);
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok(outcome)
    }

    /// Passes priority in an open priority window.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn pass_priority(
        &self,
        game: &mut Game,
        cmd: PassPriorityCommand,
    ) -> Result<PassPriorityOutcome, DomainError> {
        let outcome = game.pass_priority(cmd)?;
        let domain_events = domain_events_for_pass_priority(&outcome);
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok(outcome)
    }

    /// Declares attacking creatures.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn declare_attackers(
        &self,
        game: &mut Game,
        cmd: DeclareAttackersCommand,
    ) -> Result<AttackersDeclared, DomainError> {
        let event = game.declare_attackers(cmd)?;
        self.persist_and_publish_event(game.id().as_str(), &event)?;

        Ok(event)
    }

    /// Declares blocking creatures.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn declare_blockers(
        &self,
        game: &mut Game,
        cmd: DeclareBlockersCommand,
    ) -> Result<BlockersDeclared, DomainError> {
        let event = game.declare_blockers(cmd)?;
        self.persist_and_publish_event(game.id().as_str(), &event)?;

        Ok(event)
    }

    /// Resolves combat damage.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn resolve_combat_damage(
        &self,
        game: &mut Game,
        cmd: ResolveCombatDamageCommand,
    ) -> Result<ResolveCombatDamageOutcome, DomainError> {
        let outcome = game.resolve_combat_damage(cmd)?;
        let domain_events = domain_events_for_resolve_combat_damage(&outcome);
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok(outcome)
    }
}
