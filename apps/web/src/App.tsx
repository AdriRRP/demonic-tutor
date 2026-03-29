import { Match, Switch, createSignal, onMount } from "solid-js";
import type { Component } from "solid-js";
import { TableArena } from "./components/table-arena";
import { createArenaClient, readState, resetArena, type WebArenaClient } from "./lib/runtime";
import type { ArenaState } from "./lib/types";

const App: Component = () => {
  const [client, setClient] = createSignal<WebArenaClient | null>(null);
  const [state, setState] = createSignal<ArenaState | null>(null);
  const [error, setError] = createSignal<string | null>(null);
  const [loading, setLoading] = createSignal(true);
  const [revealedSeatId, setRevealedSeatId] = createSignal<string | null>(null);
  const [pendingHandoffPlayerId, setPendingHandoffPlayerId] = createSignal<string | null>(null);
  const [selectedAttackers, setSelectedAttackers] = createSignal<string[]>([]);
  const [blockerAssignments, setBlockerAssignments] = createSignal<Record<string, string>>({});

  onMount(() => {
    void loadArena();
  });

  async function loadArena(): Promise<void> {
    try {
      const nextClient = await createArenaClient();
      const nextState = readState(nextClient);
      const initialFocus = focusPlayerId(nextState);
      const nextSeatPrivacy = deriveSeatPrivacy(nextState, initialFocus);
      setClient(nextClient);
      setState(nextState);
      setRevealedSeatId(nextSeatPrivacy.revealedSeatId);
      setPendingHandoffPlayerId(nextSeatPrivacy.pendingHandoffPlayerId);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }

  const run = (operation: (current: WebArenaClient) => ArenaState) => {
    const current = client();
    if (!current) {
      return;
    }

    try {
      const nextState = operation(current);
      const nextSeatPrivacy = deriveSeatPrivacy(nextState, revealedSeatId());
      setState(nextState);
      setRevealedSeatId(nextSeatPrivacy.revealedSeatId);
      setPendingHandoffPlayerId(nextSeatPrivacy.pendingHandoffPlayerId);
      setSelectedAttackers([]);
      setBlockerAssignments({});
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    }
  };

  const toggleSeatPrivacy = (playerId: string) => {
    if (state()?.game.is_over) {
      return;
    }

    if (revealedSeatId() === playerId) {
      setRevealedSeatId(null);
      return;
    }

    setRevealedSeatId(playerId);
    setPendingHandoffPlayerId(null);
  };

  const toggleAttackerSelection = (cardId: string) => {
    setSelectedAttackers((current) =>
      current.includes(cardId) ? current.filter((entry) => entry !== cardId) : [...current, cardId],
    );
  };

  const setBlockerAssignment = (blockerId: string, attackerId: string) => {
    setBlockerAssignments((current) => {
      if (attackerId.length === 0) {
        return Object.fromEntries(
          Object.entries(current).filter(([entryBlockerId]) => entryBlockerId !== blockerId),
        );
      }

      return {
        ...current,
        [blockerId]: attackerId,
      };
    });
  };

  return (
    <main class="shell">
      <div class="playmat-halo playmat-halo-top" />
      <div class="playmat-halo playmat-halo-bottom" />

      <Switch>
        <Match when={loading()}>
          <section class="panel panel-loading">
            <p>Loading wasm runtime…</p>
          </section>
        </Match>
        <Match when={error()}>
          <section class="panel panel-error">
            <h2>Runtime error</h2>
            <p>{error()}</p>
            <button
              class="hero-button"
              onClick={() => {
                run(resetArena);
              }}
            >
              Retry duel reset
            </button>
          </section>
        </Match>
        <Match when={state()}>
          {(resolved) => (
            <TableArena
              blockerAssignments={blockerAssignments()}
              onToggleSeatPrivacy={toggleSeatPrivacy}
              onRun={run}
              onSetBlockerAssignment={setBlockerAssignment}
              onToggleAttackerSelection={toggleAttackerSelection}
              pendingHandoffPlayerId={pendingHandoffPlayerId()}
              revealedSeatId={revealedSeatId()}
              selectedAttackers={selectedAttackers()}
              state={resolved()}
            />
          )}
        </Match>
      </Switch>
    </main>
  );
};

function deriveSeatPrivacy(
  state: ArenaState,
  currentRevealedSeatId: string | null,
): { revealedSeatId: string | null; pendingHandoffPlayerId: string | null } {
  const nextFocus = focusPlayerId(state);

  if (state.game.is_over) {
    return { revealedSeatId: null, pendingHandoffPlayerId: null };
  }

  if (!nextFocus) {
    return {
      revealedSeatId: currentRevealedSeatId,
      pendingHandoffPlayerId: null,
    };
  }

  if (currentRevealedSeatId === nextFocus) {
    return {
      revealedSeatId: currentRevealedSeatId,
      pendingHandoffPlayerId: null,
    };
  }

  return {
    revealedSeatId: null,
    pendingHandoffPlayerId: nextFocus,
  };
}

function focusPlayerId(state: ArenaState): string | null {
  if (state.game.is_over) {
    return null;
  }

  const promptOwner = state.viewers.find((viewer) => viewer.choice_requests.length > 0)?.player_id;
  if (promptOwner) {
    return promptOwner;
  }

  if (state.game.priority_holder) {
    return state.game.priority_holder;
  }

  if (state.game.active_player_id) {
    return state.game.active_player_id;
  }

  return state.viewers[0]?.player_id ?? null;
}

export default App;
