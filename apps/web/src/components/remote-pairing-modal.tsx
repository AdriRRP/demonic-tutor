import { For, Show, createEffect, createResource, createSignal } from "solid-js";
import type { Component } from "solid-js";
import QRCode from "qrcode";
import type { RemotePairingState } from "../lib/remote-pairing";
import { readQrSignalFromFile, supportsQrSignalImport } from "../lib/qr-signal-import";

interface RemotePairingModalProps {
  onAcceptOffer: (offerPayload: string) => Promise<void>;
  onApplyAnswer: (answerPayload: string) => Promise<void>;
  onBeginHosting: () => Promise<void>;
  onClose: () => void;
  onReset: () => void;
  state: RemotePairingState | null;
  supported: boolean;
}

type PendingAction =
  | "accept-offer"
  | "apply-answer"
  | "begin-hosting"
  | "copy-signal"
  | "import-host-answer-qr"
  | "import-peer-offer-qr"
  | "paste-host-answer"
  | "paste-peer-offer"
  | null;

type SignalInputTarget = "host-answer" | "peer-offer";

export const RemotePairingModal: Component<RemotePairingModalProps> = (props) => {
  const [hostAnswerInput, setHostAnswerInput] = createSignal("");
  const [peerOfferInput, setPeerOfferInput] = createSignal("");
  const [localError, setLocalError] = createSignal<string | null>(null);
  const [pendingAction, setPendingAction] = createSignal<PendingAction>(null);
  const [copyFeedback, setCopyFeedback] = createSignal<string | null>(null);
  let hostAnswerQrInput: HTMLInputElement | undefined;
  let peerOfferQrInput: HTMLInputElement | undefined;

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

  const pasteSignal = (target: SignalInputTarget) => {
    if (!clipboardReadSupported()) {
      setLocalError("This browser cannot read from the clipboard.");
      return;
    }

    void (async () => {
      setPendingAction(target === "host-answer" ? "paste-host-answer" : "paste-peer-offer");
      setCopyFeedback(null);
      setLocalError(null);

      try {
        const clipboardValue = (await navigator.clipboard.readText()).trim();
        if (clipboardValue.length === 0) {
          throw new Error("Clipboard is empty.");
        }

        if (target === "host-answer") {
          setHostAnswerInput(clipboardValue);
        } else {
          setPeerOfferInput(clipboardValue);
        }
      } catch (error) {
        setLocalError(error instanceof Error ? error.message : "Could not read the clipboard.");
      } finally {
        setPendingAction(null);
      }
    })();
  };

  const importSignalFromQr = (target: SignalInputTarget, file: File | undefined) => {
    if (!file) {
      return;
    }

    void (async () => {
      setPendingAction(target === "host-answer" ? "import-host-answer-qr" : "import-peer-offer-qr");
      setCopyFeedback(null);
      setLocalError(null);

      try {
        const decodedPayload = await readQrSignalFromFile(file);
        if (target === "host-answer") {
          setHostAnswerInput(decodedPayload);
        } else {
          setPeerOfferInput(decodedPayload);
        }
      } catch (error) {
        setLocalError(error instanceof Error ? error.message : "Could not read the selected QR.");
      } finally {
        setPendingAction(null);
      }
    })();
  };

  const statusError = () => localError() ?? props.state?.error ?? null;
  const hostSignal = () => (props.state?.role === "host" ? props.state.localSignal : "");
  const peerSignal = () => (props.state?.role === "peer" ? props.state.localSignal : "");
  const hostSignalSummary = () => summarizeSignal(hostSignal());
  const peerSignalSummary = () => summarizeSignal(peerSignal());

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

        <div class="remote-pairing-hero">
          <div class="remote-pairing-hero-copy">
            <p class="remote-pairing-note">
              Pair two browsers directly with manual WebRTC signaling. The host still owns the
              runtime, but this flow is optimized for quick copy/paste between devices instead of
              raw transport debugging.
            </p>
          </div>
          <div class="remote-pairing-hero-status">
            <StatusChip
              label={props.state ? formatPairingPhase(props.state.phase) : "Idle"}
              tone={props.state?.connected ? "connected" : "default"}
            />
            <Show when={props.state?.role}>
              {(role) => <StatusChip label={formatPairingRole(role())} tone="default" />}
            </Show>
          </div>
        </div>

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
            <section
              classList={{
                "remote-pairing-panel": true,
                "remote-pairing-panel-active": props.state?.role === "host",
              }}
            >
              <div class="remote-pairing-panel-head">
                <p class="label">Host</p>
                <strong>Open the room</strong>
              </div>

              <PairingSteps
                steps={[
                  "Generate the host offer.",
                  "Send it to the second device.",
                  "Paste the returned answer here.",
                ]}
              />

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
                <Show when={hostSignalSummary()}>
                  {(summary) => (
                    <SignalSummaryCard
                      bytes={summary().bytes}
                      kind={summary().kind}
                      lineCount={summary().lineCount}
                    />
                  )}
                </Show>
                <Show when={hostSignal()}>
                  {(payload) => <SignalQrCard kind="Offer QR" payload={payload()} />}
                </Show>
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
                <div class="remote-pairing-field-actions remote-pairing-field-actions-dual">
                  <Show when={clipboardReadSupported()}>
                    <button
                      class="hero-button hero-button-ghost mini-button"
                      type="button"
                      disabled={pendingAction() !== null}
                      onClick={() => {
                        pasteSignal("host-answer");
                      }}
                    >
                      {pendingAction() === "paste-host-answer" ? "Reading…" : "Paste answer"}
                    </button>
                  </Show>
                  <Show when={supportsQrSignalImport()}>
                    <button
                      class="hero-button hero-button-ghost mini-button"
                      type="button"
                      disabled={pendingAction() !== null}
                      onClick={() => {
                        hostAnswerQrInput?.click();
                      }}
                    >
                      {pendingAction() === "import-host-answer-qr" ? "Scanning…" : "Import QR"}
                    </button>
                  </Show>
                </div>
                <input
                  ref={(element) => {
                    hostAnswerQrInput = element;
                  }}
                  class="remote-pairing-file-input"
                  type="file"
                  accept="image/*"
                  capture="environment"
                  onChange={(event) => {
                    importSignalFromQr("host-answer", event.currentTarget.files?.[0]);
                    event.currentTarget.value = "";
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

            <section
              classList={{
                "remote-pairing-panel": true,
                "remote-pairing-panel-active": props.state?.role === "peer",
              }}
            >
              <div class="remote-pairing-panel-head">
                <p class="label">Join</p>
                <strong>Answer the host</strong>
              </div>

              <PairingSteps
                steps={[
                  "Paste the host offer from the first device.",
                  "Generate the peer answer.",
                  "Return the answer back to the host.",
                ]}
              />

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
                <div class="remote-pairing-field-actions remote-pairing-field-actions-dual">
                  <Show when={clipboardReadSupported()}>
                    <button
                      class="hero-button hero-button-ghost mini-button"
                      type="button"
                      disabled={pendingAction() !== null}
                      onClick={() => {
                        pasteSignal("peer-offer");
                      }}
                    >
                      {pendingAction() === "paste-peer-offer" ? "Reading…" : "Paste offer"}
                    </button>
                  </Show>
                  <Show when={supportsQrSignalImport()}>
                    <button
                      class="hero-button hero-button-ghost mini-button"
                      type="button"
                      disabled={pendingAction() !== null}
                      onClick={() => {
                        peerOfferQrInput?.click();
                      }}
                    >
                      {pendingAction() === "import-peer-offer-qr" ? "Scanning…" : "Import QR"}
                    </button>
                  </Show>
                </div>
                <input
                  ref={(element) => {
                    peerOfferQrInput = element;
                  }}
                  class="remote-pairing-file-input"
                  type="file"
                  accept="image/*"
                  capture="environment"
                  onChange={(event) => {
                    importSignalFromQr("peer-offer", event.currentTarget.files?.[0]);
                    event.currentTarget.value = "";
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
                <Show when={peerSignalSummary()}>
                  {(summary) => (
                    <SignalSummaryCard
                      bytes={summary().bytes}
                      kind={summary().kind}
                      lineCount={summary().lineCount}
                    />
                  )}
                </Show>
                <Show when={peerSignal()}>
                  {(payload) => <SignalQrCard kind="Answer QR" payload={payload()} />}
                </Show>
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
          <Show when={supportsQrSignalImport()}>
            <p class="remote-pairing-note-inline">
              QR import uses native browser detection from an image or live camera capture when the
              device offers it.
            </p>
          </Show>
          <Show when={statusError()}>
            {(error) => <p class="remote-pairing-error">{error()}</p>}
          </Show>
        </div>
      </aside>
    </div>
  );
};

const PairingSteps: Component<{ steps: string[] }> = (props) => (
  <ol class="remote-pairing-steps">
    <For each={props.steps}>
      {(step, index) => (
        <li class="remote-pairing-step">
          <span class="remote-pairing-step-index">{String(index() + 1)}</span>
          <span>{step}</span>
        </li>
      )}
    </For>
  </ol>
);

const SignalSummaryCard: Component<{ kind: string; bytes: number; lineCount: number }> = (
  props,
) => (
  <div class="remote-pairing-signal-summary">
    <span class="remote-pairing-signal-chip">{props.kind}</span>
    <span>{formatSignalBytes(props.bytes)}</span>
    <span>{`${String(props.lineCount)} line${props.lineCount === 1 ? "" : "s"}`}</span>
  </div>
);

const SignalQrCard: Component<{ kind: string; payload: string }> = (props) => {
  const [qrMarkup] = createResource(
    () => props.payload,
    async (payload) =>
      QRCode.toString(payload, {
        color: {
          dark: "#f5d596",
          light: "#0000",
        },
        errorCorrectionLevel: "L",
        margin: 1,
        type: "svg",
        width: 232,
      }),
  );

  return (
    <div class="remote-pairing-qr-card">
      <div class="remote-pairing-qr-head">
        <span class="remote-pairing-signal-chip">{props.kind}</span>
        <span>Scan on the other device or keep using copy/paste.</span>
      </div>
      <Show
        when={qrMarkup()}
        fallback={
          <div class="remote-pairing-qr-shell remote-pairing-qr-shell-pending">Rendering QR…</div>
        }
      >
        {(markup) => <div class="remote-pairing-qr-shell" innerHTML={markup()} />}
      </Show>
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

function clipboardReadSupported(): boolean {
  return (
    typeof navigator !== "undefined" &&
    "clipboard" in navigator &&
    typeof navigator.clipboard.readText === "function"
  );
}

function summarizeSignal(
  payload: string,
): { kind: string; bytes: number; lineCount: number } | null {
  if (!payload) {
    return null;
  }

  let kind = "Signal";

  try {
    const parsedPayload = JSON.parse(payload) as { type?: unknown };
    if (parsedPayload.type === "offer") {
      kind = "Offer";
    } else if (parsedPayload.type === "answer") {
      kind = "Answer";
    }
  } catch {
    kind = "Signal";
  }

  return {
    kind,
    bytes: new TextEncoder().encode(payload).length,
    lineCount: payload.split("\n").length,
  };
}

function formatSignalBytes(byteLength: number): string {
  return `${String(Math.max(1, Math.round(byteLength / 1024)))} KB`;
}
