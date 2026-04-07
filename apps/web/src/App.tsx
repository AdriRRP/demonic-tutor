import { Match, Switch, createSignal, onCleanup, onMount } from "solid-js";
import type { Component } from "solid-js";
import { TableArena } from "./components/table-arena";
import { createArenaSession, type ArenaSession, type ArenaSessionInfo } from "./lib/session";
import { readState, resetArena, type ArenaCommandTarget } from "./lib/runtime";
import type { ArenaState } from "./lib/types";

const App: Component = () => {
  const [session, setSession] = createSignal<ArenaSession | null>(null);
  const [sessionInfo, setSessionInfo] = createSignal<ArenaSessionInfo | null>(null);
  const [state, setState] = createSignal<ArenaState | null>(null);
  const [error, setError] = createSignal<string | null>(null);
  const [loading, setLoading] = createSignal(true);
  const [selectedAttackers, setSelectedAttackers] = createSignal<string[]>([]);
  const [blockerAssignments, setBlockerAssignments] = createSignal<Record<string, string>>({});
  let unsubscribeSession: (() => void) | undefined;

  onMount(() => {
    void loadArena();
  });

  onCleanup(() => {
    unsubscribeSession?.();
    session()?.destroy();
  });

  async function loadArena(): Promise<void> {
    try {
      unsubscribeSession?.();
      session()?.destroy();
      setLoading(true);
      setError(null);

      const nextSession = await createArenaSession();
      const nextState = await readState(nextSession);
      const nextInfo = nextSession.info();

      unsubscribeSession = nextSession.subscribe((incomingState) => {
        const incomingInfo = nextSession.info();
        setSessionInfo(incomingInfo);
        setState(incomingState);
      });

      setSession(nextSession);
      setSessionInfo(nextInfo);
      setState(nextState);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }

  const run = (operation: (current: ArenaCommandTarget) => Promise<ArenaState>) => {
    const current = session();
    if (!current) {
      return;
    }

    void (async () => {
      try {
        const nextState = await operation(current);
        const nextInfo = current.info();
        setSessionInfo(nextInfo);
        setState(nextState);
        setSelectedAttackers([]);
        setBlockerAssignments({});
        setError(null);
      } catch (err) {
        setError(err instanceof Error ? err.message : String(err));
      }
    })();
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

  const copyInviteLink = () => {
    const inviteUrl = sessionInfo()?.inviteUrl ?? window.location.href;
    void (async () => {
      try {
        await navigator.clipboard.writeText(inviteUrl);
        setError(null);
      } catch (err) {
        setError(err instanceof Error ? err.message : "Could not copy the duel room link.");
      }
    })();
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
                if (session()) {
                  run(resetArena);
                  return;
                }

                void loadArena();
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
              onCopyInviteLink={copyInviteLink}
              onRun={run}
              onSetBlockerAssignment={setBlockerAssignment}
              onToggleAttackerSelection={toggleAttackerSelection}
              selectedAttackers={selectedAttackers()}
              sessionInfo={sessionInfo()}
              state={resolved()}
            />
          )}
        </Match>
      </Switch>
    </main>
  );
};

export default App;
