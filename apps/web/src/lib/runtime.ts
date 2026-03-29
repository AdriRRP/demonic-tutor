import init, { WebArenaClient as WasmWebArenaClient } from "../wasm/pkg/demonictutor_ui.js";
import type { ArenaState, BlockerAssignmentInput } from "./types";

let runtimePromise: Promise<unknown> | undefined;
const initWasmRuntime = init as () => Promise<void>;

export interface WebArenaClient {
  state(): unknown;
  reset(): unknown;
  pass_priority(playerId: string): unknown;
  advance_turn(): unknown;
  concede(playerId: string): unknown;
  play_land(playerId: string, cardId: string): unknown;
  tap_mana_source(playerId: string, cardId: string): unknown;
  cast_spell(playerId: string, cardId: string): unknown;
  activate_ability(playerId: string, cardId: string): unknown;
  declare_attackers(playerId: string, attackerIds: string[]): unknown;
  declare_blockers(playerId: string, assignments: BlockerAssignmentInput[]): unknown;
  resolve_combat_damage(playerId: string): unknown;
  discard_for_cleanup(playerId: string, cardId: string): unknown;
  resolve_optional_effect(playerId: string, accept: boolean): unknown;
  resolve_pending_hand_choice(playerId: string, cardId: string): unknown;
  resolve_pending_scry(playerId: string, moveToBottom: boolean): unknown;
  resolve_pending_surveil(playerId: string, moveToGraveyard: boolean): unknown;
}

type WebArenaClientConstructor = new () => WebArenaClient;

const WasmWebArenaClientConstructor = WasmWebArenaClient as unknown as WebArenaClientConstructor;

async function ensureRuntime(): Promise<void> {
  runtimePromise ??= initWasmRuntime();
  await runtimePromise;
}

function coerceArenaState(value: unknown): ArenaState {
  return value as ArenaState;
}

export async function createArenaClient(): Promise<WebArenaClient> {
  await ensureRuntime();
  return new WasmWebArenaClientConstructor();
}

export function readState(client: WebArenaClient): ArenaState {
  return coerceArenaState(client.state());
}

export function resetArena(client: WebArenaClient): ArenaState {
  return coerceArenaState(client.reset());
}

export function passPriority(client: WebArenaClient, playerId: string): ArenaState {
  return coerceArenaState(client.pass_priority(playerId));
}

export function advanceTurn(client: WebArenaClient): ArenaState {
  return coerceArenaState(client.advance_turn());
}

export function concede(client: WebArenaClient, playerId: string): ArenaState {
  return coerceArenaState(client.concede(playerId));
}

export function playLand(client: WebArenaClient, playerId: string, cardId: string): ArenaState {
  return coerceArenaState(client.play_land(playerId, cardId));
}

export function tapManaSource(
  client: WebArenaClient,
  playerId: string,
  cardId: string,
): ArenaState {
  return coerceArenaState(client.tap_mana_source(playerId, cardId));
}

export function castSpell(client: WebArenaClient, playerId: string, cardId: string): ArenaState {
  return coerceArenaState(client.cast_spell(playerId, cardId));
}

export function activateAbility(
  client: WebArenaClient,
  playerId: string,
  cardId: string,
): ArenaState {
  return coerceArenaState(client.activate_ability(playerId, cardId));
}

export function declareAttackers(
  client: WebArenaClient,
  playerId: string,
  attackerIds: string[],
): ArenaState {
  return coerceArenaState(client.declare_attackers(playerId, attackerIds));
}

export function declareBlockers(
  client: WebArenaClient,
  playerId: string,
  assignments: BlockerAssignmentInput[],
): ArenaState {
  return coerceArenaState(client.declare_blockers(playerId, assignments));
}

export function resolveCombatDamage(client: WebArenaClient, playerId: string): ArenaState {
  return coerceArenaState(client.resolve_combat_damage(playerId));
}

export function discardForCleanup(
  client: WebArenaClient,
  playerId: string,
  cardId: string,
): ArenaState {
  return coerceArenaState(client.discard_for_cleanup(playerId, cardId));
}

export function resolveOptionalEffect(
  client: WebArenaClient,
  playerId: string,
  accept: boolean,
): ArenaState {
  return coerceArenaState(client.resolve_optional_effect(playerId, accept));
}

export function resolvePendingHandChoice(
  client: WebArenaClient,
  playerId: string,
  cardId: string,
): ArenaState {
  return coerceArenaState(client.resolve_pending_hand_choice(playerId, cardId));
}

export function resolvePendingScry(
  client: WebArenaClient,
  playerId: string,
  moveToBottom: boolean,
): ArenaState {
  return coerceArenaState(client.resolve_pending_scry(playerId, moveToBottom));
}

export function resolvePendingSurveil(
  client: WebArenaClient,
  playerId: string,
  moveToGraveyard: boolean,
): ArenaState {
  return coerceArenaState(client.resolve_pending_surveil(playerId, moveToGraveyard));
}
