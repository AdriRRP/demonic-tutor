import { For, Match, Show, Switch, createMemo, createSignal, onMount } from "solid-js";
import type { Component } from "solid-js";
import type { WebDemoClient } from "./wasm/pkg/demonictutor_ui.js";
import {
  createDemoClient,
  playLand,
  readState,
  resetDemo,
  stepDemo,
  tapManaSource,
} from "./lib/runtime";
import type {
  DemoBattlefieldCard,
  DemoCardView,
  DemoLegalAction,
  DemoPlayerView,
  DemoState,
} from "./lib/types";

const App: Component = () => {
  const [client, setClient] = createSignal<WebDemoClient | null>(null);
  const [state, setState] = createSignal<DemoState | null>(null);
  const [error, setError] = createSignal<string | null>(null);
  const [loading, setLoading] = createSignal(true);

  onMount(async () => {
    try {
      const nextClient = await createDemoClient();
      setClient(nextClient);
      setState(readState(nextClient));
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  });

  const legalActions = createMemo(() => state()?.legal_actions ?? []);
  const playLandAction = createMemo(() =>
    legalActions().find((action) => action.kind === "PlayLand"),
  );
  const tapManaAction = createMemo(() =>
    legalActions().find((action) => action.kind === "TapManaSource"),
  );
  const unsupportedActions = createMemo(() =>
    legalActions().filter(
      (action) => action.kind !== "PlayLand" && action.kind !== "TapManaSource",
    ),
  );

  const run = (operation: (current: WebDemoClient) => DemoState) => {
    const current = client();
    if (!current) return;

    try {
      setState(operation(current));
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    }
  };

  return (
    <main class="shell">
      <div class="backdrop backdrop-a" />
      <div class="backdrop backdrop-b" />
      <header class="hero">
        <div class="hero-copy">
          <p class="eyebrow">DemonicTutor Web Lab</p>
          <h1>Rust engine. Wasm bridge. First playable board shell.</h1>
          <p class="lede">
            This UI is already reading a real public session from the engine, not a
            fabricated frontend-only mock. The first slice focuses on snapshot fluency,
            timeline visibility, and a small command corridor.
          </p>
        </div>
        <div class="hero-actions">
          <button class="hero-button" onClick={() => run(resetDemo)}>
            Reset demo
          </button>
          <button class="hero-button hero-button-ghost" onClick={() => run(stepDemo)}>
            Step demo
          </button>
        </div>
      </header>

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
          </section>
        </Match>
        <Match when={state()}>
          {(resolved) => (
            <div class="dashboard">
              <section class="panel status-panel">
                <div class="status-grid">
                  <div>
                    <p class="label">Game</p>
                    <p class="value">{resolved().game.game_id}</p>
                  </div>
                  <div>
                    <p class="label">Subset</p>
                    <p class="value">{resolved().game.playable_subset_version}</p>
                  </div>
                  <div>
                    <p class="label">Turn</p>
                    <p class="value">
                      {resolved().game.turn_number} · {resolved().game.phase}
                    </p>
                  </div>
                  <div>
                    <p class="label">Priority</p>
                    <p class="value">{resolved().game.priority_holder ?? "closed"}</p>
                  </div>
                </div>
                <Show when={resolved().last_command}>
                  {(lastCommand) => (
                    <div
                      classList={{
                        "command-result": true,
                        applied: lastCommand().applied,
                        rejected: !lastCommand().applied,
                      }}
                    >
                      <p class="label">Last command</p>
                      <p>{lastCommand().message}</p>
                      <Show when={lastCommand().emitted_events.length > 0}>
                        <div class="chip-row">
                          <For each={lastCommand().emitted_events}>
                            {(event) => <span class="chip">{event}</span>}
                          </For>
                        </div>
                      </Show>
                    </div>
                  )}
                </Show>
              </section>

              <section class="board">
                <For each={resolved().game.players}>
                  {(player) => (
                    <PlayerPanel
                      player={player}
                      isViewer={player.player_id === "player-1"}
                    />
                  )}
                </For>
              </section>

              <div class="sidebar">
                <section class="panel action-panel">
                  <div class="panel-head">
                    <h2>Action rail</h2>
                    <p>Viewer-scoped public actions plus a demo stepper.</p>
                  </div>

                  <Show when={playLandAction()}>
                    {(action) => (
                      <ActionCluster
                        title="Play land"
                        subtitle={action().summary}
                        cardIds={action().card_ids}
                        onPress={(cardId) => run((current) => playLand(current, cardId))}
                      />
                    )}
                  </Show>

                  <Show when={tapManaAction()}>
                    {(action) => (
                      <ActionCluster
                        title="Tap mana source"
                        subtitle={action().summary}
                        cardIds={action().card_ids}
                        onPress={(cardId) => run((current) => tapManaSource(current, cardId))}
                      />
                    )}
                  </Show>

                  <div class="action-meta">
                    <p class="label">Other visible actions</p>
                    <Show when={unsupportedActions().length > 0} fallback={<p>None right now.</p>}>
                      <div class="unsupported-list">
                        <For each={unsupportedActions()}>
                          {(action) => (
                            <div class="unsupported-item">
                              <strong>{action.kind}</strong>
                              <span>{action.summary}</span>
                            </div>
                          )}
                        </For>
                      </div>
                    </Show>
                  </div>
                </section>

                <section class="panel">
                  <div class="panel-head">
                    <h2>Choice prompts</h2>
                    <p>Raw public prompts currently exposed by the runtime.</p>
                  </div>
                  <Show
                    when={resolved().choice_requests.length > 0}
                    fallback={<p class="muted">No visible prompts for this viewer.</p>}
                  >
                    <div class="prompt-list">
                      <For each={resolved().choice_requests}>
                        {(prompt) => (
                          <article class="prompt-item">
                            <strong>{prompt.kind}</strong>
                            <p>{prompt.summary}</p>
                            <Show when={prompt.item_ids.length > 0}>
                              <div class="chip-row">
                                <For each={prompt.item_ids}>
                                  {(itemId) => <span class="chip">{itemId}</span>}
                                </For>
                              </div>
                            </Show>
                          </article>
                        )}
                      </For>
                    </div>
                  </Show>
                </section>

                <section class="panel">
                  <div class="panel-head">
                    <h2>Stack + replay</h2>
                    <p>The event stream and stack are already coming from persisted public state.</p>
                  </div>
                  <StackView stack={resolved().game.stack} />
                  <Timeline entries={resolved().event_log} />
                </section>
              </div>
            </div>
          )}
        </Match>
      </Switch>
    </main>
  );
};

const PlayerPanel: Component<{ player: DemoPlayerView; isViewer: boolean }> = (props) => (
  <section
    classList={{
      panel: true,
      "player-panel": true,
      active: props.player.is_active,
      viewer: props.isViewer,
    }}
  >
    <div class="panel-head">
      <div>
        <p class="eyebrow">{props.isViewer ? "Viewer" : "Opponent"}</p>
        <h2>{props.player.player_id}</h2>
      </div>
      <div class="stat-strip">
        <span>Life {props.player.life}</span>
        <span>Mana {props.player.mana_total}</span>
        <span>Hand {props.player.hand_count}</span>
        <span>Library {props.player.library_count}</span>
      </div>
    </div>

    <div class="zone-grid">
      <ZonePanel title="Battlefield" emptyMessage="No permanents yet.">
        <Show
          when={props.player.battlefield.length > 0}
          fallback={<p class="muted">No permanents yet.</p>}
        >
          <div class="card-grid">
            <For each={props.player.battlefield}>
              {(card) => <BattlefieldCard card={card} />}
            </For>
          </div>
        </Show>
      </ZonePanel>

      <ZonePanel title={`Graveyard (${props.player.graveyard.length})`} emptyMessage="Empty">
        <CompactCardList cards={props.player.graveyard} />
      </ZonePanel>

      <ZonePanel title={`Exile (${props.player.exile.length})`} emptyMessage="Empty">
        <CompactCardList cards={props.player.exile} />
      </ZonePanel>
    </div>
  </section>
);

const ZonePanel: Component<{
  title: string;
  emptyMessage: string;
  children: import("solid-js").JSX.Element;
}> = (props) => (
  <article class="zone-panel">
    <div class="zone-head">
      <h3>{props.title}</h3>
    </div>
    {props.children}
  </article>
);

const BattlefieldCard: Component<{ card: DemoBattlefieldCard }> = (props) => (
  <article class="battle-card">
    <div class="battle-card-top">
      <strong>{props.card.definition_id}</strong>
      <span>{props.card.card_type}</span>
    </div>
    <div class="battle-tags">
      <span>{props.card.tapped ? "Tapped" : "Ready"}</span>
      <span>{props.card.token ? "Token" : "Card"}</span>
      <Show when={props.card.summoning_sickness}>
        <span>Sick</span>
      </Show>
    </div>
    <Show when={props.card.keywords.length > 0}>
      <div class="chip-row">
        <For each={props.card.keywords}>{(keyword) => <span class="chip">{keyword}</span>}</For>
      </div>
    </Show>
    <div class="battle-stats">
      <Show when={props.card.power !== null && props.card.toughness !== null}>
        <span>
          {props.card.power}/{props.card.toughness}
        </span>
      </Show>
      <Show when={props.card.loyalty !== null}>
        <span>Loyalty {props.card.loyalty}</span>
      </Show>
    </div>
  </article>
);

const CompactCardList: Component<{ cards: DemoCardView[] }> = (props) => (
  <Show when={props.cards.length > 0} fallback={<p class="muted">Empty</p>}>
    <ul class="compact-list">
      <For each={props.cards}>
        {(card) => (
          <li>
            <strong>{card.definition_id}</strong>
            <span>{card.card_type}</span>
          </li>
        )}
      </For>
    </ul>
  </Show>
);

const ActionCluster: Component<{
  title: string;
  subtitle: string;
  cardIds: string[];
  onPress: (cardId: string) => void;
}> = (props) => (
  <div class="action-cluster">
    <div class="panel-head">
      <h3>{props.title}</h3>
      <p>{props.subtitle}</p>
    </div>
    <div class="action-list">
      <For each={props.cardIds}>
        {(cardId) => (
          <button class="action-button" onClick={() => props.onPress(cardId)}>
            {cardId}
          </button>
        )}
      </For>
    </div>
  </div>
);

const StackView: Component<{ stack: DemoState["game"]["stack"] }> = (props) => (
  <div class="stack-list">
    <Show when={props.stack.length > 0} fallback={<p class="muted">The stack is empty.</p>}>
      <For each={props.stack}>
        {(object) => (
          <article class="stack-item">
            <strong>
              #{object.number} · {object.kind}
            </strong>
            <p>{object.source_card_id ?? "unknown source"}</p>
            <Show when={object.target}>
              <span class="muted">Target: {object.target}</span>
            </Show>
          </article>
        )}
      </For>
    </Show>
  </div>
);

const Timeline: Component<{ entries: DemoState["event_log"] }> = (props) => (
  <div class="timeline">
    <For each={props.entries.slice().reverse().slice(0, 12)}>
      {(entry) => (
        <article class="timeline-entry">
          <span class="timeline-seq">#{entry.sequence}</span>
          <p>{entry.label}</p>
        </article>
      )}
    </For>
  </div>
);

export default App;
