use crate::domain::{
    commands::{
        AdvanceTurnCommand, CastSpellCommand, DealOpeningHandsCommand, DeclareAttackersCommand,
        DeclareBlockersCommand, DrawCardCommand, MulliganCommand, PlayCreatureCommand,
        PlayLandCommand, SetLifeCommand, StartGameCommand, TapLandCommand,
    },
    errors::DomainError,
    events::DomainEvent,
    game::Game,
};

/// A command that can be executed on a Game aggregate.
///
/// This trait represents user intents at the application layer, not domain concepts.
/// Commands are processed by the application layer and translated into domain operations.
pub trait Command {
    /// Executes the command on the provided Game aggregate.
    ///
    /// # Errors
    /// Returns a `DomainError` if the command violates domain rules or invariants.
    fn execute(&self, game: &mut Game) -> Result<Vec<DomainEvent>, DomainError>;
}

impl Command for StartGameCommand {
    fn execute(&self, game: &mut Game) -> Result<Vec<DomainEvent>, DomainError> {
        let (new_game, event) = Game::start(self.clone())?;
        *game = new_game;
        Ok(vec![event.into()])
    }
}

impl Command for DealOpeningHandsCommand {
    fn execute(&self, game: &mut Game) -> Result<Vec<DomainEvent>, DomainError> {
        game.deal_opening_hands(self)
            .map(|events| events.into_iter().map(Into::into).collect())
    }
}

impl Command for PlayLandCommand {
    fn execute(&self, game: &mut Game) -> Result<Vec<DomainEvent>, DomainError> {
        game.play_land(self.clone()).map(|event| vec![event.into()])
    }
}

impl Command for AdvanceTurnCommand {
    fn execute(&self, game: &mut Game) -> Result<Vec<DomainEvent>, DomainError> {
        let (turn_event, turn_number_event, phase_event, card_drawn) =
            game.advance_turn(self.clone())?;
        let mut events = vec![
            turn_event.into(),
            turn_number_event.into(),
            phase_event.into(),
        ];
        if let Some(draw_event) = card_drawn {
            events.push(draw_event.into());
        }
        Ok(events)
    }
}

impl Command for DrawCardCommand {
    fn execute(&self, game: &mut Game) -> Result<Vec<DomainEvent>, DomainError> {
        game.draw_card(self.clone()).map(|event| vec![event.into()])
    }
}

impl Command for MulliganCommand {
    fn execute(&self, game: &mut Game) -> Result<Vec<DomainEvent>, DomainError> {
        game.mulligan(self.clone()).map(|event| vec![event.into()])
    }
}

impl Command for SetLifeCommand {
    fn execute(&self, game: &mut Game) -> Result<Vec<DomainEvent>, DomainError> {
        game.set_life(self.clone()).map(|event| vec![event.into()])
    }
}

impl Command for TapLandCommand {
    fn execute(&self, game: &mut Game) -> Result<Vec<DomainEvent>, DomainError> {
        let (land_event, mana_event) = game.tap_land(self.clone())?;
        Ok(vec![land_event.into(), mana_event.into()])
    }
}

impl Command for CastSpellCommand {
    fn execute(&self, game: &mut Game) -> Result<Vec<DomainEvent>, DomainError> {
        game.cast_spell(self.clone())
            .map(|event| vec![event.into()])
    }
}

impl Command for PlayCreatureCommand {
    fn execute(&self, game: &mut Game) -> Result<Vec<DomainEvent>, DomainError> {
        game.play_creature(self.clone())
            .map(|event| vec![event.into()])
    }
}

impl Command for DeclareAttackersCommand {
    fn execute(&self, game: &mut Game) -> Result<Vec<DomainEvent>, DomainError> {
        game.declare_attackers(self.clone())
            .map(|event| vec![event.into()])
    }
}

impl Command for DeclareBlockersCommand {
    fn execute(&self, game: &mut Game) -> Result<Vec<DomainEvent>, DomainError> {
        game.declare_blockers(self.clone())
            .map(|event| vec![event.into()])
    }
}
