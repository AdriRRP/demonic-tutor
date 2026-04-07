import type { RemotePairingTransport } from "./remote-pairing";
import {
  activateAbility,
  advanceTurn,
  castSpell,
  concede,
  createArenaClient,
  declareAttackers,
  declareBlockers,
  discardForCleanup,
  passPriority,
  playLand,
  readState,
  resetArena,
  resolveCombatDamage,
  resolveOptionalEffect,
  resolvePendingHandChoice,
  resolvePendingScry,
  resolvePendingSurveil,
  tapManaSource,
  type ArenaCommandTarget,
} from "./runtime";
import type { ArenaState, BlockerAssignmentInput } from "./types";

const CHANNEL_PREFIX = "demonictutor-duel:";
const ROOM_QUERY_PARAM = "duel";
const DISCOVERY_TIMEOUT_MS = 180;
const HOST_STARTUP_GRACE_MS = 1_200;
const REQUEST_TIMEOUT_MS = 4_000;

type SessionListener = (state: ArenaState) => void;

type ArenaSessionRole = "host" | "peer";
type ArenaSessionTransport = "broadcast-channel" | "embedded" | "webrtc";

interface HelloMessage {
  type: "hello";
  from: string;
}

interface GoodbyeMessage {
  type: "goodbye";
  from: string;
}

interface StateSyncMessage {
  type: "state-sync";
  from: string;
  state: ArenaState;
}

type ArenaCommandRequest =
  | { kind: "reset" }
  | { kind: "pass_priority"; playerId: string }
  | { kind: "advance_turn" }
  | { kind: "concede"; playerId: string }
  | { kind: "play_land"; playerId: string; cardId: string }
  | { kind: "tap_mana_source"; playerId: string; cardId: string }
  | { kind: "cast_spell"; playerId: string; cardId: string }
  | { kind: "activate_ability"; playerId: string; cardId: string }
  | { kind: "declare_attackers"; playerId: string; attackerIds: string[] }
  | { kind: "declare_blockers"; playerId: string; assignments: BlockerAssignmentInput[] }
  | { kind: "resolve_combat_damage"; playerId: string }
  | { kind: "discard_for_cleanup"; playerId: string; cardId: string }
  | { kind: "resolve_optional_effect"; playerId: string; accept: boolean }
  | { kind: "resolve_pending_hand_choice"; playerId: string; cardId: string }
  | { kind: "resolve_pending_scry"; playerId: string; moveToBottom: boolean }
  | { kind: "resolve_pending_surveil"; playerId: string; moveToGraveyard: boolean };

interface CommandRequestMessage {
  type: "command-request";
  from: string;
  requestId: string;
  command: ArenaCommandRequest;
}

interface CommandResponseSuccessMessage {
  type: "command-response";
  from: string;
  requestId: string;
  ok: true;
  state: ArenaState;
}

interface CommandResponseErrorMessage {
  type: "command-response";
  from: string;
  requestId: string;
  ok: false;
  error: string;
  state: ArenaState | null;
}

type CommandResponseMessage = CommandResponseSuccessMessage | CommandResponseErrorMessage;

type SessionMessage =
  | HelloMessage
  | GoodbyeMessage
  | StateSyncMessage
  | CommandRequestMessage
  | CommandResponseMessage;

interface PendingRequest {
  reject: (reason?: unknown) => void;
  resolve: (state: ArenaState) => void;
  timeoutId: number;
}

interface BroadcastDiscoveryPeerResult {
  hostId: string;
  initialState: ArenaState;
  role: "peer";
}

interface BroadcastDiscoveryHostResult {
  role: "host";
}

type BroadcastDiscoveryResult = BroadcastDiscoveryPeerResult | BroadcastDiscoveryHostResult;

export interface ArenaSessionInfo {
  inviteUrl: string;
  localSeatId: string | null;
  role: ArenaSessionRole;
  roomId: string;
  transport: ArenaSessionTransport;
}

export interface ArenaSession extends ArenaCommandTarget {
  destroy(): void;
  info(): ArenaSessionInfo;
  subscribe(listener: SessionListener): () => void;
}

export async function createRemotePeerSession({
  inviteUrl,
  roomId,
  transport,
}: {
  inviteUrl: string;
  roomId: string;
  transport: RemotePairingTransport;
}): Promise<ArenaSession> {
  const session = new RemotePeerArenaSession({
    inviteUrl,
    roomId,
    transport,
  });

  await session.state();
  return session;
}

export function attachRemoteCommandRelay(
  session: ArenaSession,
  transport: RemotePairingTransport,
): () => void {
  const instanceId = `remote-host-${createSessionId()}`;
  const unsubscribeSession = session.subscribe((nextState) => {
    transport.send(
      JSON.stringify({
        from: instanceId,
        state: projectRemotePeerState(nextState),
        type: "state-sync",
      } satisfies StateSyncMessage),
    );
  });

  const unsubscribeTransport = transport.subscribe((payload) => {
    const message = parseTransportPayload(payload);
    if (!message || message.from === instanceId) {
      return;
    }

    switch (message.type) {
      case "hello":
        void sendRemoteStateSync(session, transport, instanceId);
        break;
      case "command-request":
        void respondToRemoteCommand(session, transport, instanceId, message);
        break;
      case "goodbye":
      case "state-sync":
      case "command-response":
        break;
    }
  });

  return () => {
    unsubscribeSession();
    unsubscribeTransport();
  };
}

export async function createArenaSession(): Promise<ArenaSession> {
  const roomId = ensureRoomIdInUrl();
  const inviteUrl = window.location.href;

  if (typeof BroadcastChannel === "undefined") {
    return createEmbeddedSession(roomId, inviteUrl);
  }

  const channel = new BroadcastChannel(`${CHANNEL_PREFIX}${roomId}`);
  const instanceId = createSessionId();
  const discovery = await discoverBroadcastRole(channel, instanceId);

  if (discovery.role === "peer") {
    return new PeerArenaSession({
      channel,
      hostId: discovery.hostId,
      initialState: discovery.initialState,
      inviteUrl,
      instanceId,
      roomId,
    });
  }

  const client = await createArenaClient();
  const initialState = await readState(client);

  return new HostArenaSession({
    channel,
    client,
    initialState,
    inviteUrl,
    instanceId,
    roomId,
    transport: "broadcast-channel",
  });
}

class HostArenaSession implements ArenaSession {
  private readonly channel: BroadcastChannel | null;
  private readonly client: ArenaCommandTarget;
  private readonly infoState: ArenaSessionInfo;
  private readonly instanceId: string;
  private readonly listeners = new Set<SessionListener>();
  private readonly remotePeerIds = new Set<string>();
  private readonly handleMessage = (event: MessageEvent<unknown>): void => {
    const message = coerceSessionMessage(event.data);
    if (!message || message.from === this.instanceId) {
      return;
    }

    switch (message.type) {
      case "hello": {
        const nextPeerIds = new Set(this.remotePeerIds);
        nextPeerIds.add(message.from);
        this.setRemotePeers(nextPeerIds);
        this.broadcastState();
        break;
      }
      case "goodbye": {
        const nextPeerIds = new Set(this.remotePeerIds);
        nextPeerIds.delete(message.from);
        this.setRemotePeers(nextPeerIds);
        break;
      }
      case "command-request": {
        const nextPeerIds = new Set(this.remotePeerIds);
        nextPeerIds.add(message.from);
        this.setRemotePeers(nextPeerIds);
        void this.respondToCommand(message);
        break;
      }
      case "state-sync":
      case "command-response":
        break;
    }
  };

  private stateCache: ArenaState;

  constructor({
    channel,
    client,
    initialState,
    inviteUrl,
    instanceId,
    roomId,
    transport,
  }: {
    channel: BroadcastChannel | null;
    client: ArenaCommandTarget;
    initialState: ArenaState;
    inviteUrl: string;
    instanceId: string;
    roomId: string;
    transport: ArenaSessionTransport;
  }) {
    this.channel = channel;
    this.client = client;
    this.infoState = {
      inviteUrl,
      localSeatId: seatForRole(initialState, "host"),
      role: "host",
      roomId,
      transport,
    };
    this.instanceId = instanceId;
    this.stateCache = initialState;
    this.refreshLocalSeatId();
    this.channel?.addEventListener("message", this.handleMessage);
    this.broadcastState();
  }

  public subscribe(listener: SessionListener): () => void {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }

  public info(): ArenaSessionInfo {
    return { ...this.infoState };
  }

  public destroy(): void {
    this.channel?.postMessage({
      from: this.instanceId,
      type: "goodbye",
    } satisfies GoodbyeMessage);
    this.channel?.removeEventListener("message", this.handleMessage);
    this.channel?.close();
  }

  public state(): Promise<ArenaState> {
    return Promise.resolve(this.stateCache);
  }

  public reset(): Promise<ArenaState> {
    return this.applyLocalCommand(() => resetArena(this.client));
  }

  public pass_priority(playerId: string): Promise<ArenaState> {
    return this.applyLocalCommand(() => passPriority(this.client, playerId));
  }

  public advance_turn(): Promise<ArenaState> {
    return this.applyLocalCommand(() => advanceTurn(this.client));
  }

  public concede(playerId: string): Promise<ArenaState> {
    return this.applyLocalCommand(() => concede(this.client, playerId));
  }

  public play_land(playerId: string, cardId: string): Promise<ArenaState> {
    return this.applyLocalCommand(() => playLand(this.client, playerId, cardId));
  }

  public tap_mana_source(playerId: string, cardId: string): Promise<ArenaState> {
    return this.applyLocalCommand(() => tapManaSource(this.client, playerId, cardId));
  }

  public cast_spell(playerId: string, cardId: string): Promise<ArenaState> {
    return this.applyLocalCommand(() => castSpell(this.client, playerId, cardId));
  }

  public activate_ability(playerId: string, cardId: string): Promise<ArenaState> {
    return this.applyLocalCommand(() => activateAbility(this.client, playerId, cardId));
  }

  public declare_attackers(playerId: string, attackerIds: string[]): Promise<ArenaState> {
    return this.applyLocalCommand(() => declareAttackers(this.client, playerId, attackerIds));
  }

  public declare_blockers(
    playerId: string,
    assignments: BlockerAssignmentInput[],
  ): Promise<ArenaState> {
    return this.applyLocalCommand(() => declareBlockers(this.client, playerId, assignments));
  }

  public resolve_combat_damage(playerId: string): Promise<ArenaState> {
    return this.applyLocalCommand(() => resolveCombatDamage(this.client, playerId));
  }

  public discard_for_cleanup(playerId: string, cardId: string): Promise<ArenaState> {
    return this.applyLocalCommand(() => discardForCleanup(this.client, playerId, cardId));
  }

  public resolve_optional_effect(playerId: string, accept: boolean): Promise<ArenaState> {
    return this.applyLocalCommand(() => resolveOptionalEffect(this.client, playerId, accept));
  }

  public resolve_pending_hand_choice(playerId: string, cardId: string): Promise<ArenaState> {
    return this.applyLocalCommand(() => resolvePendingHandChoice(this.client, playerId, cardId));
  }

  public resolve_pending_scry(playerId: string, moveToBottom: boolean): Promise<ArenaState> {
    return this.applyLocalCommand(() => resolvePendingScry(this.client, playerId, moveToBottom));
  }

  public resolve_pending_surveil(playerId: string, moveToGraveyard: boolean): Promise<ArenaState> {
    return this.applyLocalCommand(() =>
      resolvePendingSurveil(this.client, playerId, moveToGraveyard),
    );
  }

  private async applyLocalCommand(operation: () => Promise<ArenaState>): Promise<ArenaState> {
    const nextState = await operation();
    this.commitState(nextState);
    return nextState;
  }

  private commitState(nextState: ArenaState): void {
    this.stateCache = nextState;
    this.refreshLocalSeatId();
    this.broadcastState();
    this.notifyListeners(nextState);
  }

  private refreshLocalSeatId(): void {
    this.infoState.localSeatId = seatForRole(this.stateCache, "host");
  }

  private setRemotePeers(peerIds: Set<string>): void {
    if (samePeerSet(this.remotePeerIds, peerIds)) {
      return;
    }

    this.remotePeerIds.clear();
    for (const peerId of peerIds) {
      this.remotePeerIds.add(peerId);
    }

    this.refreshLocalSeatId();
    this.notifyListeners(this.stateCache);
  }

  private notifyListeners(nextState: ArenaState): void {
    for (const listener of this.listeners) {
      listener(nextState);
    }
  }

  private broadcastState(): void {
    this.channel?.postMessage({
      from: this.instanceId,
      state: this.stateCache,
      type: "state-sync",
    } satisfies StateSyncMessage);
  }

  private async respondToCommand(message: CommandRequestMessage): Promise<void> {
    try {
      assertCommandMatchesBoundSeat(message.command, seatForRole(this.stateCache, "peer"));
      const nextState = await runRequestedCommand(this.client, message.command);
      this.commitState(nextState);
      this.channel?.postMessage({
        from: this.instanceId,
        ok: true,
        requestId: message.requestId,
        state: nextState,
        type: "command-response",
      } satisfies CommandResponseSuccessMessage);
    } catch (error) {
      this.channel?.postMessage({
        error: formatSessionError(error),
        from: this.instanceId,
        ok: false,
        requestId: message.requestId,
        state: this.stateCache,
        type: "command-response",
      } satisfies CommandResponseErrorMessage);
    }
  }
}

class PeerArenaSession implements ArenaSession {
  private readonly channel: BroadcastChannel;
  private readonly hostId: string;
  private readonly infoState: ArenaSessionInfo;
  private readonly instanceId: string;
  private readonly listeners = new Set<SessionListener>();
  private readonly pendingRequests = new Map<string, PendingRequest>();
  private readonly handleMessage = (event: MessageEvent<unknown>): void => {
    const message = coerceSessionMessage(event.data);
    if (!message || message.from === this.instanceId) {
      return;
    }

    if (message.type === "state-sync" && message.from === this.hostId) {
      this.stateCache = message.state;
      this.infoState.localSeatId = seatForRole(message.state, "peer");
      this.notifyListeners(message.state);
      return;
    }

    if (message.type === "command-response" && message.from === this.hostId) {
      const pendingRequest = this.pendingRequests.get(message.requestId);
      if (!pendingRequest) {
        return;
      }

      this.pendingRequests.delete(message.requestId);
      window.clearTimeout(pendingRequest.timeoutId);

      if (message.ok) {
        this.stateCache = message.state;
        this.infoState.localSeatId = seatForRole(message.state, "peer");
        this.notifyListeners(message.state);
        pendingRequest.resolve(message.state);
      } else {
        if (message.state !== null) {
          this.stateCache = message.state;
          this.infoState.localSeatId = seatForRole(message.state, "peer");
          this.notifyListeners(message.state);
        }
        pendingRequest.reject(new Error(message.error));
      }
    }
  };

  private stateCache: ArenaState;

  constructor({
    channel,
    hostId,
    initialState,
    inviteUrl,
    instanceId,
    roomId,
  }: {
    channel: BroadcastChannel;
    hostId: string;
    initialState: ArenaState;
    inviteUrl: string;
    instanceId: string;
    roomId: string;
  }) {
    this.channel = channel;
    this.hostId = hostId;
    this.infoState = {
      inviteUrl,
      localSeatId: seatForRole(initialState, "peer"),
      role: "peer",
      roomId,
      transport: "broadcast-channel",
    };
    this.instanceId = instanceId;
    this.stateCache = initialState;
    this.channel.addEventListener("message", this.handleMessage);
    this.channel.postMessage({
      from: this.instanceId,
      type: "hello",
    } satisfies HelloMessage);
  }

  public subscribe(listener: SessionListener): () => void {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }

  public info(): ArenaSessionInfo {
    return { ...this.infoState };
  }

  public destroy(): void {
    for (const pendingRequest of this.pendingRequests.values()) {
      window.clearTimeout(pendingRequest.timeoutId);
      pendingRequest.reject(new Error("Peer session destroyed"));
    }
    this.pendingRequests.clear();
    this.channel.postMessage({
      from: this.instanceId,
      type: "goodbye",
    } satisfies GoodbyeMessage);
    this.channel.removeEventListener("message", this.handleMessage);
    this.channel.close();
  }

  public state(): Promise<ArenaState> {
    return Promise.resolve(this.stateCache);
  }

  public reset(): Promise<ArenaState> {
    return this.sendCommand({ kind: "reset" });
  }

  public pass_priority(playerId: string): Promise<ArenaState> {
    return this.sendCommand({ kind: "pass_priority", playerId });
  }

  public advance_turn(): Promise<ArenaState> {
    return this.sendCommand({ kind: "advance_turn" });
  }

  public concede(playerId: string): Promise<ArenaState> {
    return this.sendCommand({ kind: "concede", playerId });
  }

  public play_land(playerId: string, cardId: string): Promise<ArenaState> {
    return this.sendCommand({ kind: "play_land", playerId, cardId });
  }

  public tap_mana_source(playerId: string, cardId: string): Promise<ArenaState> {
    return this.sendCommand({ kind: "tap_mana_source", playerId, cardId });
  }

  public cast_spell(playerId: string, cardId: string): Promise<ArenaState> {
    return this.sendCommand({ kind: "cast_spell", playerId, cardId });
  }

  public activate_ability(playerId: string, cardId: string): Promise<ArenaState> {
    return this.sendCommand({ kind: "activate_ability", playerId, cardId });
  }

  public declare_attackers(playerId: string, attackerIds: string[]): Promise<ArenaState> {
    return this.sendCommand({ kind: "declare_attackers", attackerIds, playerId });
  }

  public declare_blockers(
    playerId: string,
    assignments: BlockerAssignmentInput[],
  ): Promise<ArenaState> {
    return this.sendCommand({ assignments, kind: "declare_blockers", playerId });
  }

  public resolve_combat_damage(playerId: string): Promise<ArenaState> {
    return this.sendCommand({ kind: "resolve_combat_damage", playerId });
  }

  public discard_for_cleanup(playerId: string, cardId: string): Promise<ArenaState> {
    return this.sendCommand({ kind: "discard_for_cleanup", playerId, cardId });
  }

  public resolve_optional_effect(playerId: string, accept: boolean): Promise<ArenaState> {
    return this.sendCommand({ accept, kind: "resolve_optional_effect", playerId });
  }

  public resolve_pending_hand_choice(playerId: string, cardId: string): Promise<ArenaState> {
    return this.sendCommand({ cardId, kind: "resolve_pending_hand_choice", playerId });
  }

  public resolve_pending_scry(playerId: string, moveToBottom: boolean): Promise<ArenaState> {
    return this.sendCommand({ kind: "resolve_pending_scry", moveToBottom, playerId });
  }

  public resolve_pending_surveil(playerId: string, moveToGraveyard: boolean): Promise<ArenaState> {
    return this.sendCommand({ kind: "resolve_pending_surveil", moveToGraveyard, playerId });
  }

  private notifyListeners(nextState: ArenaState): void {
    for (const listener of this.listeners) {
      listener(nextState);
    }
  }

  private sendCommand(command: ArenaCommandRequest): Promise<ArenaState> {
    assertCommandMatchesBoundSeat(command, this.infoState.localSeatId);
    const requestId = createSessionId();

    return new Promise<ArenaState>((resolve, reject) => {
      const timeoutId = window.setTimeout(() => {
        this.pendingRequests.delete(requestId);
        reject(new Error("The host window did not respond in time."));
      }, REQUEST_TIMEOUT_MS);

      this.pendingRequests.set(requestId, { reject, resolve, timeoutId });

      this.channel.postMessage({
        command,
        from: this.instanceId,
        requestId,
        type: "command-request",
      } satisfies CommandRequestMessage);
    });
  }
}

class RemotePeerArenaSession implements ArenaSession {
  private readonly infoState: ArenaSessionInfo;
  private readonly instanceId: string;
  private readonly listeners = new Set<SessionListener>();
  private readonly pendingRequests = new Map<string, PendingRequest>();
  private readonly transport: RemotePairingTransport;
  private readonly unsubscribeTransport: () => void;
  private initialStatePromise: Promise<ArenaState>;
  private resolveInitialState!: (state: ArenaState) => void;
  private rejectInitialState!: (reason?: unknown) => void;
  private stateCache: ArenaState | null = null;

  constructor({
    inviteUrl,
    roomId,
    transport,
  }: {
    inviteUrl: string;
    roomId: string;
    transport: RemotePairingTransport;
  }) {
    this.transport = transport;
    this.instanceId = `remote-peer-${createSessionId()}`;
    this.infoState = {
      inviteUrl,
      localSeatId: null,
      role: "peer",
      roomId,
      transport: "webrtc",
    };
    this.initialStatePromise = new Promise<ArenaState>((resolve, reject) => {
      this.resolveInitialState = resolve;
      this.rejectInitialState = reject;
    });
    this.unsubscribeTransport = this.transport.subscribe((payload) => {
      const message = parseTransportPayload(payload);
      if (!message) {
        return;
      }

      if (message.type === "state-sync") {
        this.commitState(message.state);
        return;
      }

      if (message.type !== "command-response") {
        return;
      }

      const pendingRequest = this.pendingRequests.get(message.requestId);
      if (!pendingRequest) {
        return;
      }

      this.pendingRequests.delete(message.requestId);
      window.clearTimeout(pendingRequest.timeoutId);

      if (message.ok) {
        this.commitState(message.state);
        pendingRequest.resolve(message.state);
      } else {
        if (message.state !== null) {
          this.commitState(message.state);
        }
        pendingRequest.reject(new Error(message.error));
      }
    });

    this.transport.send(
      JSON.stringify({
        from: this.instanceId,
        type: "hello",
      } satisfies HelloMessage),
    );
  }

  public subscribe(listener: SessionListener): () => void {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }

  public info(): ArenaSessionInfo {
    return { ...this.infoState };
  }

  public destroy(): void {
    for (const pendingRequest of this.pendingRequests.values()) {
      window.clearTimeout(pendingRequest.timeoutId);
      pendingRequest.reject(new Error("Remote peer session destroyed"));
    }
    this.pendingRequests.clear();
    this.rejectInitialState(new Error("Remote peer session destroyed"));
    try {
      this.transport.send(
        JSON.stringify({
          from: this.instanceId,
          type: "goodbye",
        } satisfies GoodbyeMessage),
      );
    } catch {
      // Ignore teardown transport failures; resilience is handled by later slices.
    }
    this.unsubscribeTransport();
  }

  public state(): Promise<ArenaState> {
    if (this.stateCache !== null) {
      return Promise.resolve(this.stateCache);
    }

    return this.initialStatePromise;
  }

  public reset(): Promise<ArenaState> {
    return this.sendCommand({ kind: "reset" });
  }

  public pass_priority(playerId: string): Promise<ArenaState> {
    return this.sendCommand({ kind: "pass_priority", playerId });
  }

  public advance_turn(): Promise<ArenaState> {
    return this.sendCommand({ kind: "advance_turn" });
  }

  public concede(playerId: string): Promise<ArenaState> {
    return this.sendCommand({ kind: "concede", playerId });
  }

  public play_land(playerId: string, cardId: string): Promise<ArenaState> {
    return this.sendCommand({ kind: "play_land", playerId, cardId });
  }

  public tap_mana_source(playerId: string, cardId: string): Promise<ArenaState> {
    return this.sendCommand({ kind: "tap_mana_source", playerId, cardId });
  }

  public cast_spell(playerId: string, cardId: string): Promise<ArenaState> {
    return this.sendCommand({ kind: "cast_spell", playerId, cardId });
  }

  public activate_ability(playerId: string, cardId: string): Promise<ArenaState> {
    return this.sendCommand({ kind: "activate_ability", playerId, cardId });
  }

  public declare_attackers(playerId: string, attackerIds: string[]): Promise<ArenaState> {
    return this.sendCommand({ kind: "declare_attackers", attackerIds, playerId });
  }

  public declare_blockers(
    playerId: string,
    assignments: BlockerAssignmentInput[],
  ): Promise<ArenaState> {
    return this.sendCommand({ assignments, kind: "declare_blockers", playerId });
  }

  public resolve_combat_damage(playerId: string): Promise<ArenaState> {
    return this.sendCommand({ kind: "resolve_combat_damage", playerId });
  }

  public discard_for_cleanup(playerId: string, cardId: string): Promise<ArenaState> {
    return this.sendCommand({ kind: "discard_for_cleanup", playerId, cardId });
  }

  public resolve_optional_effect(playerId: string, accept: boolean): Promise<ArenaState> {
    return this.sendCommand({ accept, kind: "resolve_optional_effect", playerId });
  }

  public resolve_pending_hand_choice(playerId: string, cardId: string): Promise<ArenaState> {
    return this.sendCommand({ cardId, kind: "resolve_pending_hand_choice", playerId });
  }

  public resolve_pending_scry(playerId: string, moveToBottom: boolean): Promise<ArenaState> {
    return this.sendCommand({ kind: "resolve_pending_scry", moveToBottom, playerId });
  }

  public resolve_pending_surveil(playerId: string, moveToGraveyard: boolean): Promise<ArenaState> {
    return this.sendCommand({ kind: "resolve_pending_surveil", moveToGraveyard, playerId });
  }

  private commitState(nextState: ArenaState): void {
    this.stateCache = nextState;
    this.infoState.localSeatId = seatForRole(nextState, "peer");
    this.resolveInitialState(nextState);
    this.notifyListeners(nextState);
  }

  private notifyListeners(nextState: ArenaState): void {
    for (const listener of this.listeners) {
      listener(nextState);
    }
  }

  private sendCommand(command: ArenaCommandRequest): Promise<ArenaState> {
    assertCommandMatchesBoundSeat(command, this.infoState.localSeatId);
    const requestId = createSessionId();

    return new Promise<ArenaState>((resolve, reject) => {
      const timeoutId = window.setTimeout(() => {
        this.pendingRequests.delete(requestId);
        reject(new Error("The remote host did not respond in time."));
      }, REQUEST_TIMEOUT_MS);

      this.pendingRequests.set(requestId, { reject, resolve, timeoutId });
      this.transport.send(
        JSON.stringify({
          command,
          from: this.instanceId,
          requestId,
          type: "command-request",
        } satisfies CommandRequestMessage),
      );
    });
  }
}

async function createEmbeddedSession(roomId: string, inviteUrl: string): Promise<ArenaSession> {
  const client = await createArenaClient();
  const initialState = await readState(client);

  return new HostArenaSession({
    channel: null,
    client,
    initialState,
    inviteUrl,
    instanceId: createSessionId(),
    roomId,
    transport: "embedded",
  });
}

async function discoverBroadcastRole(
  channel: BroadcastChannel,
  instanceId: string,
): Promise<BroadcastDiscoveryResult> {
  const discoveryState = {
    hostId: null as string | null,
    initialState: null as ArenaState | null,
  };
  const seenIds = new Set<string>([instanceId]);
  const handleMessage = (event: MessageEvent<unknown>): void => {
    const message = coerceSessionMessage(event.data);
    if (!message || message.from === instanceId) {
      return;
    }

    if (message.type === "hello") {
      seenIds.add(message.from);
      return;
    }

    if (message.type === "state-sync") {
      discoveryState.hostId ??= message.from;
      discoveryState.initialState ??= message.state;
    }
  };

  channel.addEventListener("message", handleMessage);
  channel.postMessage({ from: instanceId, type: "hello" } satisfies HelloMessage);

  await delay(DISCOVERY_TIMEOUT_MS);

  if (discoveryState.hostId !== null && discoveryState.initialState !== null) {
    channel.removeEventListener("message", handleMessage);
    return {
      hostId: discoveryState.hostId,
      initialState: discoveryState.initialState,
      role: "peer",
    };
  }

  if (smallestId(seenIds) === instanceId) {
    channel.removeEventListener("message", handleMessage);
    return { role: "host" };
  }

  await delay(HOST_STARTUP_GRACE_MS);

  channel.removeEventListener("message", handleMessage);

  if (discoveryState.hostId !== null && discoveryState.initialState !== null) {
    return {
      hostId: discoveryState.hostId,
      initialState: discoveryState.initialState,
      role: "peer",
    };
  }

  return { role: "host" };
}

async function runRequestedCommand(
  target: ArenaCommandTarget,
  command: ArenaCommandRequest,
): Promise<ArenaState> {
  switch (command.kind) {
    case "reset":
      return resetArena(target);
    case "pass_priority":
      return passPriority(target, command.playerId);
    case "advance_turn":
      return advanceTurn(target);
    case "concede":
      return concede(target, command.playerId);
    case "play_land":
      return playLand(target, command.playerId, command.cardId);
    case "tap_mana_source":
      return tapManaSource(target, command.playerId, command.cardId);
    case "cast_spell":
      return castSpell(target, command.playerId, command.cardId);
    case "activate_ability":
      return activateAbility(target, command.playerId, command.cardId);
    case "declare_attackers":
      return declareAttackers(target, command.playerId, command.attackerIds);
    case "declare_blockers":
      return declareBlockers(target, command.playerId, command.assignments);
    case "resolve_combat_damage":
      return resolveCombatDamage(target, command.playerId);
    case "discard_for_cleanup":
      return discardForCleanup(target, command.playerId, command.cardId);
    case "resolve_optional_effect":
      return resolveOptionalEffect(target, command.playerId, command.accept);
    case "resolve_pending_hand_choice":
      return resolvePendingHandChoice(target, command.playerId, command.cardId);
    case "resolve_pending_scry":
      return resolvePendingScry(target, command.playerId, command.moveToBottom);
    case "resolve_pending_surveil":
      return resolvePendingSurveil(target, command.playerId, command.moveToGraveyard);
  }
}

function assertCommandMatchesBoundSeat(
  command: ArenaCommandRequest,
  localSeatId: string | null,
): void {
  const commandSeatId = commandPlayerId(command);
  if (commandSeatId === null) {
    return;
  }

  if (localSeatId === null) {
    throw new Error("This browser is not bound to a seat yet.");
  }

  if (commandSeatId !== localSeatId) {
    throw new Error(`This browser is bound to ${localSeatId} and cannot act as ${commandSeatId}.`);
  }
}

function commandPlayerId(command: ArenaCommandRequest): string | null {
  switch (command.kind) {
    case "reset":
    case "advance_turn":
      return null;
    case "pass_priority":
    case "concede":
    case "play_land":
    case "tap_mana_source":
    case "cast_spell":
    case "activate_ability":
    case "declare_attackers":
    case "declare_blockers":
    case "resolve_combat_damage":
    case "discard_for_cleanup":
    case "resolve_optional_effect":
    case "resolve_pending_hand_choice":
    case "resolve_pending_scry":
    case "resolve_pending_surveil":
      return command.playerId;
  }
}

function projectRemotePeerState(state: ArenaState): ArenaState {
  const peerSeatId = seatForRole(state, "peer");
  if (peerSeatId === null) {
    return state;
  }

  return {
    ...state,
    viewers: state.viewers.map((viewer) =>
      viewer.player_id === peerSeatId
        ? viewer
        : {
            ...viewer,
            choice_requests: [],
            hand: [],
            legal_actions: [],
          },
    ),
  };
}

function seatForRole(state: ArenaState, role: ArenaSessionRole): string | null {
  if (role === "peer") {
    return state.viewers[1]?.player_id ?? state.viewers[0]?.player_id ?? null;
  }

  return state.viewers[0]?.player_id ?? null;
}

function ensureRoomIdInUrl(): string {
  const url = new URL(window.location.href);
  const existingRoomId = url.searchParams.get(ROOM_QUERY_PARAM);
  if (existingRoomId) {
    return existingRoomId;
  }

  const roomId = `duel-${createSessionId().slice(0, 8)}`;
  url.searchParams.set(ROOM_QUERY_PARAM, roomId);
  window.history.replaceState({}, "", url);
  return roomId;
}

function createSessionId(): string {
  if ("randomUUID" in window.crypto) {
    return window.crypto.randomUUID();
  }

  return `${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 10)}`;
}

function formatSessionError(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

function parseTransportPayload(payload: string): SessionMessage | null {
  try {
    return coerceSessionMessage(JSON.parse(payload) as unknown);
  } catch {
    return null;
  }
}

async function sendRemoteStateSync(
  session: ArenaSession,
  transport: RemotePairingTransport,
  instanceId: string,
): Promise<void> {
  const state = await readState(session);
  transport.send(
    JSON.stringify({
      from: instanceId,
      state: projectRemotePeerState(state),
      type: "state-sync",
    } satisfies StateSyncMessage),
  );
}

async function respondToRemoteCommand(
  session: ArenaSession,
  transport: RemotePairingTransport,
  instanceId: string,
  message: CommandRequestMessage,
): Promise<void> {
  const currentState = await readState(session);

  try {
    assertCommandMatchesBoundSeat(message.command, seatForRole(currentState, "peer"));
    const nextState = await runRequestedCommand(session, message.command);
    transport.send(
      JSON.stringify({
        from: instanceId,
        ok: true,
        requestId: message.requestId,
        state: projectRemotePeerState(nextState),
        type: "command-response",
      } satisfies CommandResponseSuccessMessage),
    );
  } catch (error) {
    transport.send(
      JSON.stringify({
        error: formatSessionError(error),
        from: instanceId,
        ok: false,
        requestId: message.requestId,
        state: projectRemotePeerState(currentState),
        type: "command-response",
      } satisfies CommandResponseErrorMessage),
    );
  }
}

function samePeerSet(left: Set<string>, right: Set<string>): boolean {
  if (left.size !== right.size) {
    return false;
  }

  for (const value of left) {
    if (!right.has(value)) {
      return false;
    }
  }

  return true;
}

function smallestId(ids: Set<string>): string {
  return [...ids].sort()[0] ?? "";
}

function delay(milliseconds: number): Promise<void> {
  return new Promise((resolve) => {
    window.setTimeout(resolve, milliseconds);
  });
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function coerceSessionMessage(value: unknown): SessionMessage | null {
  if (!isRecord(value)) {
    return null;
  }

  if (typeof value.type !== "string" || typeof value.from !== "string") {
    return null;
  }

  switch (value.type) {
    case "hello":
      return { from: value.from, type: "hello" };
    case "goodbye":
      return { from: value.from, type: "goodbye" };
    case "state-sync":
      if (!("state" in value)) {
        return null;
      }

      return {
        from: value.from,
        state: value.state as ArenaState,
        type: "state-sync",
      };
    case "command-request":
      if (!("command" in value) || typeof value.requestId !== "string") {
        return null;
      }

      return {
        command: value.command as ArenaCommandRequest,
        from: value.from,
        requestId: value.requestId,
        type: "command-request",
      };
    case "command-response":
      if (typeof value.requestId !== "string" || typeof value.ok !== "boolean") {
        return null;
      }

      if (value.ok) {
        return {
          from: value.from,
          ok: true,
          requestId: value.requestId,
          state: value.state as ArenaState,
          type: "command-response",
        };
      }

      return {
        error: typeof value.error === "string" ? value.error : "Unknown session error",
        from: value.from,
        ok: false,
        requestId: value.requestId,
        state: "state" in value ? (value.state as ArenaState | null) : null,
        type: "command-response",
      };
    default:
      return null;
  }
}
