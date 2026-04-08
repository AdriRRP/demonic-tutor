import { Show } from "solid-js";
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
  const mulliganUsed = () => localViewer()?.mulligan_used ?? false;
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
              You can review your opening hand and decide whether to keep it.
            </Show>
          </p>
        </section>

        <section class="pregame-action-dock panel">
          <div class="pregame-action-copy">
            <p class="eyebrow">Opening seven</p>
            <strong>{localHandCount()} cards ready</strong>
            <span>
              <Show
                when={!mulliganUsed()}
                fallback="Your mulligan has been used for this opening hand."
              >
                One redraw is available in the current subset.
              </Show>
            </span>
          </div>

          <div class="pregame-action-buttons">
            <Show when={canAct()}>
              <>
                <button
                  class="hero-button hero-button-ghost"
                  disabled={mulliganUsed()}
                  type="button"
                  onClick={() => {
                    const playerId = localViewer()?.player_id;
                    if (!playerId || mulliganUsed()) {
                      return;
                    }
                    props.onRun((current) => mulliganOpeningHand(current, playerId));
                  }}
                >
                  Mulligan
                </button>
                <button
                  class="hero-button"
                  type="button"
                  onClick={() => {
                    const playerId = localViewer()?.player_id;
                    if (!playerId) {
                      return;
                    }
                    props.onRun((current) => keepOpeningHand(current, playerId));
                  }}
                >
                  Keep {localHandCount()}
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
