import { For, Show, createMemo } from "solid-js";
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

export const PregameSetupOverlay: Component<PregameSetupOverlayProps> = (props) => {
  const pregame = () => props.state.pregame;
  const localSeatId = () => props.sessionInfo?.localSeatId ?? null;
  const localViewer = () =>
    props.state.viewers.find((viewer) => viewer.player_id === localSeatId()) ??
    props.state.viewers[0];
  const localPlayerName = () =>
    formatPlayerLabel(localViewer()?.player_id ?? localSeatId() ?? "Player");
  const currentDecisionPlayerId = () => pregame()?.current_decision_player_id ?? null;
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
  const localHandCards = () => localViewer()?.hand ?? [];
  const canConfirmKeep = () => props.selectedBottomCardIds.length === cardsToBottom();
  const selectedBottomCards = createMemo(() => {
    const selectedIds = new Set(props.selectedBottomCardIds);
    return localHandCards().filter((card) => selectedIds.has(card.card_id));
  });

  const waitingForLabel = () => {
    const current = currentDecisionPlayerId();
    if (current === null) {
      return null;
    }

    return current === localSeatId() ? localPlayerName() : formatPlayerLabel(current);
  };

  return (
    <Show when={pregame()}>
      <div class="pregame-overlay">
        <section class="pregame-banner panel">
          <p class="eyebrow">Opening hand</p>
          <h2>{localGoesFirst() ? "You go first" : "Opponent goes first"}</h2>
          <p>
            <Show
              when={canAct()}
              fallback={`Waiting for ${waitingForLabel() ?? "the other player"} to choose whether to keep or mulligan.`}
            >
              <Show
                when={cardsToBottom() > 0}
                fallback="You can review your opening hand and decide whether to keep it."
              >
                Click {cardsToBottom()} card{cardsToBottom() === 1 ? "" : "s"} in your visible hand
                fan to put on the bottom, then keep your opening hand.
              </Show>
            </Show>
          </p>
        </section>

        <Show when={canAct() && cardsToBottom() > 0}>
          <section class="pregame-selection-dock panel">
            <div class="pregame-selection-copy pregame-selection-summary">
              <p class="eyebrow">London mulligan</p>
              <strong>
                Select {cardsToBottom()} card{cardsToBottom() === 1 ? "" : "s"} to bottom
              </strong>
              <span>
                Selected {props.selectedBottomCardIds.length} / {cardsToBottom()}
              </span>
            </div>
            <Show
              when={selectedBottomCards().length > 0}
              fallback={
                <p class="pregame-selection-hint">
                  Selected cards will glow in your hand and appear here as you mark them for the
                  bottom.
                </p>
              }
            >
              <div class="chip-row pregame-selection-chip-row">
                <For each={selectedBottomCards()}>
                  {(card) => (
                    <span class="chip chip-forest">{formatCardLabel(card.definition_id)}</span>
                  )}
                </For>
              </div>
            </Show>
          </section>
        </Show>

        <section class="pregame-action-dock panel">
          <div class="pregame-action-copy">
            <p class="eyebrow">Opening seven</p>
            <strong>{localHandCount()} cards ready</strong>
            <span>Mulligans taken: {mulliganCount()}</span>
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
                  Mulligan
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
                  Keep {keepCount()}
                </button>
              </>
            </Show>
          </div>
        </section>
      </div>
    </Show>
  );
};

function formatPlayerLabel(playerId: string): string {
  return playerId.replace(/[-_]/g, " ").replace(/\b\w/g, (character) => character.toUpperCase());
}

function formatCardLabel(definitionId: string): string {
  return definitionId
    .replace(/[-_]/g, " ")
    .replace(/\b\w/g, (character) => character.toUpperCase());
}
