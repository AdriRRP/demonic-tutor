import init, { WebArenaClient as WasmWebArenaClient } from "../wasm/pkg/demonictutor_ui.js";
import type { ArenaState, BlockerAssignmentInput } from "./types";

let runtimePromise: Promise<unknown> | undefined;
const initWasmRuntime = init as () => Promise<void>;

type MaybePromise<T> = Promise<T> | T;

export interface ArenaCommandTarget {
  state(): MaybePromise<unknown>;
  reset(): MaybePromise<unknown>;
  pass_priority(playerId: string): MaybePromise<unknown>;
  advance_turn(): MaybePromise<unknown>;
  concede(playerId: string): MaybePromise<unknown>;
  play_land(playerId: string, cardId: string): MaybePromise<unknown>;
  tap_mana_source(playerId: string, cardId: string): MaybePromise<unknown>;
  cast_spell(playerId: string, cardId: string): MaybePromise<unknown>;
  activate_ability(playerId: string, cardId: string): MaybePromise<unknown>;
  declare_attackers(playerId: string, attackerIds: string[]): MaybePromise<unknown>;
  declare_blockers(playerId: string, assignments: BlockerAssignmentInput[]): MaybePromise<unknown>;
  resolve_combat_damage(playerId: string): MaybePromise<unknown>;
  discard_for_cleanup(playerId: string, cardId: string): MaybePromise<unknown>;
  resolve_optional_effect(playerId: string, accept: boolean): MaybePromise<unknown>;
  resolve_pending_hand_choice(playerId: string, cardId: string): MaybePromise<unknown>;
  resolve_pending_scry(playerId: string, moveToBottom: boolean): MaybePromise<unknown>;
  resolve_pending_surveil(playerId: string, moveToGraveyard: boolean): MaybePromise<unknown>;
}

export type WebArenaClient = ArenaCommandTarget;

type WebArenaClientConstructor = new () => ArenaCommandTarget;

const WasmWebArenaClientConstructor = WasmWebArenaClient as unknown as WebArenaClientConstructor;

async function ensureRuntime(): Promise<void> {
  runtimePromise ??= initWasmRuntime();
  await runtimePromise;
}

function coerceArenaState(value: unknown): ArenaState {
  return value as ArenaState;
}

function isPromiseLike<T>(value: MaybePromise<T>): value is Promise<T> {
  return typeof value === "object" && value !== null && "then" in value;
}

async function resolveArenaState(value: MaybePromise<unknown>): Promise<ArenaState> {
  const resolvedValue = isPromiseLike(value) ? await value : value;
  return coerceArenaState(resolvedValue);
}

export async function createArenaClient(): Promise<ArenaCommandTarget> {
  await ensureRuntime();
  return new WasmWebArenaClientConstructor();
}

export async function readState(client: ArenaCommandTarget): Promise<ArenaState> {
  return resolveArenaState(client.state());
}

export async function resetArena(client: ArenaCommandTarget): Promise<ArenaState> {
  return resolveArenaState(client.reset());
}

export async function passPriority(
  client: ArenaCommandTarget,
  playerId: string,
): Promise<ArenaState> {
  return resolveArenaState(client.pass_priority(playerId));
}

export async function advanceTurn(client: ArenaCommandTarget): Promise<ArenaState> {
  return resolveArenaState(client.advance_turn());
}

export async function concede(client: ArenaCommandTarget, playerId: string): Promise<ArenaState> {
  return resolveArenaState(client.concede(playerId));
}

export async function playLand(
  client: ArenaCommandTarget,
  playerId: string,
  cardId: string,
): Promise<ArenaState> {
  return resolveArenaState(client.play_land(playerId, cardId));
}

export async function tapManaSource(
  client: ArenaCommandTarget,
  playerId: string,
  cardId: string,
): Promise<ArenaState> {
  return resolveArenaState(client.tap_mana_source(playerId, cardId));
}

export async function castSpell(
  client: ArenaCommandTarget,
  playerId: string,
  cardId: string,
): Promise<ArenaState> {
  return resolveArenaState(client.cast_spell(playerId, cardId));
}

export async function activateAbility(
  client: ArenaCommandTarget,
  playerId: string,
  cardId: string,
): Promise<ArenaState> {
  return resolveArenaState(client.activate_ability(playerId, cardId));
}

export async function declareAttackers(
  client: ArenaCommandTarget,
  playerId: string,
  attackerIds: string[],
): Promise<ArenaState> {
  return resolveArenaState(client.declare_attackers(playerId, attackerIds));
}

export async function declareBlockers(
  client: ArenaCommandTarget,
  playerId: string,
  assignments: BlockerAssignmentInput[],
): Promise<ArenaState> {
  return resolveArenaState(client.declare_blockers(playerId, assignments));
}

export async function resolveCombatDamage(
  client: ArenaCommandTarget,
  playerId: string,
): Promise<ArenaState> {
  return resolveArenaState(client.resolve_combat_damage(playerId));
}

export async function discardForCleanup(
  client: ArenaCommandTarget,
  playerId: string,
  cardId: string,
): Promise<ArenaState> {
  return resolveArenaState(client.discard_for_cleanup(playerId, cardId));
}

export async function resolveOptionalEffect(
  client: ArenaCommandTarget,
  playerId: string,
  accept: boolean,
): Promise<ArenaState> {
  return resolveArenaState(client.resolve_optional_effect(playerId, accept));
}

export async function resolvePendingHandChoice(
  client: ArenaCommandTarget,
  playerId: string,
  cardId: string,
): Promise<ArenaState> {
  return resolveArenaState(client.resolve_pending_hand_choice(playerId, cardId));
}

export async function resolvePendingScry(
  client: ArenaCommandTarget,
  playerId: string,
  moveToBottom: boolean,
): Promise<ArenaState> {
  return resolveArenaState(client.resolve_pending_scry(playerId, moveToBottom));
}

export async function resolvePendingSurveil(
  client: ArenaCommandTarget,
  playerId: string,
  moveToGraveyard: boolean,
): Promise<ArenaState> {
  return resolveArenaState(client.resolve_pending_surveil(playerId, moveToGraveyard));
}
