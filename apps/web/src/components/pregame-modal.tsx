import { For, Show } from "solid-js";
import type { Component } from "solid-js";
import type { ArenaSessionInfo } from "../lib/session";
import { keepOpeningHand, takeMulligan, type ArenaCommandTarget } from "../lib/runtime";
import type { ArenaPregamePlayerState, ArenaPregameState, ArenaState } from "../lib/types";

interface PregameModalProps {
  state: ArenaPregameState;
  sessionInfo: ArenaSessionInfo | null;
  revealedSeatId: string | null;
  onToggleSeatPrivacy: (playerId: string) => void;
  onRun: (operation: (current: ArenaCommandTarget) => Promise<ArenaState>) => void;
}

export const PregameModal: Component<PregameModalProps> = (props) => {
  const currentPlayer = () =>
    props.state.players.find((player) => player.player_id === props.state.current_player_id) ??
    props.state.players[0];
  const localSeatId = () => props.sessionInfo?.localSeatId ?? null;
  const localCanAct = () => {
    const current = currentPlayer();
    if (!current) {
      return false;
    }

    return localSeatId() === null || localSeatId() === current.player_id;
  };
  const needsReveal = () => {
    const current = currentPlayer();
    if (!current) {
      return false;
    }

    return localSeatId() === null && props.revealedSeatId !== current.player_id;
  };

  return (
    <div class="table-modal-backdrop pregame-modal-backdrop">
      <aside class="table-sidebar panel open pregame-modal">
        <div class="table-sidebar-head">
          <div>
            <p class="eyebrow sidebar-eyebrow">Pregame</p>
            <h2>Opening hands</h2>
          </div>
          <div class="chip-row">
            <span class="chip chip-night">
              Starter {shortPlayerTag(props.state.starting_player_id)}
            </span>
            <span class="chip chip-ember">Setup</span>
          </div>
        </div>

        <div class="pregame-lead">
          <p>
            Starting player chosen at random:
            <strong>{` ${shortPlayerTag(props.state.starting_player_id)}`}</strong>.
          </p>
          <p class="muted">
            Each player may keep the opening hand or take the single mulligan supported by the
            current playable subset.
          </p>
        </div>

        <div class="pregame-player-grid">
          <For each={props.state.players}>{(player) => <PregameSeatCard player={player} />}</For>
        </div>

        <Show when={currentPlayer()}>
          {(player) => (
            <section class="pregame-decision-card">
              <div>
                <p class="label">Current decision</p>
                <h3>{shortPlayerTag(player().player_id)}</h3>
                <p class="muted">
                  <Show
                    when={player().mulligan_used}
                    fallback={
                      <>
                        Review the opening hand, then choose whether to keep it or take the
                        available mulligan.
                      </>
                    }
                  >
                    A new opening hand is ready after mulligan. Keep it to continue.
                  </Show>
                </p>
              </div>

              <Show when={!localCanAct()}>
                <p class="support-note">
                  Waiting for {shortPlayerTag(player().player_id)} on the other screen.
                </p>
              </Show>

              <Show when={localCanAct() && needsReveal()}>
                <div class="pregame-action-row">
                  <button
                    class="hero-button"
                    onClick={() => {
                      props.onToggleSeatPrivacy(player().player_id);
                    }}
                  >
                    Reveal opening hand
                  </button>
                </div>
              </Show>

              <Show when={localCanAct() && !needsReveal()}>
                <div class="pregame-action-row">
                  <button
                    class="hero-button"
                    onClick={() => {
                      const playerId = player().player_id;
                      props.onRun((current) => keepOpeningHand(current, playerId));
                    }}
                  >
                    Keep hand
                  </button>
                  <Show when={player().can_mulligan}>
                    <button
                      class="hero-button hero-button-ghost"
                      onClick={() => {
                        const playerId = player().player_id;
                        props.onRun((current) => takeMulligan(current, playerId));
                      }}
                    >
                      Mulligan
                    </button>
                  </Show>
                </div>
              </Show>
            </section>
          )}
        </Show>
      </aside>
    </div>
  );
};

const PregameSeatCard: Component<{ player: ArenaPregamePlayerState }> = (props) => {
  return (
    <article
      classList={{
        "pregame-seat-card": true,
        current: props.player.is_current,
        starter: props.player.is_starting_player,
        kept: props.player.kept,
      }}
    >
      <div class="pregame-seat-head">
        <div>
          <p class="label">Seat</p>
          <h3>{shortPlayerTag(props.player.player_id)}</h3>
        </div>
        <div class="chip-row">
          <Show when={props.player.is_starting_player}>
            <span class="chip chip-night">Starter</span>
          </Show>
          <Show when={props.player.is_current}>
            <span class="chip chip-ember">Current</span>
          </Show>
          <Show when={props.player.kept}>
            <span class="chip chip-forest">Kept</span>
          </Show>
        </div>
      </div>

      <div class="pregame-seat-stats">
        <span class="seat-summary-pill">{`Hand ${String(props.player.hand_count)}`}</span>
        <span class="seat-summary-pill">
          {props.player.mulligan_used ? "Mulligan used" : "Mulligan ready"}
        </span>
      </div>
    </article>
  );
};

function shortPlayerTag(playerId: string | null | undefined): string {
  if (!playerId) {
    return "—";
  }

  return playerId.replace("player-", "P");
}
