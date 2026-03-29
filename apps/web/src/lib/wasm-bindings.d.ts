import type { DemoState } from "./types";

declare module "../wasm/pkg/demonictutor_ui.js" {
  export default function init(): Promise<void>;

  export class WebDemoClient {
    constructor();

    state(): DemoState;
    reset(): DemoState;
    step_demo(): DemoState;
    play_land(cardId: string): DemoState;
    tap_mana_source(cardId: string): DemoState;
  }
}
