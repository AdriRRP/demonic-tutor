import init, { WebDemoClient } from "../wasm/pkg/demonictutor_ui.js";
import type { DemoState } from "./types";

let runtimePromise: Promise<unknown> | undefined;

async function ensureRuntime(): Promise<void> {
  runtimePromise ??= init();
  await runtimePromise;
}

export async function createDemoClient(): Promise<WebDemoClient> {
  await ensureRuntime();
  return new WebDemoClient();
}

export function readState(client: WebDemoClient): DemoState {
  return client.state() as DemoState;
}

export function resetDemo(client: WebDemoClient): DemoState {
  return client.reset() as DemoState;
}

export function stepDemo(client: WebDemoClient): DemoState {
  return client.step_demo() as DemoState;
}

export function playLand(client: WebDemoClient, cardId: string): DemoState {
  return client.play_land(cardId) as DemoState;
}

export function tapManaSource(client: WebDemoClient, cardId: string): DemoState {
  return client.tap_mana_source(cardId) as DemoState;
}
