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
  const [revealedHands, setRevealedHands] = createSignal<Record<string, boolean>>({});
  const [selectedAttackers, setSelectedAttackers] = createSignal<string[]>([]);
  const [blockerAssignments, setBlockerAssignments] = createSignal<Record<string, string>>({});

  onMount(() => {
    void loadArena();
  });

  async function loadArena(): Promise<void> {
    try {
      const nextClient = await createArenaClient();
      const nextState = readState(nextClient);
      setClient(nextClient);
      setState(nextState);
      setRevealedHands(revealPattern(nextState));
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
      setState(nextState);
      setRevealedHands(revealPattern(nextState));
      setSelectedAttackers([]);
      setBlockerAssignments({});
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    }
  };

  const toggleHandReveal = (playerId: string) => {
    setRevealedHands((current) => ({
      ...current,
      [playerId]: !current[playerId],
    }));
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
              onRun={run}
              onSetBlockerAssignment={setBlockerAssignment}
              onToggleAttackerSelection={toggleAttackerSelection}
              onToggleHandReveal={toggleHandReveal}
              revealedHands={revealedHands()}
              selectedAttackers={selectedAttackers()}
              state={resolved()}
            />
          )}
        </Match>
      </Switch>
    </main>
  );
};

function revealPattern(state: ArenaState): Record<string, boolean> {
  if (state.game.is_over) {
    return Object.fromEntries(state.viewers.map((viewer) => [viewer.player_id, true]));
  }

  return Object.fromEntries(
    state.viewers.map((viewer) => [
      viewer.player_id,
      viewer.player_id === state.game.active_player_id,
    ]),
  );
}

export default App;
