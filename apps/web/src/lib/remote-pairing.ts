export type RemotePairingRole = "host" | "peer" | null;

export type RemotePairingPhase =
  | "idle"
  | "offer-ready"
  | "answer-ready"
  | "connecting"
  | "reconnecting"
  | "connected"
  | "failed";

export interface RemotePairingState {
  role: RemotePairingRole;
  phase: RemotePairingPhase;
  connected: boolean;
  localSignal: string;
  error: string | null;
  statusLabel: string;
}

type RemotePairingListener = (state: RemotePairingState) => void;

export interface RemotePairingTransport {
  id: string;
  role: Exclude<RemotePairingRole, null>;
  send(payload: string): void;
  subscribe(listener: RemoteTransportListener): () => void;
  subscribeState(listener: RemoteTransportStateListener): () => void;
}

type RemoteTransportListener = (payload: string) => void;
type RemoteTransportStateListener = (state: RemotePairingState) => void;

interface SignalEnvelope {
  sdp: string;
  type: "answer" | "offer";
  version: 1;
}

const DEFAULT_PAIRING_STATE: RemotePairingState = {
  connected: false,
  error: null,
  localSignal: "",
  phase: "idle",
  role: null,
  statusLabel: "Remote pairing idle",
};

const REMOTE_ICE_SERVERS: RTCIceServer[] = [
  {
    urls: ["stun:stun.l.google.com:19302"],
  },
];

export interface RemotePairingController {
  acceptOfferAndCreateAnswer(offerPayload: string): Promise<string>;
  applyAnswer(answerPayload: string): Promise<void>;
  beginHosting(): Promise<string>;
  destroy(): void;
  reset(): void;
  snapshot(): RemotePairingState;
  subscribe(listener: RemotePairingListener): () => void;
  transport(): RemotePairingTransport | null;
}

export function createRemotePairingController(): RemotePairingController {
  return new WebRtcPairingController();
}

class WebRtcPairingController implements RemotePairingController {
  private readonly listeners = new Set<RemotePairingListener>();
  private readonly transportListeners = new Set<RemoteTransportListener>();
  private readonly transportStateListeners = new Set<RemoteTransportStateListener>();
  private channel: RTCDataChannel | null = null;
  private connection: RTCPeerConnection | null = null;
  private state: RemotePairingState = { ...DEFAULT_PAIRING_STATE };
  private transportId: string | null = null;

  public snapshot(): RemotePairingState {
    return { ...this.state };
  }

  public subscribe(listener: RemotePairingListener): () => void {
    this.listeners.add(listener);
    listener(this.snapshot());

    return () => {
      this.listeners.delete(listener);
    };
  }

  public transport(): RemotePairingTransport | null {
    const role = this.state.role;
    const channel = this.channel;
    const transportId = this.transportId;

    if (
      role === null ||
      channel === null ||
      transportId === null ||
      !this.state.connected ||
      channel.readyState !== "open"
    ) {
      return null;
    }

    return {
      id: transportId,
      role,
      send: (payload: string) => {
        if (this.channel?.readyState !== "open") {
          throw new Error("Remote duel transport is not connected.");
        }

        this.channel.send(payload);
      },
      subscribe: (listener: RemoteTransportListener) => {
        this.transportListeners.add(listener);
        return () => {
          this.transportListeners.delete(listener);
        };
      },
      subscribeState: (listener: RemoteTransportStateListener) => {
        this.transportStateListeners.add(listener);
        listener(this.snapshot());
        return () => {
          this.transportStateListeners.delete(listener);
        };
      },
    };
  }

  public async beginHosting(): Promise<string> {
    this.reset();

    const connection = this.initializeConnection("host");
    const channel = connection.createDataChannel("demonictutor-remote-duel", {
      ordered: true,
    });
    this.attachChannel(channel);

    const offer = await connection.createOffer();
    await connection.setLocalDescription(offer);
    await waitForIceGatheringComplete(connection);

    const localDescription = connection.localDescription;
    if (!localDescription) {
      throw new Error("Remote pairing could not produce an offer.");
    }

    const payload = serializeSignal(localDescription);
    this.setState({
      localSignal: payload,
      phase: "offer-ready",
      role: "host",
      statusLabel: "Offer ready. Share it with the remote peer.",
    });
    return payload;
  }

  public async acceptOfferAndCreateAnswer(offerPayload: string): Promise<string> {
    this.reset();

    const offer = parseSignal(offerPayload, "offer");
    const connection = this.initializeConnection("peer");

    await connection.setRemoteDescription(offer);
    const answer = await connection.createAnswer();
    await connection.setLocalDescription(answer);
    await waitForIceGatheringComplete(connection);

    const localDescription = connection.localDescription;
    if (!localDescription) {
      throw new Error("Remote pairing could not produce an answer.");
    }

    const payload = serializeSignal(localDescription);
    this.setState({
      localSignal: payload,
      phase: "answer-ready",
      role: "peer",
      statusLabel: "Answer ready. Return it to the host to finish pairing.",
    });
    return payload;
  }

  public async applyAnswer(answerPayload: string): Promise<void> {
    const connection = this.connection;
    if (!connection || this.state.role !== "host") {
      throw new Error("Host offer must be created before applying an answer.");
    }

    const answer = parseSignal(answerPayload, "answer");
    this.setState({
      phase: "connecting",
      statusLabel: "Connecting remote duel channel…",
    });
    await connection.setRemoteDescription(answer);
  }

  public reset(): void {
    this.channel?.close();
    this.channel = null;

    this.connection?.close();
    this.connection = null;
    this.transportId = null;

    this.setState({ ...DEFAULT_PAIRING_STATE });
  }

  public destroy(): void {
    this.reset();
    this.listeners.clear();
  }

  private initializeConnection(role: Exclude<RemotePairingRole, null>): RTCPeerConnection {
    const connection = new RTCPeerConnection({ iceServers: REMOTE_ICE_SERVERS });
    this.connection = connection;
    this.transportId = createRemotePairingId();

    connection.addEventListener("connectionstatechange", () => {
      const wasConnected = this.state.connected || this.state.phase === "reconnecting";

      switch (connection.connectionState) {
        case "new":
          this.setState({
            connected: false,
            error: null,
            phase: "connecting",
            role,
            statusLabel: role === "host" ? "Preparing host offer…" : "Accepting host offer…",
          });
          break;
        case "connected":
          this.setState({
            connected: true,
            error: null,
            phase: "connected",
            role,
            statusLabel: "Remote duel channel connected.",
          });
          break;
        case "connecting":
          this.setState({
            error: null,
            phase: wasConnected ? "reconnecting" : "connecting",
            role,
            statusLabel: wasConnected
              ? "Remote duel channel interrupted. Reconnecting…"
              : "Connecting remote duel channel…",
          });
          break;
        case "failed":
          this.setState({
            connected: false,
            error: "Remote pairing failed. Check the signaling payloads and try again.",
            phase: "failed",
            role,
            statusLabel: "Remote pairing failed.",
          });
          break;
        case "disconnected":
          this.setState({
            connected: false,
            error: null,
            phase: "reconnecting",
            role,
            statusLabel: "Remote duel channel interrupted. Reconnecting…",
          });
          break;
        case "closed":
          this.setState({
            connected: false,
            phase: "idle",
            role: null,
            statusLabel: "Remote pairing idle",
          });
          break;
      }
    });

    connection.addEventListener("datachannel", (event) => {
      this.attachChannel(event.channel);
    });

    this.setState({
      connected: false,
      error: null,
      phase: "connecting",
      role,
      statusLabel: role === "host" ? "Preparing host offer…" : "Accepting host offer…",
    });

    return connection;
  }

  private attachChannel(channel: RTCDataChannel): void {
    this.channel = channel;

    channel.addEventListener("open", () => {
      this.setState({
        connected: true,
        error: null,
        phase: "connected",
        statusLabel: "Remote duel channel connected.",
      });
    });

    channel.addEventListener("message", (event) => {
      if (typeof event.data !== "string") {
        return;
      }

      for (const listener of this.transportListeners) {
        listener(event.data);
      }
    });

    channel.addEventListener("close", () => {
      if (this.state.phase === "connected") {
        this.setState({
          connected: false,
          phase: "failed",
          statusLabel: "Remote duel channel closed.",
        });
      }
    });

    channel.addEventListener("error", () => {
      this.setState({
        connected: false,
        error: "Remote duel channel reported an error.",
        phase: "failed",
        statusLabel: "Remote duel channel error.",
      });
    });
  }

  private setState(nextState: Partial<RemotePairingState>): void {
    this.state = {
      ...this.state,
      ...nextState,
    };

    const snapshot = this.snapshot();
    for (const listener of this.listeners) {
      listener(snapshot);
    }
    for (const listener of this.transportStateListeners) {
      listener(snapshot);
    }
  }
}

function createRemotePairingId(): string {
  if ("randomUUID" in window.crypto) {
    return window.crypto.randomUUID();
  }

  return `${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 10)}`;
}

async function waitForIceGatheringComplete(connection: RTCPeerConnection): Promise<void> {
  if (connection.iceGatheringState === "complete") {
    return;
  }

  await new Promise<void>((resolve) => {
    const handleIceGatheringStateChange = () => {
      if (connection.iceGatheringState !== "complete") {
        return;
      }

      connection.removeEventListener("icegatheringstatechange", handleIceGatheringStateChange);
      resolve();
    };

    connection.addEventListener("icegatheringstatechange", handleIceGatheringStateChange);
  });
}

function serializeSignal(description: RTCSessionDescriptionInit): string {
  if (description.sdp === undefined) {
    throw new Error("The local signaling payload is missing SDP.");
  }

  if (description.type !== "offer" && description.type !== "answer") {
    throw new Error("The local signaling payload has an unsupported type.");
  }

  return JSON.stringify({
    sdp: description.sdp,
    type: description.type,
    version: 1,
  } satisfies SignalEnvelope);
}

function parseSignal(
  rawPayload: string,
  expectedType: SignalEnvelope["type"],
): RTCSessionDescriptionInit {
  let parsedPayload: unknown;

  try {
    parsedPayload = JSON.parse(rawPayload);
  } catch {
    throw new Error("The remote signaling payload is not valid JSON.");
  }

  if (!isSignalEnvelope(parsedPayload)) {
    throw new Error("The remote signaling payload has an unsupported shape.");
  }

  if (parsedPayload.type !== expectedType) {
    throw new Error(`Expected a ${expectedType} payload.`);
  }

  return {
    sdp: parsedPayload.sdp,
    type: parsedPayload.type,
  };
}

function isSignalEnvelope(value: unknown): value is SignalEnvelope {
  return (
    typeof value === "object" &&
    value !== null &&
    "version" in value &&
    "type" in value &&
    "sdp" in value &&
    value.version === 1 &&
    (value.type === "offer" || value.type === "answer") &&
    typeof value.sdp === "string"
  );
}
