export interface ArenaCardView {
  card_id: string;
  definition_id: string;
  card_type: string;
}

export interface ArenaHandCard {
  card_id: string;
  definition_id: string;
  card_type: string;
  mana_cost: number;
  power: number | null;
  toughness: number | null;
  loyalty: number | null;
  keywords: string[];
  requires_target: boolean;
  requires_choice: boolean;
  has_activated_ability: boolean;
  can_cast_in_open_priority: boolean;
  can_cast_in_open_priority_during_own_turn: boolean;
}

export interface ArenaBattlefieldCard extends ArenaCardView {
  tapped: boolean;
  token: boolean;
  attached_to: string | null;
  power: number | null;
  toughness: number | null;
  loyalty: number | null;
  summoning_sickness: boolean;
  attacking: boolean;
  blocking: boolean;
  keywords: string[];
}

export interface ArenaPlayerView {
  player_id: string;
  is_active: boolean;
  life: number;
  mana_total: number;
  hand_count: number;
  library_count: number;
  battlefield: ArenaBattlefieldCard[];
  graveyard: ArenaCardView[];
  exile: ArenaCardView[];
}

export interface ArenaStackObject {
  number: number;
  kind: string;
  controller_id: string | null;
  source_card_id: string | null;
  card_type: string | null;
  target: string | null;
  requires_choice: boolean;
}

export interface ArenaBlockerOption {
  blocker_id: string;
  attacker_ids: string[];
}

export interface ArenaLegalAction {
  kind: string;
  player_id: string;
  summary: string;
  card_ids: string[];
  blocker_options: ArenaBlockerOption[];
}

export interface ArenaChoicePrompt {
  kind: string;
  player_id: string;
  source_card_id: string | null;
  summary: string;
  item_ids: string[];
  options: string[];
}

export interface ArenaTimelineEntry {
  sequence: number;
  label: string;
}

export interface ArenaCommandFeedback {
  applied: boolean;
  message: string;
  emitted_events: string[];
}

export interface ArenaPregamePlayerState {
  player_id: string;
  hand_count: number;
  is_starting_player: boolean;
  is_current: boolean;
  mulligan_used: boolean;
  kept: boolean;
  can_mulligan: boolean;
}

export interface ArenaPregameState {
  starting_player_id: string;
  current_player_id: string;
  players: ArenaPregamePlayerState[];
}

export interface ArenaGameView {
  game_id: string;
  playable_subset_version: string;
  active_player_id: string | null;
  phase: string;
  turn_number: number;
  priority_holder: string | null;
  priority_has_pending_pass: boolean | null;
  is_over: boolean;
  winner_id: string | null;
  loser_id: string | null;
  end_reason: string | null;
  players: ArenaPlayerView[];
  stack: ArenaStackObject[];
}

export interface ArenaViewerState {
  player_id: string;
  is_active: boolean;
  is_priority_holder: boolean;
  hand: ArenaHandCard[];
  legal_actions: ArenaLegalAction[];
  choice_requests: ArenaChoicePrompt[];
}

export interface ArenaState {
  game: ArenaGameView;
  viewers: ArenaViewerState[];
  event_log: ArenaTimelineEntry[];
  pregame: ArenaPregameState | null;
  last_command: ArenaCommandFeedback | null;
}

export interface BlockerAssignmentInput {
  blocker_id: string;
  attacker_id: string;
}
