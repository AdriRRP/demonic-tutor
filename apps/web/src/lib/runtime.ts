import init, { WebDemoClient as WasmWebDemoClient } from "../wasm/pkg/demonictutor_ui.js";
import type { DemoState } from "./types";

let runtimePromise: Promise<unknown> | undefined;
const initWasmRuntime = init as () => Promise<void>;

export interface WebDemoClient {
  state(): unknown;
  reset(): unknown;
  step_demo(): unknown;
  play_land(cardId: string): unknown;
  tap_mana_source(cardId: string): unknown;
}

type WebDemoClientConstructor = new () => WebDemoClient;

const WasmWebDemoClientConstructor = WasmWebDemoClient as unknown as WebDemoClientConstructor;

async function ensureRuntime(): Promise<void> {
  runtimePromise ??= initWasmRuntime();
  await runtimePromise;
}

function coerceDemoState(value: unknown): DemoState {
  return value as DemoState;
}

export async function createDemoClient(): Promise<WebDemoClient> {
  await ensureRuntime();
  return new WasmWebDemoClientConstructor();
}

export function readState(client: WebDemoClient): DemoState {
  return coerceDemoState(client.state());
}

export function resetDemo(client: WebDemoClient): DemoState {
  return coerceDemoState(client.reset());
}

export function stepDemo(client: WebDemoClient): DemoState {
  return coerceDemoState(client.step_demo());
}

export function playLand(client: WebDemoClient, cardId: string): DemoState {
  return coerceDemoState(client.play_land(cardId));
}

export function tapManaSource(client: WebDemoClient, cardId: string): DemoState {
  return coerceDemoState(client.tap_mana_source(cardId));
}
