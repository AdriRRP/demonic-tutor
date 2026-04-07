import { Match, Show, Switch, createSignal, onCleanup, onMount } from "solid-js";
import type { Component } from "solid-js";
import { RemotePairingModal } from "./components/remote-pairing-modal";
import { TableArena } from "./components/table-arena";
import {
  createRemotePairingController,
  type RemotePairingController,
  type RemotePairingState,
} from "./lib/remote-pairing";
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
  const [remotePairingModalOpen, setRemotePairingModalOpen] = createSignal(false);
  const [remotePairingState, setRemotePairingState] = createSignal<RemotePairingState | null>(null);
  const [remotePairingSupported, setRemotePairingSupported] = createSignal(false);
  let remotePairingController: RemotePairingController | null = null;
  let unsubscribeSession: (() => void) | undefined;
  let unsubscribeRemotePairing: (() => void) | undefined;

  onMount(() => {
    if (typeof RTCPeerConnection !== "undefined") {
      remotePairingController = createRemotePairingController();
      unsubscribeRemotePairing = remotePairingController.subscribe((stateSnapshot) => {
        setRemotePairingState(stateSnapshot);
      });
      setRemotePairingSupported(true);
    }

    void loadArena();
  });

  onCleanup(() => {
    unsubscribeSession?.();
    unsubscribeRemotePairing?.();
    remotePairingController?.destroy();
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

  const beginRemoteHosting = async (): Promise<void> => {
    if (!remotePairingController) {
      throw new Error("This browser does not support remote pairing.");
    }

    await remotePairingController.beginHosting();
  };

  const acceptRemoteOffer = async (offerPayload: string): Promise<void> => {
    if (!remotePairingController) {
      throw new Error("This browser does not support remote pairing.");
    }

    await remotePairingController.acceptOfferAndCreateAnswer(offerPayload);
  };

  const applyRemoteAnswer = async (answerPayload: string): Promise<void> => {
    if (!remotePairingController) {
      throw new Error("This browser does not support remote pairing.");
    }

    await remotePairingController.applyAnswer(answerPayload);
  };

  const resetRemotePairing = (): void => {
    remotePairingController?.reset();
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
              onOpenRemotePairing={
                remotePairingSupported()
                  ? () => {
                      setRemotePairingModalOpen(true);
                    }
                  : undefined
              }
              onRun={run}
              onSetBlockerAssignment={setBlockerAssignment}
              onToggleAttackerSelection={toggleAttackerSelection}
              remotePairingState={remotePairingState()}
              selectedAttackers={selectedAttackers()}
              sessionInfo={sessionInfo()}
              state={resolved()}
            />
          )}
        </Match>
      </Switch>

      <Show when={remotePairingModalOpen()}>
        <RemotePairingModal
          onAcceptOffer={acceptRemoteOffer}
          onApplyAnswer={applyRemoteAnswer}
          onBeginHosting={beginRemoteHosting}
          onClose={() => {
            setRemotePairingModalOpen(false);
          }}
          onReset={resetRemotePairing}
          state={remotePairingState()}
          supported={remotePairingSupported()}
        />
      </Show>
    </main>
  );
};

export default App;
