import { Show, createEffect, createSignal } from "solid-js";
import type { Component } from "solid-js";
import type { RemotePairingState } from "../lib/remote-pairing";

interface RemotePairingModalProps {
  onAcceptOffer: (offerPayload: string) => Promise<void>;
  onApplyAnswer: (answerPayload: string) => Promise<void>;
  onBeginHosting: () => Promise<void>;
  onClose: () => void;
  onReset: () => void;
  state: RemotePairingState | null;
  supported: boolean;
}

type PendingAction = "accept-offer" | "apply-answer" | "begin-hosting" | "copy-signal" | null;

export const RemotePairingModal: Component<RemotePairingModalProps> = (props) => {
  const [hostAnswerInput, setHostAnswerInput] = createSignal("");
  const [peerOfferInput, setPeerOfferInput] = createSignal("");
  const [localError, setLocalError] = createSignal<string | null>(null);
  const [pendingAction, setPendingAction] = createSignal<PendingAction>(null);
  const [copyFeedback, setCopyFeedback] = createSignal<string | null>(null);

  createEffect(() => {
    const signal = props.state?.localSignal ?? "";
    setLocalError(null);

    if (!signal) {
      setCopyFeedback(null);
    }
  });

  const copySignal = (payload: string) => {
    if (!payload) {
      setLocalError("There is no local signaling payload to copy yet.");
      return;
    }

    void (async () => {
      setPendingAction("copy-signal");
      try {
        await navigator.clipboard.writeText(payload);
        setCopyFeedback("Copied");
        setLocalError(null);
      } catch {
        setLocalError("The browser could not copy the signaling payload.");
      } finally {
        setPendingAction(null);
      }
    })();
  };

  const runAction = (action: PendingAction, work: () => Promise<void>) => {
    void (async () => {
      setPendingAction(action);
      setCopyFeedback(null);
      setLocalError(null);

      try {
        await work();
      } catch (error) {
        setLocalError(error instanceof Error ? error.message : String(error));
      } finally {
        setPendingAction(null);
      }
    })();
  };

  const statusError = () => localError() ?? props.state?.error ?? null;
  const hostSignal = () => (props.state?.role === "host" ? props.state.localSignal : "");
  const peerSignal = () => (props.state?.role === "peer" ? props.state.localSignal : "");

  return (
    <div
      class="table-modal-backdrop"
      onClick={() => {
        props.onClose();
      }}
    >
      <aside
        class="table-sidebar panel open remote-pairing-modal"
        onClick={(event) => {
          event.stopPropagation();
        }}
      >
        <div class="table-sidebar-head remote-pairing-head">
          <div>
            <p class="eyebrow sidebar-eyebrow">Remote duel</p>
            <h2>Pair two browsers</h2>
          </div>
          <div class="chip-row">
            <StatusChip
              label={props.state ? formatPairingPhase(props.state.phase) : "Idle"}
              tone={props.state?.connected ? "connected" : "default"}
            />
            <Show when={props.state?.role}>
              {(role) => <StatusChip label={formatPairingRole(role())} tone="default" />}
            </Show>
            <button
              class="hero-button hero-button-ghost mini-button"
              type="button"
              onClick={() => {
                props.onReset();
                setHostAnswerInput("");
                setPeerOfferInput("");
                setLocalError(null);
                setCopyFeedback(null);
              }}
            >
              Reset
            </button>
            <button
              class="hero-button hero-button-ghost mini-button"
              type="button"
              onClick={() => {
                props.onClose();
              }}
            >
              Close
            </button>
          </div>
        </div>

        <p class="remote-pairing-note">
          Manual WebRTC pairing closes the transport handshake for the first remote-duel slice. The
          authoritative host still owns gameplay execution until the next transport slices land.
        </p>

        <Show
          when={props.supported}
          fallback={
            <div class="remote-pairing-error">
              <p class="label">Unavailable</p>
              <strong>This browser cannot create a WebRTC duel channel.</strong>
            </div>
          }
        >
          <div class="remote-pairing-grid">
            <section class="remote-pairing-panel">
              <div class="remote-pairing-panel-head">
                <p class="label">Host</p>
                <strong>Open the room</strong>
              </div>
              <p class="muted">
                Generate an offer in the authoritative browser, send it to the second device, then
                paste the remote answer here.
              </p>

              <button
                class="hero-button"
                type="button"
                disabled={pendingAction() !== null && pendingAction() !== "copy-signal"}
                onClick={() => {
                  runAction("begin-hosting", props.onBeginHosting);
                }}
              >
                {pendingAction() === "begin-hosting" ? "Preparing offer…" : "Generate offer"}
              </button>

              <div class="remote-pairing-field">
                <label for="remote-host-offer">Offer payload</label>
                <textarea
                  id="remote-host-offer"
                  class="remote-pairing-textarea remote-pairing-output"
                  readonly
                  value={hostSignal()}
                />
                <div class="remote-pairing-field-actions">
                  <button
                    class="hero-button hero-button-ghost mini-button"
                    type="button"
                    disabled={!hostSignal()}
                    onClick={() => {
                      copySignal(hostSignal());
                    }}
                  >
                    {pendingAction() === "copy-signal"
                      ? "Copying…"
                      : (copyFeedback() ?? "Copy payload")}
                  </button>
                </div>
              </div>

              <div class="remote-pairing-field">
                <label for="remote-host-answer">Remote answer</label>
                <textarea
                  id="remote-host-answer"
                  class="remote-pairing-textarea"
                  placeholder="Paste the answer returned by the remote peer."
                  value={hostAnswerInput()}
                  onInput={(event) => {
                    setHostAnswerInput(event.currentTarget.value);
                  }}
                />
              </div>

              <button
                class="hero-button hero-button-ghost"
                type="button"
                disabled={hostAnswerInput().trim().length === 0 || pendingAction() !== null}
                onClick={() => {
                  runAction("apply-answer", () => props.onApplyAnswer(hostAnswerInput().trim()));
                }}
              >
                {pendingAction() === "apply-answer" ? "Connecting…" : "Apply answer"}
              </button>
            </section>

            <section class="remote-pairing-panel">
              <div class="remote-pairing-panel-head">
                <p class="label">Join</p>
                <strong>Answer the host</strong>
              </div>
              <p class="muted">
                Paste the host offer in the second browser, create the answer, then return that
                answer back to the host.
              </p>

              <div class="remote-pairing-field">
                <label for="remote-peer-offer">Host offer</label>
                <textarea
                  id="remote-peer-offer"
                  class="remote-pairing-textarea"
                  placeholder="Paste the host offer payload here."
                  value={peerOfferInput()}
                  onInput={(event) => {
                    setPeerOfferInput(event.currentTarget.value);
                  }}
                />
              </div>

              <button
                class="hero-button"
                type="button"
                disabled={peerOfferInput().trim().length === 0 || pendingAction() !== null}
                onClick={() => {
                  runAction("accept-offer", () => props.onAcceptOffer(peerOfferInput().trim()));
                }}
              >
                {pendingAction() === "accept-offer" ? "Creating answer…" : "Generate answer"}
              </button>

              <div class="remote-pairing-field">
                <label for="remote-peer-answer">Answer payload</label>
                <textarea
                  id="remote-peer-answer"
                  class="remote-pairing-textarea remote-pairing-output"
                  readonly
                  value={peerSignal()}
                />
                <div class="remote-pairing-field-actions">
                  <button
                    class="hero-button hero-button-ghost mini-button"
                    type="button"
                    disabled={!peerSignal()}
                    onClick={() => {
                      copySignal(peerSignal());
                    }}
                  >
                    {pendingAction() === "copy-signal"
                      ? "Copying…"
                      : (copyFeedback() ?? "Copy payload")}
                  </button>
                </div>
              </div>
            </section>
          </div>
        </Show>

        <div class="remote-pairing-status">
          <div class="remote-pairing-status-copy">
            <p class="label">Transport status</p>
            <strong>{props.state?.statusLabel ?? "Remote pairing idle"}</strong>
          </div>
          <Show when={statusError()}>
            {(error) => <p class="remote-pairing-error">{error()}</p>}
          </Show>
        </div>
      </aside>
    </div>
  );
};

const StatusChip: Component<{ label: string; tone: "connected" | "default" }> = (props) => (
  <span
    classList={{
      chip: true,
      "chip-forest": props.tone === "connected",
    }}
  >
    {props.label}
  </span>
);

function formatPairingPhase(phase: RemotePairingState["phase"]): string {
  switch (phase) {
    case "offer-ready":
      return "Offer ready";
    case "answer-ready":
      return "Answer ready";
    case "connecting":
      return "Connecting";
    case "reconnecting":
      return "Reconnecting";
    case "connected":
      return "Connected";
    case "failed":
      return "Failed";
    case "idle":
      return "Idle";
  }
}

function formatPairingRole(role: NonNullable<RemotePairingState["role"]>): string {
  return role === "host" ? "Host" : "Peer";
}
