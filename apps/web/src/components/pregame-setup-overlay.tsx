import { Show, createMemo } from "solid-js";
import type { Component } from "solid-js";
import { keepOpeningHand, mulliganOpeningHand, type ArenaCommandTarget } from "../lib/runtime";
import type { ArenaSessionInfo } from "../lib/session";
import type { ArenaState } from "../lib/types";

interface PregameSetupOverlayProps {
  state: ArenaState;
  sessionInfo: ArenaSessionInfo | null;
  selectedBottomCardIds: string[];
  onRun: (operation: (current: ArenaCommandTarget) => Promise<ArenaState>) => void;
}

interface PregameHeroState {
  tone: "bottoming" | "deciding" | "waiting";
  title: string;
  body: string;
  spotlight: string;
}

export const PregameSetupOverlay: Component<PregameSetupOverlayProps> = (props) => {
  const pregame = () => props.state.pregame;
  const localSeatId = () => props.sessionInfo?.localSeatId ?? null;
  const localViewer = () =>
    props.state.viewers.find((viewer) => viewer.player_id === localSeatId()) ??
    props.state.viewers[0];
  const opponentViewer = () =>
    props.state.viewers.find((viewer) => viewer.player_id !== localViewer()?.player_id) ?? null;
  const localPlayerName = () =>
    formatPlayerLabel(localViewer()?.player_id ?? localSeatId() ?? "Player");
  const opponentPlayerName = () => formatPlayerLabel(opponentViewer()?.player_id ?? "Opponent");
  const currentDecisionPlayerId = () => pregame()?.current_decision_player_id ?? null;
  const keptPlayerIds = () => new Set(pregame()?.kept_player_ids ?? []);
  const canAct = () => {
    const viewer = localViewer();
    const current = currentDecisionPlayerId();
    return viewer !== undefined && current !== null && viewer.player_id === current;
  };
  const localGoesFirst = () => {
    const currentPregame = pregame();
    const currentSeatId = localSeatId();
    return (
      currentPregame !== null &&
      currentSeatId !== null &&
      currentPregame.starting_player_id === currentSeatId
    );
  };
  const localHandCount = () => localViewer()?.hand.length ?? 0;
  const mulliganCount = () => localViewer()?.mulligan_count ?? 0;
  const cardsToBottom = () => (canAct() ? (pregame()?.current_bottom_count ?? 0) : 0);
  const keepCount = () => Math.max(localHandCount() - cardsToBottom(), 0);
  const canConfirmKeep = () => props.selectedBottomCardIds.length === cardsToBottom();
  const selectionProgress = createMemo(() =>
    cardsToBottom() === 0 ? 0 : props.selectedBottomCardIds.length / cardsToBottom(),
  );
  const waitingForLabel = () => {
    const current = currentDecisionPlayerId();
    if (current === null) {
      return null;
    }

    return current === localSeatId() ? localPlayerName() : formatPlayerLabel(current);
  };
  const heroState = createMemo<PregameHeroState>(() => {
    if (canAct() && cardsToBottom() > 0) {
      return {
        tone: "bottoming",
        title: `Choose ${String(cardsToBottom())} card${cardsToBottom() === 1 ? "" : "s"} to put back`,
        body: `Mark the cards you want to place on the bottom of your library, then keep ${String(keepCount())}.`,
        spotlight: "Selection live",
      };
    }

    if (canAct()) {
      return {
        tone: "deciding",
        title: localGoesFirst() ? "You go first" : "Opponent goes first",
        body: "Review your opening hand and decide whether to keep it or take a mulligan.",
        spotlight: "Your decision",
      };
    }

    if (localGoesFirst()) {
      return {
        tone: "waiting",
        title: "You are on the play",
        body: `Your hand stays visible while ${waitingForLabel() ?? "the other player"} finishes their opening-hand choice.`,
        spotlight: "Waiting",
      };
    }

    return {
      tone: "waiting",
      title: "You are on the draw",
      body: `Your hand stays visible while ${waitingForLabel() ?? "the other player"} finishes their opening-hand choice.`,
      spotlight: "Waiting",
    };
  });
  const localSeatState = createMemo(() =>
    describePregameSeat({
      viewerPlayerId: localViewer()?.player_id ?? null,
      viewerName: localPlayerName(),
      currentDecisionPlayerId: currentDecisionPlayerId(),
      keptPlayerIds: keptPlayerIds(),
      bottomCount: cardsToBottom(),
      canAct: canAct(),
      mulliganCount: mulliganCount(),
      local: true,
    }),
  );
  const opponentSeatState = createMemo(() =>
    describePregameSeat({
      viewerPlayerId: opponentViewer()?.player_id ?? null,
      viewerName: opponentPlayerName(),
      currentDecisionPlayerId: currentDecisionPlayerId(),
      keptPlayerIds: keptPlayerIds(),
      bottomCount:
        currentDecisionPlayerId() === opponentViewer()?.player_id
          ? (pregame()?.current_bottom_count ?? 0)
          : 0,
      canAct: false,
      mulliganCount: opponentViewer()?.mulligan_count ?? 0,
      local: false,
    }),
  );

  return (
    <Show when={pregame()}>
      <div class="pregame-overlay">
        <section class={`pregame-hero panel tone-${heroState().tone}`}>
          <div class="pregame-hero-ornament pregame-hero-ornament-left" aria-hidden="true" />
          <div class="pregame-hero-ornament pregame-hero-ornament-right" aria-hidden="true" />
          <div class="pregame-hero-topline">
            <p class="eyebrow">Opening hand</p>
            <span class="pregame-hero-spotlight">{heroState().spotlight}</span>
          </div>
          <div class="pregame-hero-centerline">
            <div class={`pregame-hero-sigil tone-${heroState().tone}`} aria-hidden="true">
              <span>{heroGlyphForTone(heroState().tone)}</span>
            </div>
            <div class="pregame-hero-copy">
              <h2>{heroState().title}</h2>
              <p>{heroState().body}</p>
            </div>
          </div>
          <div class="pregame-hero-metrics">
            <div class="pregame-hero-metric">
              <span>Seat</span>
              <strong>{localGoesFirst() ? "On the play" : "On the draw"}</strong>
            </div>
            <div class="pregame-hero-metric">
              <span>Hand</span>
              <strong>{localHandCount()} cards</strong>
            </div>
            <div class="pregame-hero-metric">
              <span>Mulligans</span>
              <strong>{mulliganCount()}</strong>
            </div>
          </div>
          <div class="pregame-seat-status-row">
            <PregameSeatStateCard state={opponentSeatState()} />
            <div class="pregame-seat-status-divider">VS</div>
            <PregameSeatStateCard state={localSeatState()} />
          </div>
        </section>

        <Show when={canAct() && cardsToBottom() > 0}>
          <section class="pregame-selection-dock panel tone-bottoming">
            <div class="pregame-selection-copy">
              <p class="eyebrow">London mulligan</p>
              <strong class="pregame-selection-title">
                Select {cardsToBottom()} card{cardsToBottom() === 1 ? "" : "s"} to bottom
              </strong>
              <span class="pregame-selection-meter">
                Selected {props.selectedBottomCardIds.length} / {cardsToBottom()}
              </span>
            </div>
            <div class="pregame-selection-progress" aria-hidden="true">
              <span style={{ width: `${String(selectionProgress() * 100)}%` }} />
            </div>
            <p class="pregame-selection-hint">
              The hand fan is the source of truth here: marked cards keep numbered seals directly on
              the cards you send to the bottom.
            </p>
          </section>
        </Show>

        <section class={`pregame-action-dock panel tone-${heroState().tone}`}>
          <div class={`pregame-action-rune tone-${heroState().tone}`} aria-hidden="true">
            <span>{heroGlyphForTone(heroState().tone)}</span>
          </div>
          <div class="pregame-action-copy">
            <p class="eyebrow">Opening hand</p>
            <strong>{localHandCount()} cards in view</strong>
            <span>
              {canAct()
                ? "Your device can act in this step."
                : `Waiting for ${waitingForLabel() ?? "the other player"} to act.`}
            </span>
          </div>

          <div class="pregame-action-buttons">
            <Show when={canAct()}>
              <>
                <button
                  class="hero-button hero-button-ghost"
                  type="button"
                  onClick={() => {
                    const playerId = localViewer()?.player_id;
                    if (!playerId) {
                      return;
                    }
                    props.onRun((current) => mulliganOpeningHand(current, playerId));
                  }}
                >
                  <span class="pregame-action-button-icon" aria-hidden="true">
                    ↺
                  </span>
                  <span class="pregame-action-button-copy">
                    <strong>Mulligan</strong>
                    <span>Draw a fresh seven</span>
                  </span>
                </button>
                <button
                  class="hero-button"
                  disabled={!canConfirmKeep()}
                  type="button"
                  onClick={() => {
                    const playerId = localViewer()?.player_id;
                    if (!playerId || !canConfirmKeep()) {
                      return;
                    }
                    props.onRun((current) =>
                      keepOpeningHand(current, playerId, props.selectedBottomCardIds),
                    );
                  }}
                >
                  <span class="pregame-action-button-icon" aria-hidden="true">
                    ✦
                  </span>
                  <span class="pregame-action-button-copy">
                    <strong>Keep {keepCount()}</strong>
                    <span>
                      {cardsToBottom() > 0 ? "Lock in your bottom picks" : "Start the duel"}
                    </span>
                  </span>
                </button>
              </>
            </Show>
          </div>
        </section>
      </div>
    </Show>
  );
};

interface PregameSeatDescriptor {
  label: string;
  state: string;
  detail: string;
  accent: "deciding" | "waiting" | "kept";
}

const PregameSeatStateCard: Component<{ state: PregameSeatDescriptor }> = (props) => (
  <article class={`pregame-seat-state pregameseat-${props.state.accent}`}>
    <span class="pregame-seat-state-icon" aria-hidden="true">
      {seatGlyphForAccent(props.state.accent)}
    </span>
    <span class="pregame-seat-state-role">{props.state.label}</span>
    <strong>{props.state.state}</strong>
    <span>{props.state.detail}</span>
  </article>
);

function describePregameSeat(input: {
  viewerPlayerId: string | null;
  viewerName: string;
  currentDecisionPlayerId: string | null;
  keptPlayerIds: Set<string>;
  bottomCount: number;
  canAct: boolean;
  mulliganCount: number;
  local: boolean;
}): PregameSeatDescriptor {
  if (input.viewerPlayerId !== null && input.keptPlayerIds.has(input.viewerPlayerId)) {
    return {
      label: input.local ? "You" : input.viewerName,
      state: "Kept",
      detail: `Mulligans ${String(input.mulliganCount)}`,
      accent: "kept",
    };
  }

  if (input.canAct) {
    return {
      label: input.local ? "You" : input.viewerName,
      state: input.bottomCount > 0 ? "Bottoming" : "Deciding",
      detail:
        input.bottomCount > 0
          ? `${String(input.bottomCount)} card${input.bottomCount === 1 ? "" : "s"} to choose`
          : `Mulligans ${String(input.mulliganCount)}`,
      accent: "deciding",
    };
  }

  if (input.viewerPlayerId !== null && input.viewerPlayerId === input.currentDecisionPlayerId) {
    return {
      label: input.local ? "You" : input.viewerName,
      state: input.bottomCount > 0 ? "Bottoming" : "Deciding",
      detail:
        input.bottomCount > 0
          ? `${String(input.bottomCount)} card${input.bottomCount === 1 ? "" : "s"} to choose`
          : `Mulligans ${String(input.mulliganCount)}`,
      accent: "deciding",
    };
  }

  return {
    label: input.local ? "You" : input.viewerName,
    state: "Waiting",
    detail: `Mulligans ${String(input.mulliganCount)}`,
    accent: "waiting",
  };
}

function formatPlayerLabel(playerId: string): string {
  return playerId.replace(/[-_]/g, " ").replace(/\b\w/g, (character) => character.toUpperCase());
}

function heroGlyphForTone(tone: "bottoming" | "deciding" | "waiting"): string {
  switch (tone) {
    case "bottoming":
      return "✦";
    case "deciding":
      return "◈";
    case "waiting":
      return "☾";
  }
}

function seatGlyphForAccent(accent: PregameSeatDescriptor["accent"]): string {
  switch (accent) {
    case "deciding":
      return "✦";
    case "kept":
      return "✓";
    case "waiting":
      return "◌";
  }
}
