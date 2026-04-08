import { Match, Show, Switch, createEffect, createSignal, onCleanup, onMount } from "solid-js";
import type { Component } from "solid-js";
import { PregameSetupOverlay } from "./components/pregame-setup-overlay";
import { RemotePairingModal } from "./components/remote-pairing-modal";
import { TableArena } from "./components/table-arena";
import {
  createRemotePairingController,
  type RemotePairingController,
  type RemotePairingState,
  type RemotePairingTransport,
} from "./lib/remote-pairing";
import {
  attachRemoteCommandRelay,
  createArenaSession,
  createRemotePeerSession,
  type ArenaSession,
  type ArenaSessionInfo,
} from "./lib/session";
import { readState, resetArena, type ArenaCommandTarget } from "./lib/runtime";
import type { ArenaPresentationState, ArenaState } from "./lib/types";

const EMPTY_PRESENTATION_STATE: ArenaPresentationState = {
  battlefield_layouts: {},
};

const App: Component = () => {
  const [session, setSession] = createSignal<ArenaSession | null>(null);
  const [sessionInfo, setSessionInfo] = createSignal<ArenaSessionInfo | null>(null);
  const [presentationState, setPresentationState] = createSignal(EMPTY_PRESENTATION_STATE);
  const [state, setState] = createSignal<ArenaState | null>(null);
  const [error, setError] = createSignal<string | null>(null);
  const [loading, setLoading] = createSignal(true);
  const [selectedAttackers, setSelectedAttackers] = createSignal<string[]>([]);
  const [blockerAssignments, setBlockerAssignments] = createSignal<Record<string, string>>({});
  const [pregameBottomCardIds, setPregameBottomCardIds] = createSignal<string[]>([]);
  const [remotePairingModalOpen, setRemotePairingModalOpen] = createSignal(false);
  const [remotePairingState, setRemotePairingState] = createSignal<RemotePairingState | null>(null);
  const [remotePairingSupported, setRemotePairingSupported] = createSignal(false);
  const [remoteSessionEndedReason, setRemoteSessionEndedReason] = createSignal<string | null>(null);
  let remotePairingController: RemotePairingController | null = null;
  let activeRemotePeerTransportId: string | null = null;
  let attachedRemoteHostTransportId: string | null = null;
  let detachRemoteCommandRelay: (() => void) | undefined;
  let unsubscribeSession: (() => void) | undefined;
  let unsubscribePresentation: (() => void) | undefined;
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
    unsubscribePresentation?.();
    unsubscribeRemotePairing?.();
    detachRemoteCommandRelay?.();
    remotePairingController?.destroy();
    session()?.destroy();
  });

  createEffect(() => {
    const pairing = remotePairingState();
    const transport = remotePairingController?.transport() ?? null;
    const currentSession = session();

    if (!pairing || !transport) {
      if (attachedRemoteHostTransportId !== null) {
        detachRemoteCommandRelay?.();
        detachRemoteCommandRelay = undefined;
        attachedRemoteHostTransportId = null;
      }
      return;
    }

    if (pairing.role === "host") {
      if (currentSession?.info().role !== "host") {
        return;
      }

      if (attachedRemoteHostTransportId === transport.id) {
        return;
      }

      detachRemoteCommandRelay?.();
      detachRemoteCommandRelay = attachRemoteCommandRelay(currentSession, transport);
      attachedRemoteHostTransportId = transport.id;
      return;
    }

    if (attachedRemoteHostTransportId !== null) {
      detachRemoteCommandRelay?.();
      detachRemoteCommandRelay = undefined;
      attachedRemoteHostTransportId = null;
    }

    if (pairing.role === "peer" && activeRemotePeerTransportId !== transport.id) {
      activeRemotePeerTransportId = transport.id;
      void loadRemotePeerArena(transport);
    }
  });

  createEffect(() => {
    const pairing = remotePairingState();
    const currentSessionInfo = sessionInfo();

    if (
      currentSessionInfo?.role === "peer" &&
      pairing?.role === "peer" &&
      pairing.phase === "failed"
    ) {
      setRemoteSessionEndedReason(pairing.error ?? pairing.statusLabel);
      return;
    }

    if (pairing?.connected || currentSessionInfo?.role !== "peer") {
      setRemoteSessionEndedReason(null);
    }
  });

  createEffect(() => {
    const currentState = state();
    const localSeatId = sessionInfo()?.localSeatId ?? null;
    const currentDecisionPlayerId = currentState?.pregame?.current_decision_player_id ?? null;
    const requiredBottomCount = currentState?.pregame?.current_bottom_count ?? 0;
    const localViewer =
      currentState?.viewers.find((viewer) => viewer.player_id === localSeatId) ??
      currentState?.viewers[0];
    const handCardIds = new Set(localViewer?.hand.map((card) => card.card_id) ?? []);

    setPregameBottomCardIds((current) => {
      if (
        !currentState?.pregame ||
        localViewer?.player_id !== currentDecisionPlayerId ||
        requiredBottomCount === 0
      ) {
        return [];
      }

      return current.filter((cardId) => handCardIds.has(cardId)).slice(0, requiredBottomCount);
    });
  });

  async function loadArena(): Promise<void> {
    try {
      activeRemotePeerTransportId = null;
      unsubscribeSession?.();
      unsubscribePresentation?.();
      session()?.destroy();
      setLoading(true);
      setError(null);
      setRemoteSessionEndedReason(null);
      setPresentationState(EMPTY_PRESENTATION_STATE);

      const nextSession = await createArenaSession();
      const nextState = await readState(nextSession);
      const nextInfo = nextSession.info();

      unsubscribeSession = nextSession.subscribe((incomingState) => {
        const incomingInfo = nextSession.info();
        setSessionInfo(incomingInfo);
        setState(incomingState);
      });
      unsubscribePresentation = nextSession.subscribePresentation((incomingPresentationState) => {
        setPresentationState(incomingPresentationState);
      });

      setSession(nextSession);
      setSessionInfo(nextInfo);
      setPresentationState(nextSession.presentation());
      setState(nextState);
    } catch (err) {
      activeRemotePeerTransportId = null;
      setPresentationState(EMPTY_PRESENTATION_STATE);
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }

  async function loadRemotePeerArena(transport: RemotePairingTransport): Promise<void> {
    try {
      unsubscribeSession?.();
      unsubscribePresentation?.();
      session()?.destroy();
      setLoading(true);
      setError(null);
      setRemoteSessionEndedReason(null);
      setPresentationState(EMPTY_PRESENTATION_STATE);

      const roomId =
        sessionInfo()?.roomId ?? new URL(window.location.href).searchParams.get("duel") ?? "remote";
      const nextSession = await createRemotePeerSession({
        inviteUrl: window.location.href,
        roomId,
        transport,
      });
      const nextState = await readState(nextSession);
      const nextInfo = nextSession.info();

      unsubscribeSession = nextSession.subscribe((incomingState) => {
        const incomingInfo = nextSession.info();
        setSessionInfo(incomingInfo);
        setState(incomingState);
      });
      unsubscribePresentation = nextSession.subscribePresentation((incomingPresentationState) => {
        setPresentationState(incomingPresentationState);
      });

      setSession(nextSession);
      setSessionInfo(nextInfo);
      setPresentationState(nextSession.presentation());
      setState(nextState);
      setSelectedAttackers([]);
      setBlockerAssignments({});
    } catch (err) {
      setPresentationState(EMPTY_PRESENTATION_STATE);
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

    if (remoteSessionEndedReason() !== null) {
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

  const togglePregameBottomCard = (cardId: string) => {
    const currentState = state();
    const currentPregame = currentState?.pregame;
    const localSeatId = sessionInfo()?.localSeatId ?? null;
    const localViewer =
      currentState?.viewers.find((viewer) => viewer.player_id === localSeatId) ??
      currentState?.viewers[0];

    if (
      !currentPregame ||
      localViewer?.player_id !== currentPregame.current_decision_player_id ||
      currentPregame.current_bottom_count === 0 ||
      !localViewer.hand.some((card) => card.card_id === cardId)
    ) {
      return;
    }

    setPregameBottomCardIds((current) => {
      if (current.includes(cardId)) {
        return current.filter((entry) => entry !== cardId);
      }

      if (current.length >= currentPregame.current_bottom_count) {
        return [...current.slice(1), cardId];
      }

      return [...current, cardId];
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

  const reopenRemotePairing = (): void => {
    remotePairingController?.reset();
    setRemotePairingModalOpen(true);
  };

  const returnToLocalDuel = (): void => {
    remotePairingController?.reset();
    setRemotePairingModalOpen(false);
    void loadArena();
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
            <>
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
                presentationState={presentationState()}
                pregameBottomCardIds={pregameBottomCardIds()}
                state={resolved()}
                onSyncBattlefieldLayout={(playerId, positions) => {
                  session()?.updateBattlefieldLayout(playerId, positions);
                }}
                onTogglePregameBottomCard={togglePregameBottomCard}
              />
              <PregameSetupOverlay
                onRun={run}
                selectedBottomCardIds={pregameBottomCardIds()}
                sessionInfo={sessionInfo()}
                state={resolved()}
              />
            </>
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

      <Show when={remoteSessionEndedReason()}>
        {(reason) => (
          <div class="table-modal-backdrop">
            <aside class="table-sidebar panel open remote-session-ended-modal">
              <div class="table-sidebar-head remote-pairing-head">
                <div>
                  <p class="eyebrow sidebar-eyebrow">Remote duel ended</p>
                  <h2>Authoritative host lost</h2>
                </div>
              </div>
              <p class="remote-pairing-note">
                The remote duel cannot continue because the authoritative host browser is no longer
                reachable.
              </p>
              <div class="remote-pairing-error">
                <p class="label">Connection status</p>
                <strong>{reason()}</strong>
              </div>
              <div class="remote-pairing-field-actions">
                <button class="hero-button" type="button" onClick={reopenRemotePairing}>
                  Pair again
                </button>
                <button
                  class="hero-button hero-button-ghost"
                  type="button"
                  onClick={returnToLocalDuel}
                >
                  Reset to local duel
                </button>
              </div>
            </aside>
          </div>
        )}
      </Show>
    </main>
  );
};

export default App;
