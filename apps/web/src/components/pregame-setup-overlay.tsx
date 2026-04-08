import { For, Show, createEffect, createMemo, createSignal } from "solid-js";
import type { Component } from "solid-js";
import { keepOpeningHand, mulliganOpeningHand, type ArenaCommandTarget } from "../lib/runtime";
import type { ArenaSessionInfo } from "../lib/session";
import type { ArenaState } from "../lib/types";

interface PregameSetupOverlayProps {
  state: ArenaState;
  sessionInfo: ArenaSessionInfo | null;
  onRun: (operation: (current: ArenaCommandTarget) => Promise<ArenaState>) => void;
}

export const PregameSetupOverlay: Component<PregameSetupOverlayProps> = (props) => {
  const [selectedBottomCardIds, setSelectedBottomCardIds] = createSignal<string[]>([]);
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
  const canConfirmKeep = () => selectedBottomCardIds().length === cardsToBottom();
  const selectedBottomCardSet = createMemo(() => new Set(selectedBottomCardIds()));

  createEffect(() => {
    const availableCardIds = new Set(localHandCards().map((card) => card.card_id));
    const requiredCount = cardsToBottom();
    const keepSelection = canAct();

    setSelectedBottomCardIds((current) => {
      if (!keepSelection || requiredCount === 0) {
        return [];
      }

      return current.filter((cardId) => availableCardIds.has(cardId)).slice(0, requiredCount);
    });
  });

  const toggleBottomCardSelection = (cardId: string) => {
    if (!canAct()) {
      return;
    }

    const requiredCount = cardsToBottom();
    if (requiredCount === 0) {
      return;
    }

    setSelectedBottomCardIds((current) => {
      if (current.includes(cardId)) {
        return current.filter((entry) => entry !== cardId);
      }

      if (current.length >= requiredCount) {
        return [...current.slice(1), cardId];
      }

      return [...current, cardId];
    });
  };

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
                Choose {cardsToBottom()} card{cardsToBottom() === 1 ? "" : "s"} to put on the
                bottom, then keep your opening hand.
              </Show>
            </Show>
          </p>
        </section>

        <Show when={canAct() && cardsToBottom() > 0}>
          <section class="pregame-selection-dock panel">
            <div class="pregame-selection-copy">
              <p class="eyebrow">London mulligan</p>
              <strong>
                Select {cardsToBottom()} card{cardsToBottom() === 1 ? "" : "s"} to bottom
              </strong>
              <span>
                Selected {selectedBottomCardIds().length} / {cardsToBottom()}
              </span>
            </div>
            <div class="pregame-bottom-card-grid">
              <For each={localHandCards()}>
                {(card) => (
                  <button
                    classList={{
                      "pregame-bottom-card": true,
                      selected: selectedBottomCardSet().has(card.card_id),
                    }}
                    type="button"
                    onClick={() => {
                      toggleBottomCardSelection(card.card_id);
                    }}
                  >
                    <span class="pregame-bottom-card-name">
                      {formatCardLabel(card.definition_id)}
                    </span>
                    <span class="pregame-bottom-card-meta">{card.card_type}</span>
                  </button>
                )}
              </For>
            </div>
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
                      keepOpeningHand(current, playerId, selectedBottomCardIds()),
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
