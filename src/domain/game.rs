use crate::domain::{
    cards::{CardInstance, CardType},
    commands::{
        AdvanceTurnCommand, DealOpeningHandsCommand, DrawCardCommand, MulliganCommand,
        PlayLandCommand, StartGameCommand, TapLandCommand,
    },
    errors::DomainError,
    events::{
        CardDrawn, GameStarted, LandPlayed, LandTapped, LifeChanged, ManaAdded, MulliganTaken,
        OpeningHandDealt, PhaseChanged, TurnAdvanced, TurnNumberChanged,
    },
    ids::{CardInstanceId, DeckId, GameId, PlayerId},
    zones::{Battlefield, Hand, Library},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    Setup,
    Beginning,
    Main,
    Ending,
}

mod player {
    use super::{Battlefield, DeckId, Hand, Library, PlayerId};

    const DEFAULT_STARTING_LIFE: u32 = 20;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Player {
        id: PlayerId,
        deck_id: DeckId,
        library: Library,
        hand: Hand,
        battlefield: Battlefield,
        life: u32,
        mana: u32,
        lands_played_this_turn: usize,
        mulligan_used: bool,
    }

    impl Player {
        pub const fn new(id: PlayerId, deck_id: DeckId) -> Self {
            Self {
                id,
                deck_id,
                library: Library::new(Vec::new()),
                hand: Hand::new(),
                battlefield: Battlefield::new(),
                life: DEFAULT_STARTING_LIFE,
                mana: 0,
                lands_played_this_turn: 0,
                mulligan_used: false,
            }
        }

        #[must_use]
        pub const fn id(&self) -> &PlayerId {
            &self.id
        }

        #[must_use]
        pub const fn deck_id(&self) -> &DeckId {
            &self.deck_id
        }

        #[must_use]
        pub const fn hand(&self) -> &Hand {
            &self.hand
        }

        #[must_use]
        pub const fn library(&self) -> &Library {
            &self.library
        }

        #[must_use]
        pub const fn battlefield(&self) -> &Battlefield {
            &self.battlefield
        }

        #[must_use]
        pub const fn life(&self) -> u32 {
            self.life
        }

        pub const fn life_mut(&mut self) -> &mut u32 {
            &mut self.life
        }

        #[must_use]
        pub const fn mana(&self) -> u32 {
            self.mana
        }

        pub const fn mana_mut(&mut self) -> &mut u32 {
            &mut self.mana
        }

        #[must_use]
        pub const fn lands_played_this_turn(&self) -> usize {
            self.lands_played_this_turn
        }

        pub const fn library_mut(&mut self) -> &mut Library {
            &mut self.library
        }

        pub const fn hand_mut(&mut self) -> &mut Hand {
            &mut self.hand
        }

        pub const fn battlefield_mut(&mut self) -> &mut Battlefield {
            &mut self.battlefield
        }

        pub const fn lands_played_this_turn_mut(&mut self) -> &mut usize {
            &mut self.lands_played_this_turn
        }

        #[must_use]
        pub const fn mulligan_used(&self) -> bool {
            self.mulligan_used
        }

        pub const fn mulligan_used_mut(&mut self) -> &mut bool {
            &mut self.mulligan_used
        }
    }
}

use player::Player;

#[derive(Debug)]
pub struct Game {
    id: GameId,
    active_player: PlayerId,
    phase: Phase,
    turn_number: u32,
    players: Vec<Player>,
}

impl Game {
    #[must_use]
    pub const fn id(&self) -> &GameId {
        &self.id
    }

    #[must_use]
    pub const fn active_player(&self) -> &PlayerId {
        &self.active_player
    }

    #[must_use]
    pub const fn phase(&self) -> &Phase {
        &self.phase
    }

    #[must_use]
    pub const fn turn_number(&self) -> u32 {
        self.turn_number
    }

    #[must_use]
    pub fn players(&self) -> &[Player] {
        &self.players
    }

    /// Starts a new game with the given command.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Player count is less than 2
    /// - Player count is more than 2
    /// - There are duplicate players
    ///
    /// # Panics
    ///
    /// Panics if the player list is empty after validation (internal invariant violation).
    pub fn start(cmd: StartGameCommand) -> Result<(Self, GameStarted), DomainError> {
        let player_count = cmd.players.len();

        if player_count < 2 {
            return Err(DomainError::NotEnoughPlayers {
                actual: player_count,
            });
        }

        if player_count > 2 {
            return Err(DomainError::TooManyPlayers {
                actual: player_count,
            });
        }

        let mut seen_players = std::collections::HashSet::new();
        let mut players = Vec::new();
        let mut player_ids = Vec::new();

        for pd in &cmd.players {
            if !seen_players.insert(pd.player_id.clone()) {
                return Err(DomainError::DuplicatePlayer(pd.player_id.clone()));
            }
            players.push(Player::new(pd.player_id.clone(), pd.deck_id.clone()));
            player_ids.push(pd.player_id.clone());
        }

        let game_started = GameStarted::new(cmd.game_id.clone(), player_ids.clone());

        let active_player = player_ids.into_iter().next().ok_or_else(|| {
            DomainError::InternalInvariantViolation {
                message: "player list should not be empty after validation".to_string(),
            }
        })?;

        let game = Self {
            id: cmd.game_id,
            active_player,
            phase: Phase::Setup,
            turn_number: 1,
            players,
        };

        Ok((game, game_started))
    }

    /// Deals opening hands to all players.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - A player is not found in the game
    /// - A player's library has fewer than 7 cards
    ///
    /// # Panics
    ///
    /// Panics if a validated player is not found (internal invariant violation).
    pub fn deal_opening_hands(
        &mut self,
        cmd: &DealOpeningHandsCommand,
    ) -> Result<Vec<OpeningHandDealt>, DomainError> {
        let hand_size = 7;

        for pc in &cmd.player_cards {
            let player_exists = self.players.iter().any(|p| p.id() == &pc.player_id);
            if !player_exists {
                return Err(DomainError::PlayerNotFound(pc.player_id.clone()));
            }
        }

        for pc in &cmd.player_cards {
            if pc.cards.len() < hand_size {
                return Err(DomainError::NotEnoughCardsInLibrary {
                    player_id: pc.player_id.clone(),
                    available: pc.cards.len(),
                    requested: hand_size,
                });
            }
        }

        let mut events: Vec<OpeningHandDealt> = Vec::new();

        for pc in &cmd.player_cards {
            let idx = self
                .players
                .iter()
                .position(|p| p.id() == &pc.player_id)
                .ok_or_else(|| DomainError::InternalInvariantViolation {
                    message: format!("player {} should exist after validation", pc.player_id.0),
                })?;

            let player_id_owned = pc.player_id.clone();

            let cards: Vec<CardInstance> = pc
                .cards
                .iter()
                .take(hand_size)
                .enumerate()
                .map(|(i, (def_id, card_type))| {
                    CardInstance::new(
                        CardInstanceId::new(format!("{}-{}-{}", self.id.0, player_id_owned.0, i)),
                        def_id.clone(),
                        card_type.clone(),
                    )
                })
                .collect();

            let library_cards: Vec<CardInstance> = pc
                .cards
                .iter()
                .skip(hand_size)
                .enumerate()
                .map(|(i, (def_id, card_type))| {
                    CardInstance::new(
                        CardInstanceId::new(format!(
                            "{}-{}-lib-{}",
                            self.id.0, player_id_owned.0, i
                        )),
                        def_id.clone(),
                        card_type.clone(),
                    )
                })
                .collect();

            let player = &mut self.players[idx];
            player.hand_mut().receive(cards.clone());
            *player.library_mut() = Library::new(library_cards);

            events.push(OpeningHandDealt::new(
                self.id.clone(),
                player_id_owned,
                cards.into_iter().map(|c| c.id().clone()).collect(),
            ));
        }

        Ok(events)
    }

    /// Performs a mulligan for a player.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The player is not found
    /// - The player has already used mulligan
    /// - The current phase is not Setup
    /// - The library has fewer than 7 cards
    pub fn mulligan(&mut self, cmd: MulliganCommand) -> Result<MulliganTaken, DomainError> {
        if !matches!(self.phase, Phase::Setup) {
            return Err(DomainError::InvalidPhaseForMulligan);
        }

        let player_idx = self
            .players
            .iter()
            .position(|p| p.id() == &cmd.player_id)
            .ok_or_else(|| DomainError::PlayerNotFound(cmd.player_id.clone()))?;

        let player = &mut self.players[player_idx];

        if player.mulligan_used() {
            return Err(DomainError::MulliganAlreadyUsed {
                player_id: cmd.player_id,
            });
        }

        let hand_size = 7;
        if player.library().len() < hand_size {
            return Err(DomainError::NotEnoughCardsInLibrary {
                player_id: cmd.player_id,
                available: player.library().len(),
                requested: hand_size,
            });
        }

        let hand_cards: Vec<CardInstance> = player.hand().cards().to_vec();
        player.hand_mut().receive(Vec::new());
        player.library_mut().receive(hand_cards);
        player.library_mut().shuffle();

        let drawn_cards = player.library_mut().draw(hand_size).ok_or_else(|| {
            DomainError::NotEnoughCardsInLibrary {
                player_id: cmd.player_id.clone(),
                available: player.library().len(),
                requested: hand_size,
            }
        })?;

        player.hand_mut().receive(drawn_cards);
        *player.mulligan_used_mut() = true;

        Ok(MulliganTaken::new(self.id.clone(), cmd.player_id))
    }

    /// Plays a land card from the player's hand to the battlefield.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The player is not the active player
    /// - The current phase is not the main phase
    /// - The player is not found
    /// - The player has already played a land this turn
    /// - The card is not in the player's hand
    /// - The card is not a land card
    pub fn play_land(&mut self, cmd: PlayLandCommand) -> Result<LandPlayed, DomainError> {
        if self.active_player != cmd.player_id {
            return Err(DomainError::NotYourTurn {
                current_player: self.active_player.clone(),
                requested_player: cmd.player_id,
            });
        }

        if !matches!(self.phase, Phase::Main) {
            return Err(DomainError::InvalidPhaseForLand);
        }

        let player_idx = self
            .players
            .iter()
            .position(|p| p.id() == &cmd.player_id)
            .ok_or_else(|| DomainError::PlayerNotFound(cmd.player_id.clone()))?;

        let player = &mut self.players[player_idx];

        if player.lands_played_this_turn() > 0 {
            return Err(DomainError::AlreadyPlayedLandThisTurn {
                player_id: cmd.player_id,
            });
        }

        let card_id = cmd.card_id.clone();

        let card =
            player
                .hand_mut()
                .remove(&card_id)
                .ok_or_else(|| DomainError::CardNotInHand {
                    player_id: cmd.player_id.clone(),
                    card_id: card_id.clone(),
                })?;

        if !matches!(card.card_type(), CardType::Land) {
            return Err(DomainError::NotALand { card_id });
        }

        player.battlefield_mut().add(card);
        *player.lands_played_this_turn_mut() += 1;

        Ok(LandPlayed::new(self.id.clone(), cmd.player_id, card_id))
    }

    /// Advances the turn to the next phase or player.
    ///
    /// Phase progression: Setup → Main → Ending → (next player) Main
    /// Beginning is skipped for simplicity - game goes directly to Main phase.
    ///
    /// # Errors
    ///
    /// Returns an error if the active player cannot be found (internal invariant violation).
    pub fn advance_turn(
        &mut self,
        _cmd: AdvanceTurnCommand,
    ) -> Result<(TurnAdvanced, TurnNumberChanged, PhaseChanged), DomainError> {
        let from_phase = self.phase;

        let (to_phase, change_player) = match &self.phase {
            Phase::Setup | Phase::Ending => (Phase::Main, true),
            Phase::Main => (Phase::Ending, false),
            Phase::Beginning => (Phase::Main, false),
        };

        // Reset lands played when starting a new turn
        if matches!(&self.phase, Phase::Setup | Phase::Ending) {
            for player in &mut self.players {
                *player.lands_played_this_turn_mut() = 0;
            }
        }

        if change_player {
            let current_idx = self
                .players
                .iter()
                .position(|p| p.id() == &self.active_player)
                .ok_or_else(|| DomainError::InternalInvariantViolation {
                    message: "active player should exist in player list".to_string(),
                })?;
            let next_idx = (current_idx + 1) % self.players.len();
            self.active_player = self.players[next_idx].id().clone();

            for player in &mut self.players {
                *player.lands_played_this_turn_mut() = 0;
            }

            let from = self.turn_number;
            self.turn_number += 1;

            self.phase = to_phase;

            return Ok((
                TurnAdvanced::new(self.id.clone(), self.active_player.clone()),
                TurnNumberChanged::new(self.id.clone(), from, self.turn_number),
                PhaseChanged::new(self.id.clone(), from_phase, to_phase),
            ));
        }

        self.phase = to_phase;

        Ok((
            TurnAdvanced::new(self.id.clone(), self.active_player.clone()),
            TurnNumberChanged::new(self.id.clone(), self.turn_number, self.turn_number),
            PhaseChanged::new(self.id.clone(), from_phase, to_phase),
        ))
    }

    /// Draws a card from the player's library to their hand.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The player is not the active player
    /// - The current phase is not the main phase
    /// - The player is not found
    /// - The player's library is empty
    ///
    /// # Panics
    ///
    /// Panics if the library returns fewer cards than requested after validation.
    pub fn draw_card(&mut self, cmd: DrawCardCommand) -> Result<CardDrawn, DomainError> {
        if self.active_player != cmd.player_id {
            return Err(DomainError::NotYourTurn {
                current_player: self.active_player.clone(),
                requested_player: cmd.player_id,
            });
        }

        if !matches!(self.phase, Phase::Main | Phase::Setup) {
            return Err(DomainError::InvalidPhaseForDraw { phase: self.phase });
        }

        let player_idx = self
            .players
            .iter()
            .position(|p| p.id() == &cmd.player_id)
            .ok_or_else(|| DomainError::PlayerNotFound(cmd.player_id.clone()))?;

        let player = &mut self.players[player_idx];

        let drawn_cards =
            player
                .library_mut()
                .draw(1)
                .ok_or_else(|| DomainError::NotEnoughCardsInLibrary {
                    player_id: cmd.player_id.clone(),
                    available: player.library().len(),
                    requested: 1,
                })?;

        let card = drawn_cards.into_iter().next().ok_or_else(|| {
            DomainError::InternalInvariantViolation {
                message: "draw(1) should return exactly one card".to_string(),
            }
        })?;
        let card_id = card.id().clone();

        player.hand_mut().receive(vec![card]);

        Ok(CardDrawn::new(self.id.clone(), cmd.player_id, card_id))
    }

    /// Modifies a player's life total by the given amount.
    /// Positive values gain life, negative values lose life.
    ///
    /// # Errors
    ///
    /// Returns an error if the player is not found.
    pub fn set_life(
        &mut self,
        cmd: crate::domain::commands::SetLifeCommand,
    ) -> Result<LifeChanged, DomainError> {
        let player_idx = self
            .players
            .iter()
            .position(|p| p.id() == &cmd.player_id)
            .ok_or_else(|| DomainError::PlayerNotFound(cmd.player_id.clone()))?;

        let player = &mut self.players[player_idx];
        let from_life = player.life();
        let to_life = from_life.saturating_add_signed(cmd.life_change);

        *player.life_mut() = to_life;

        Ok(LifeChanged::new(
            self.id.clone(),
            cmd.player_id,
            from_life,
            to_life,
        ))
    }

    /// Taps a land to add mana.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The player is not found
    /// - The card is not on the battlefield
    /// - The card is already tapped
    /// - The card is not a land
    pub fn tap_land(
        &mut self,
        cmd: TapLandCommand,
    ) -> Result<(LandTapped, ManaAdded), DomainError> {
        let player_idx = self
            .players
            .iter()
            .position(|p| p.id() == &cmd.player_id)
            .ok_or_else(|| DomainError::PlayerNotFound(cmd.player_id.clone()))?;

        let player = &mut self.players[player_idx];

        let card = player
            .battlefield_mut()
            .card_mut(&cmd.card_id)
            .ok_or_else(|| DomainError::CardNotOnBattlefield {
                player_id: cmd.player_id.clone(),
                card_id: cmd.card_id.clone(),
            })?;

        if card.is_tapped() {
            return Err(DomainError::CardAlreadyTapped {
                player_id: cmd.player_id.clone(),
                card_id: cmd.card_id.clone(),
            });
        }

        if !matches!(card.card_type(), CardType::Land) {
            return Err(DomainError::NotALand {
                card_id: cmd.card_id.clone(),
            });
        }

        card.tap();
        *player.mana_mut() += 1;
        let new_mana = player.mana();

        Ok((
            LandTapped::new(self.id.clone(), cmd.player_id.clone(), cmd.card_id.clone()),
            ManaAdded::new(self.id.clone(), cmd.player_id, 1, new_mana),
        ))
    }
}
