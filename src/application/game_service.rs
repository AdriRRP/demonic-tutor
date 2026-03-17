use crate::{
    application::{EventBus, EventStore},
    domain::play::{
        commands::{
            AdjustLifeCommand, AdvanceTurnCommand, CastSpellCommand, DealOpeningHandsCommand,
            DeclareAttackersCommand, DeclareBlockersCommand, DrawCardCommand, MulliganCommand,
            PlayLandCommand, ResolveCombatDamageCommand, StartGameCommand, TapLandCommand,
        },
        errors::{DomainError, GameError},
        events::{
            AttackersDeclared, BlockersDeclared, CardDrawn, CombatDamageResolved, CreatureDied,
            DomainEvent, GameStarted, LandPlayed, LandTapped, LifeChanged, ManaAdded,
            MulliganTaken, OpeningHandDealt, SpellCast, TurnProgressed,
        },
        game::Game,
    },
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
    ) -> Result<TurnProgressed, DomainError> {
        let (turn_event, card_drawn) = game.advance_turn(cmd)?;

        let mut domain_events = vec![turn_event.clone().into()];
        if let Some(draw_event) = card_drawn {
            domain_events.push(draw_event.into());
        }

        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok(turn_event)
    }

    /// Draws a card from the player's library.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn draw_card(
        &self,
        game: &mut Game,
        cmd: DrawCardCommand,
    ) -> Result<CardDrawn, DomainError> {
        let event = game.draw_card(cmd)?;
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

    /// Adjusts a player's life total by a signed delta.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn adjust_life(
        &self,
        game: &mut Game,
        cmd: AdjustLifeCommand,
    ) -> Result<LifeChanged, DomainError> {
        let event = game.adjust_life(cmd)?;
        self.persist_and_publish_event(game.id().as_str(), &event)?;

        Ok(event)
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
        let domain_events = vec![land_event.clone().into(), mana_event.clone().into()];
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
    ) -> Result<SpellCast, DomainError> {
        let event = game.cast_spell(cmd)?;
        self.persist_and_publish_event(game.id().as_str(), &event)?;

        Ok(event)
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
    ) -> Result<(CombatDamageResolved, Vec<CreatureDied>), DomainError> {
        let (damage_event, destroyed_events) = game.resolve_combat_damage(cmd)?;
        let mut domain_events = vec![damage_event.clone().into()];
        domain_events.extend(
            destroyed_events
                .iter()
                .cloned()
                .map(DomainEvent::CreatureDied),
        );
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok((damage_event, destroyed_events))
    }
}
