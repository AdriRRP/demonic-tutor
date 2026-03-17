use crate::{
    application::{Command, EventBus, EventStore},
    domain::{
        commands::{
            AdvanceTurnCommand, CastSpellCommand, DealOpeningHandsCommand, DeclareAttackersCommand,
            DeclareBlockersCommand, DrawCardCommand, MulliganCommand, PlayCreatureCommand,
            PlayLandCommand, ResolveCombatDamageCommand, SetLifeCommand, StartGameCommand,
            TapLandCommand,
        },
        errors::DomainError,
        events::{
            AttackersDeclared, BlockersDeclared, CardDrawn, CombatDamageResolved,
            CreatureEnteredBattlefield, DomainEvent, GameStarted, LandPlayed, LandTapped,
            LifeChanged, ManaAdded, MulliganTaken, OpeningHandDealt, SpellCast, TurnAdvanced,
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

    fn persist_and_publish_events(&self, game_id: &str, events: &[DomainEvent]) {
        if !events.is_empty() {
            let _ = self.event_store.append(game_id, events);
            for event in events {
                self.event_bus.publish(event);
            }
        }
    }

    /// Executes a command using the `Command` pattern.
    ///
    /// # Errors
    /// Returns a `DomainError` if the command violates domain rules or invariants.
    pub fn execute_command<C: Command>(
        &self,
        game: &mut Game,
        command: &C,
    ) -> Result<Vec<DomainEvent>, DomainError> {
        let events = game.execute_command(command)?;
        self.persist_and_publish_events(game.id().as_str(), &events);
        Ok(events)
    }

    /// Starts a new game.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn start_game(&self, cmd: StartGameCommand) -> Result<(Game, GameStarted), DomainError> {
        let (game, event) = Game::start(cmd)?;
        let domain_event: DomainEvent = event.clone().into();

        self.persist_and_publish_events(game.id().as_str(), &[domain_event]);

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

        if !events.is_empty() {
            let domain_events: Vec<DomainEvent> = events.iter().cloned().map(Into::into).collect();
            self.persist_and_publish_events(game.id().as_str(), &domain_events);
        }

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
        let domain_event: DomainEvent = event.clone().into();

        self.persist_and_publish_events(game.id().as_str(), &[domain_event]);

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
    ) -> Result<TurnAdvanced, DomainError> {
        let (turn_event, turn_number_event, phase_event, card_drawn) = game.advance_turn(cmd)?;

        let mut domain_events = vec![
            turn_event.clone().into(),
            turn_number_event.into(),
            phase_event.into(),
        ];
        if let Some(draw_event) = card_drawn {
            domain_events.push(draw_event.into());
        }

        self.persist_and_publish_events(game.id().as_str(), &domain_events);

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
        let domain_event: DomainEvent = event.clone().into();

        self.persist_and_publish_events(game.id().as_str(), &[domain_event]);

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
        let domain_event: DomainEvent = event.clone().into();

        self.persist_and_publish_events(game.id().as_str(), &[domain_event]);

        Ok(event)
    }

    /// Sets a player's life total.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn set_life(
        &self,
        game: &mut Game,
        cmd: SetLifeCommand,
    ) -> Result<LifeChanged, DomainError> {
        let event = game.set_life(cmd)?;
        let domain_event: DomainEvent = event.clone().into();

        self.persist_and_publish_events(game.id().as_str(), &[domain_event]);

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
        self.persist_and_publish_events(game.id().as_str(), &domain_events);

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
        let domain_event: DomainEvent = event.clone().into();

        self.persist_and_publish_events(game.id().as_str(), &[domain_event]);

        Ok(event)
    }

    /// Plays a creature.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn play_creature(
        &self,
        game: &mut Game,
        cmd: PlayCreatureCommand,
    ) -> Result<CreatureEnteredBattlefield, DomainError> {
        let event = game.play_creature(cmd)?;
        let domain_event: DomainEvent = event.clone().into();

        self.persist_and_publish_events(game.id().as_str(), &[domain_event]);

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
        let domain_event: DomainEvent = event.clone().into();

        self.persist_and_publish_events(game.id().as_str(), &[domain_event]);

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
        let domain_event: DomainEvent = event.clone().into();

        self.persist_and_publish_events(game.id().as_str(), &[domain_event]);

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
    ) -> Result<CombatDamageResolved, DomainError> {
        let event = game.resolve_combat_damage(cmd)?;
        let domain_event: DomainEvent = event.clone().into();

        self.persist_and_publish_events(game.id().as_str(), &[domain_event]);

        Ok(event)
    }
}
