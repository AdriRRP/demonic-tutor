export interface DemoCardView {
  card_id: string;
  definition_id: string;
  card_type: string;
}

export interface DemoBattlefieldCard extends DemoCardView {
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

export interface DemoPlayerView {
  player_id: string;
  is_active: boolean;
  life: number;
  mana_total: number;
  hand_count: number;
  library_count: number;
  battlefield: DemoBattlefieldCard[];
  graveyard: DemoCardView[];
  exile: DemoCardView[];
}

export interface DemoStackObject {
  number: number;
  kind: string;
  controller_id: string | null;
  source_card_id: string | null;
  card_type: string | null;
  target: string | null;
  requires_choice: boolean;
}

export interface DemoLegalAction {
  kind: string;
  player_id: string;
  summary: string;
  card_ids: string[];
}

export interface DemoChoicePrompt {
  kind: string;
  player_id: string;
  source_card_id: string | null;
  summary: string;
  item_ids: string[];
}

export interface DemoTimelineEntry {
  sequence: number;
  label: string;
}

export interface DemoCommandFeedback {
  applied: boolean;
  message: string;
  emitted_events: string[];
}

export interface DemoGameView {
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
  players: DemoPlayerView[];
  stack: DemoStackObject[];
}

export interface DemoState {
  game: DemoGameView;
  legal_actions: DemoLegalAction[];
  choice_requests: DemoChoicePrompt[];
  event_log: DemoTimelineEntry[];
  last_command: DemoCommandFeedback | null;
}
