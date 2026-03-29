import { For, Match, Show, Switch, createSignal, onMount } from "solid-js";
import type { Component } from "solid-js";
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
  type WebArenaClient,
} from "./lib/runtime";
import type {
  ArenaBattlefieldCard,
  ArenaCardView,
  ArenaChoicePrompt,
  ArenaGameView,
  ArenaHandCard,
  ArenaLegalAction,
  ArenaPlayerView,
  ArenaStackObject,
  ArenaState,
  ArenaTimelineEntry,
  ArenaViewerState,
  BlockerAssignmentInput,
} from "./lib/types";

const App: Component = () => {
  const [client, setClient] = createSignal<WebArenaClient | null>(null);
  const [state, setState] = createSignal<ArenaState | null>(null);
  const [error, setError] = createSignal<string | null>(null);
  const [loading, setLoading] = createSignal(true);
  const [revealedHands, setRevealedHands] = createSignal<Record<string, boolean>>({});
  const [selectedAttackers, setSelectedAttackers] = createSignal<string[]>([]);
  const [blockerAssignments, setBlockerAssignments] = createSignal<Record<string, string>>({});

  onMount(() => {
    void loadArena();
  });

  async function loadArena(): Promise<void> {
    try {
      const nextClient = await createArenaClient();
      const nextState = readState(nextClient);
      setClient(nextClient);
      setState(nextState);
      setRevealedHands(revealPattern(nextState));
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }

  const run = (operation: (current: WebArenaClient) => ArenaState) => {
    const current = client();
    if (!current) {
      return;
    }

    try {
      const nextState = operation(current);
      setState(nextState);
      setRevealedHands(revealPattern(nextState));
      setSelectedAttackers([]);
      setBlockerAssignments({});
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    }
  };

  const toggleHandReveal = (playerId: string) => {
    setRevealedHands((current) => ({
      ...current,
      [playerId]: !current[playerId],
    }));
  };

  const toggleAttackerSelection = (cardId: string) => {
    setSelectedAttackers((current) =>
      current.includes(cardId) ? current.filter((entry) => entry !== cardId) : [...current, cardId],
    );
  };

  const setBlockerAssignment = (blockerId: string, attackerId: string) => {
    setBlockerAssignments((current) => {
      if (attackerId.length === 0) {
        return Object.fromEntries(
          Object.entries(current).filter(([entryBlockerId]) => entryBlockerId !== blockerId),
        );
      }

      return {
        ...current,
        [blockerId]: attackerId,
      };
    });
  };

  return (
    <main class="shell">
      <div class="backdrop backdrop-a" />
      <div class="backdrop backdrop-b" />
      <header class="hero">
        <div class="hero-copy">
          <p class="eyebrow">DemonicTutor Duel Arena</p>
          <h1>Two hot-seat views, one Rust game, zero frontend rules.</h1>
          <p class="lede">
            This arena runs a real shared game and renders two viewer-scoped seats over the same
            deterministic engine state, so we can generate meaningful logs while pressure-testing
            the command corridor.
          </p>
        </div>
        <div class="hero-actions">
          <button
            class="hero-button"
            onClick={() => {
              run(resetArena);
            }}
          >
            Reset duel
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
            <div class="arena-dashboard">
              <section class="panel arena-status">
                <div class="panel-head">
                  <div>
                    <h2>Duel state</h2>
                    <p>Shared public board plus viewer-scoped seats.</p>
                  </div>
                  <div class="chip-row">
                    <span class="chip">Game {resolved().game.game_id}</span>
                    <span class="chip">Subset {resolved().game.playable_subset_version}</span>
                    <span class="chip">
                      Turn {resolved().game.turn_number} · {resolved().game.phase}
                    </span>
                    <span class="chip">Priority {resolved().game.priority_holder ?? "closed"}</span>
                  </div>
                </div>

                <div class="status-grid">
                  <div>
                    <p class="label">Active player</p>
                    <p class="value">{resolved().game.active_player_id ?? "n/a"}</p>
                  </div>
                  <div>
                    <p class="label">Priority pending pass</p>
                    <p class="value">
                      {resolved().game.priority_has_pending_pass === null
                        ? "n/a"
                        : resolved().game.priority_has_pending_pass
                          ? "yes"
                          : "no"}
                    </p>
                  </div>
                  <div>
                    <p class="label">Game over</p>
                    <p class="value">{resolved().game.is_over ? "yes" : "no"}</p>
                  </div>
                  <div>
                    <p class="label">Winner</p>
                    <p class="value">{resolved().game.winner_id ?? "pending"}</p>
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
                  {(player) => <PlayerBoardPanel player={player} />}
                </For>
              </section>

              <section class="panel shared-stack-panel">
                <div class="panel-head">
                  <div>
                    <h2>Stack + replay</h2>
                    <p>Persisted public log plus the current shared stack.</p>
                  </div>
                </div>
                <StackView stack={resolved().game.stack} />
                <Timeline entries={resolved().event_log} />
              </section>

              <section class="viewer-grid">
                <For each={resolved().viewers}>
                  {(viewer) => (
                    <ViewerSeat
                      blockerAssignments={blockerAssignments()}
                      game={resolved().game}
                      onRun={run}
                      onSetBlockerAssignment={setBlockerAssignment}
                      onToggleAttackerSelection={toggleAttackerSelection}
                      onToggleHandReveal={() => {
                        toggleHandReveal(viewer.player_id);
                      }}
                      revealed={Boolean(revealedHands()[viewer.player_id])}
                      selectedAttackers={selectedAttackers()}
                      viewer={viewer}
                    />
                  )}
                </For>
              </section>
            </div>
          )}
        </Match>
      </Switch>
    </main>
  );
};

interface ViewerSeatProps {
  viewer: ArenaViewerState;
  game: ArenaGameView;
  revealed: boolean;
  selectedAttackers: string[];
  blockerAssignments: Record<string, string>;
  onToggleHandReveal: () => void;
  onToggleAttackerSelection: (cardId: string) => void;
  onSetBlockerAssignment: (blockerId: string, attackerId: string) => void;
  onRun: (operation: (current: WebArenaClient) => ArenaState) => void;
}

const ViewerSeat: Component<ViewerSeatProps> = (props) => {
  const viewerPlayer = () => findPlayer(props.game, props.viewer.player_id);
  const playLandAction = () => findAction(props.viewer, "PlayLand");
  const castSpellAction = () => findAction(props.viewer, "CastSpell");
  const tapManaSourceAction = () => findAction(props.viewer, "TapManaSource");
  const activateAbilityAction = () => findAction(props.viewer, "ActivateAbility");
  const declareAttackersAction = () => findAction(props.viewer, "DeclareAttackers");
  const declareBlockersAction = () => findAction(props.viewer, "DeclareBlockers");
  const resolveCombatDamageAction = () => findAction(props.viewer, "ResolveCombatDamage");
  const advanceTurnAction = () => findAction(props.viewer, "AdvanceTurn");
  const discardForCleanupAction = () => findAction(props.viewer, "DiscardForCleanup");
  const passPriorityAction = () => findAction(props.viewer, "PassPriority");
  const concedeAction = () => findAction(props.viewer, "Concede");
  const playLandIds = () => new Set(playLandAction()?.card_ids ?? []);
  const castSpellIds = () => new Set(castSpellAction()?.card_ids ?? []);
  const tapManaSourceIds = () => new Set(tapManaSourceAction()?.card_ids ?? []);
  const activateAbilityIds = () => new Set(activateAbilityAction()?.card_ids ?? []);
  const discardForCleanupIds = () => new Set(discardForCleanupAction()?.card_ids ?? []);
  const supportedPrompts = () =>
    props.viewer.choice_requests.filter((prompt) =>
      ["PendingScry", "PendingSurveil", "PendingHandChoice", "OptionalEffectDecision"].includes(
        prompt.kind,
      ),
    );
  const unsupportedPrompts = () =>
    props.viewer.choice_requests.filter(
      (prompt) =>
        !["PendingScry", "PendingSurveil", "PendingHandChoice", "OptionalEffectDecision"].includes(
          prompt.kind,
        ),
    );

  return (
    <section
      classList={{
        panel: true,
        "viewer-seat": true,
        active: props.viewer.is_active,
        priority: props.viewer.is_priority_holder,
      }}
    >
      <div class="panel-head">
        <div>
          <h2>{props.viewer.player_id}</h2>
          <p>
            {props.viewer.is_active ? "Active seat" : "Waiting seat"} ·{" "}
            {props.viewer.is_priority_holder ? "holding priority" : "not holding priority"}
          </p>
        </div>
        <div class="chip-row">
          <Show when={props.viewer.is_active}>
            <span class="chip chip-ember">Active</span>
          </Show>
          <Show when={props.viewer.is_priority_holder}>
            <span class="chip chip-night">Priority</span>
          </Show>
        </div>
      </div>

      <div class="seat-toolbar">
        <Show when={passPriorityAction()}>
          <button
            class="hero-button seat-button"
            onClick={() => {
              const playerId = props.viewer.player_id;
              props.onRun((current) => passPriority(current, playerId));
            }}
          >
            Pass priority
          </button>
        </Show>
        <Show when={advanceTurnAction()}>
          <button
            class="hero-button seat-button"
            onClick={() => {
              props.onRun(advanceTurn);
            }}
          >
            Advance turn
          </button>
        </Show>
        <Show when={resolveCombatDamageAction()}>
          <button
            class="hero-button seat-button"
            onClick={() => {
              const playerId = props.viewer.player_id;
              props.onRun((current) => resolveCombatDamage(current, playerId));
            }}
          >
            Resolve combat damage
          </button>
        </Show>
        <Show when={concedeAction()}>
          <button
            class="hero-button hero-button-ghost seat-button"
            onClick={() => {
              const playerId = props.viewer.player_id;
              props.onRun((current) => concede(current, playerId));
            }}
          >
            Concede
          </button>
        </Show>
      </div>

      <section class="seat-section">
        <div class="panel-head">
          <div>
            <h3>Private hand</h3>
            <p>Hot-seat reveal keeps only the active player open by default.</p>
          </div>
          <button
            class="hero-button hero-button-ghost mini-button"
            onClick={() => {
              props.onToggleHandReveal();
            }}
          >
            {props.revealed ? "Hide hand" : "Reveal hand"}
          </button>
        </div>

        <Show when={props.revealed} fallback={<p class="muted">Hand hidden for seat passing.</p>}>
          <div class="hand-grid">
            <For each={props.viewer.hand}>
              {(card) => (
                <article class="hand-card">
                  <div class="battle-card-top">
                    <strong>{card.definition_id}</strong>
                    <span>{card.card_type}</span>
                  </div>
                  <div class="chip-row">
                    <span class="chip">Cost {card.mana_cost}</span>
                    <Show when={card.power !== null && card.toughness !== null}>
                      <span class="chip">
                        {card.power}/{card.toughness}
                      </span>
                    </Show>
                    <Show when={card.loyalty !== null}>
                      <span class="chip">Loyalty {card.loyalty}</span>
                    </Show>
                    <For each={card.keywords}>
                      {(keyword) => <span class="chip">{keyword}</span>}
                    </For>
                    <Show when={card.can_cast_in_open_priority}>
                      <span class="chip chip-night">Open priority</span>
                    </Show>
                    <Show when={card.can_cast_in_open_priority_during_own_turn}>
                      <span class="chip chip-night">Own-turn priority</span>
                    </Show>
                    <Show when={card.has_activated_ability}>
                      <span class="chip chip-forest">Ability</span>
                    </Show>
                  </div>
                  <div class="card-actions">
                    <Show when={playLandIds().has(card.card_id)}>
                      <button
                        class="action-button"
                        onClick={() => {
                          const playerId = props.viewer.player_id;
                          const cardId = card.card_id;
                          props.onRun((current) => playLand(current, playerId, cardId));
                        }}
                      >
                        Play land
                      </button>
                    </Show>
                    <Show when={castSpellIds().has(card.card_id)}>
                      <button
                        class="action-button"
                        disabled={card.requires_target || card.requires_choice}
                        onClick={() => {
                          const playerId = props.viewer.player_id;
                          const cardId = card.card_id;
                          props.onRun((current) => castSpell(current, playerId, cardId));
                        }}
                      >
                        {card.requires_target || card.requires_choice
                          ? "Cast UI pending"
                          : "Cast spell"}
                      </button>
                    </Show>
                    <Show when={discardForCleanupIds().has(card.card_id)}>
                      <button
                        class="action-button"
                        onClick={() => {
                          const playerId = props.viewer.player_id;
                          const cardId = card.card_id;
                          props.onRun((current) => discardForCleanup(current, playerId, cardId));
                        }}
                      >
                        Discard
                      </button>
                    </Show>
                  </div>
                </article>
              )}
            </For>
          </div>
        </Show>
      </section>

      <section class="seat-section">
        <div class="panel-head">
          <div>
            <h3>Battlefield controls</h3>
            <p>Actions are still validated entirely by the engine.</p>
          </div>
        </div>

        <Show when={tapManaSourceAction()}>
          <ActionList
            title="Tap mana"
            items={tapManaSourceAction()?.card_ids ?? []}
            labelFor={(cardId) => battlefieldCardLabel(props.game, cardId)}
            onPress={(cardId) => {
              const playerId = props.viewer.player_id;
              props.onRun((current) => tapManaSource(current, playerId, cardId));
            }}
          />
        </Show>

        <Show when={activateAbilityAction()}>
          <ActionList
            title="Activate ability"
            items={activateAbilityAction()?.card_ids ?? []}
            labelFor={(cardId) => battlefieldCardLabel(props.game, cardId)}
            onPress={(cardId) => {
              const playerId = props.viewer.player_id;
              props.onRun((current) => activateAbility(current, playerId, cardId));
            }}
          />
        </Show>

        <Show when={declareAttackersAction()}>
          <article class="combat-planner">
            <div class="panel-head">
              <div>
                <h3>Declare attackers</h3>
                <p>Select any subset, including zero.</p>
              </div>
            </div>
            <div class="chip-row">
              <For each={declareAttackersAction()?.card_ids ?? []}>
                {(cardId) => (
                  <button
                    classList={{
                      chip: true,
                      "chip-toggle": true,
                      selected: props.selectedAttackers.includes(cardId),
                    }}
                    onClick={() => {
                      props.onToggleAttackerSelection(cardId);
                    }}
                  >
                    {battlefieldCardLabel(props.game, cardId)}
                  </button>
                )}
              </For>
            </div>
            <button
              class="hero-button seat-button"
              onClick={() => {
                const playerId = props.viewer.player_id;
                const attackerIds = [...props.selectedAttackers];
                props.onRun((current) => declareAttackers(current, playerId, attackerIds));
              }}
            >
              Submit attackers
            </button>
          </article>
        </Show>

        <Show when={declareBlockersAction()}>
          <article class="combat-planner">
            <div class="panel-head">
              <div>
                <h3>Declare blockers</h3>
                <p>Assign each blocker to one attacker or leave it unused.</p>
              </div>
            </div>
            <div class="blocker-grid">
              <For each={declareBlockersAction()?.blocker_options ?? []}>
                {(option) => (
                  <label class="blocker-select">
                    <span>{battlefieldCardLabel(props.game, option.blocker_id)}</span>
                    <select
                      onInput={(event) => {
                        props.onSetBlockerAssignment(option.blocker_id, event.currentTarget.value);
                      }}
                      value={props.blockerAssignments[option.blocker_id] ?? ""}
                    >
                      <option value="">No block</option>
                      <For each={option.attacker_ids}>
                        {(attackerId) => (
                          <option value={attackerId}>
                            {battlefieldCardLabel(props.game, attackerId)}
                          </option>
                        )}
                      </For>
                    </select>
                  </label>
                )}
              </For>
            </div>
            <button
              class="hero-button seat-button"
              onClick={() => {
                const playerId = props.viewer.player_id;
                const assignments = blockerAssignmentsToArray(props.blockerAssignments);
                props.onRun((current) => declareBlockers(current, playerId, assignments));
              }}
            >
              Submit blockers
            </button>
          </article>
        </Show>

        <Show
          when={
            !tapManaSourceIds().size &&
            !activateAbilityIds().size &&
            !declareAttackersAction() &&
            !declareBlockersAction()
          }
        >
          <p class="muted">No battlefield-specific controls for this seat right now.</p>
        </Show>
      </section>

      <section class="seat-section">
        <div class="panel-head">
          <div>
            <h3>Choice prompts</h3>
            <p>Viewer-scoped prompts coming directly from the public surface.</p>
          </div>
        </div>

        <Show
          when={supportedPrompts().length > 0}
          fallback={<p class="muted">No active prompt.</p>}
        >
          <div class="prompt-list">
            <For each={supportedPrompts()}>
              {(prompt) => (
                <SupportedPrompt
                  hand={props.viewer.hand}
                  onRun={props.onRun}
                  playerId={props.viewer.player_id}
                  prompt={prompt}
                />
              )}
            </For>
          </div>
        </Show>

        <Show when={unsupportedPrompts().length > 0}>
          <div class="unsupported-list">
            <For each={unsupportedPrompts()}>
              {(prompt) => (
                <article class="unsupported-item">
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

      <Show when={viewerPlayer()}>
        {(player) => (
          <section class="seat-section">
            <div class="panel-head">
              <div>
                <h3>Seat snapshot</h3>
                <p>
                  Life {player().life} · mana {player().mana_total} · hand {player().hand_count}
                </p>
              </div>
            </div>
            <div class="compact-zones">
              <CompactZone
                cards={player().graveyard}
                emptyLabel="Graveyard clear"
                title={`Graveyard (${String(player().graveyard.length)})`}
              />
              <CompactZone
                cards={player().exile}
                emptyLabel="Nothing in exile"
                title={`Exile (${String(player().exile.length)})`}
              />
            </div>
          </section>
        )}
      </Show>
    </section>
  );
};

const SupportedPrompt: Component<{
  prompt: ArenaChoicePrompt;
  playerId: string;
  hand: ArenaHandCard[];
  onRun: (operation: (current: WebArenaClient) => ArenaState) => void;
}> = (props) => (
  <article class="prompt-item">
    <strong>{props.prompt.kind}</strong>
    <p>{props.prompt.summary}</p>

    <Switch>
      <Match when={props.prompt.kind === "PendingScry"}>
        <div class="chip-row">
          <button
            class="action-button"
            onClick={() => {
              const playerId = props.playerId;
              props.onRun((current) => resolvePendingScry(current, playerId, false));
            }}
          >
            Keep on top
          </button>
          <button
            class="action-button"
            onClick={() => {
              const playerId = props.playerId;
              props.onRun((current) => resolvePendingScry(current, playerId, true));
            }}
          >
            Move to bottom
          </button>
        </div>
      </Match>
      <Match when={props.prompt.kind === "PendingSurveil"}>
        <div class="chip-row">
          <button
            class="action-button"
            onClick={() => {
              const playerId = props.playerId;
              props.onRun((current) => resolvePendingSurveil(current, playerId, false));
            }}
          >
            Keep on top
          </button>
          <button
            class="action-button"
            onClick={() => {
              const playerId = props.playerId;
              props.onRun((current) => resolvePendingSurveil(current, playerId, true));
            }}
          >
            Move to graveyard
          </button>
        </div>
      </Match>
      <Match when={props.prompt.kind === "OptionalEffectDecision"}>
        <div class="chip-row">
          <button
            class="action-button"
            onClick={() => {
              const playerId = props.playerId;
              props.onRun((current) => resolveOptionalEffect(current, playerId, true));
            }}
          >
            Yes
          </button>
          <button
            class="action-button"
            onClick={() => {
              const playerId = props.playerId;
              props.onRun((current) => resolveOptionalEffect(current, playerId, false));
            }}
          >
            No
          </button>
        </div>
      </Match>
      <Match when={props.prompt.kind === "PendingHandChoice"}>
        <div class="chip-row">
          <For each={props.prompt.item_ids}>
            {(itemId) => (
              <button
                class="action-button"
                onClick={() => {
                  const playerId = props.playerId;
                  props.onRun((current) => resolvePendingHandChoice(current, playerId, itemId));
                }}
              >
                {handCardLabel(props.hand, itemId)}
              </button>
            )}
          </For>
        </div>
      </Match>
    </Switch>

    <Show when={props.prompt.item_ids.length > 0}>
      <div class="chip-row">
        <For each={props.prompt.item_ids}>{(itemId) => <span class="chip">{itemId}</span>}</For>
      </div>
    </Show>
  </article>
);

const ActionList: Component<{
  title: string;
  items: string[];
  labelFor: (itemId: string) => string;
  onPress: (itemId: string) => void;
}> = (props) => (
  <article class="action-cluster">
    <p class="label">{props.title}</p>
    <div class="action-list">
      <For each={props.items}>
        {(itemId) => (
          <button
            class="action-button"
            onClick={() => {
              props.onPress(itemId);
            }}
          >
            {props.labelFor(itemId)}
          </button>
        )}
      </For>
    </div>
  </article>
);

const PlayerBoardPanel: Component<{ player: ArenaPlayerView }> = (props) => (
  <section
    classList={{
      panel: true,
      "player-panel": true,
      active: props.player.is_active,
    }}
  >
    <div class="panel-head">
      <div>
        <h2>{props.player.player_id}</h2>
        <p>
          Life {props.player.life} · mana {props.player.mana_total} · hand {props.player.hand_count}
        </p>
      </div>
      <div class="chip-row">
        <span class="chip">Library {props.player.library_count}</span>
        <Show when={props.player.is_active}>
          <span class="chip chip-ember">Active</span>
        </Show>
      </div>
    </div>

    <div class="zone-grid">
      <div class="zone-panel">
        <div class="zone-head">
          <h3>Battlefield</h3>
        </div>
        <Show
          when={props.player.battlefield.length > 0}
          fallback={<p class="muted">Nothing on the battlefield yet.</p>}
        >
          <div class="card-grid">
            <For each={props.player.battlefield}>{(card) => <BattlefieldCard card={card} />}</For>
          </div>
        </Show>
      </div>

      <div class="zone-panel">
        <div class="zone-head">
          <h3>Graveyard</h3>
        </div>
        <CompactZone
          cards={props.player.graveyard}
          emptyLabel="No graveyard cards"
          title={`${String(props.player.graveyard.length)} cards`}
        />
      </div>

      <div class="zone-panel">
        <div class="zone-head">
          <h3>Exile</h3>
        </div>
        <CompactZone
          cards={props.player.exile}
          emptyLabel="No exile cards"
          title={`${String(props.player.exile.length)} cards`}
        />
      </div>
    </div>
  </section>
);

const BattlefieldCard: Component<{ card: ArenaBattlefieldCard }> = (props) => (
  <article class="battle-card">
    <div class="battle-card-top">
      <strong>{props.card.definition_id}</strong>
      <span>{props.card.card_type}</span>
    </div>
    <div class="chip-row">
      <Show when={props.card.power !== null && props.card.toughness !== null}>
        <span class="chip">
          {props.card.power}/{props.card.toughness}
        </span>
      </Show>
      <Show when={props.card.loyalty !== null}>
        <span class="chip">Loyalty {props.card.loyalty}</span>
      </Show>
      <Show when={props.card.tapped}>
        <span class="chip chip-night">Tapped</span>
      </Show>
      <Show when={props.card.summoning_sickness}>
        <span class="chip">Summoning sick</span>
      </Show>
      <Show when={props.card.attacking}>
        <span class="chip chip-ember">Attacking</span>
      </Show>
      <Show when={props.card.blocking}>
        <span class="chip chip-forest">Blocking</span>
      </Show>
      <Show when={props.card.token}>
        <span class="chip">Token</span>
      </Show>
      <For each={props.card.keywords}>{(keyword) => <span class="chip">{keyword}</span>}</For>
    </div>
  </article>
);

const CompactZone: Component<{
  title: string;
  cards: ArenaCardView[];
  emptyLabel: string;
}> = (props) => (
  <>
    <p class="label">{props.title}</p>
    <Show when={props.cards.length > 0} fallback={<p class="muted">{props.emptyLabel}</p>}>
      <ul class="compact-list">
        <For each={props.cards}>
          {(card) => (
            <li>
              <span>{card.definition_id}</span>
              <span>{card.card_type}</span>
            </li>
          )}
        </For>
      </ul>
    </Show>
  </>
);

const StackView: Component<{ stack: ArenaStackObject[] }> = (props) => (
  <div class="stack-list">
    <Show when={props.stack.length > 0} fallback={<p class="muted">Stack empty.</p>}>
      <For each={props.stack}>
        {(object) => (
          <article class="stack-item">
            <strong>
              #{object.number} · {object.kind}
            </strong>
            <p>
              {object.source_card_id ?? "unknown source"} ·{" "}
              {object.controller_id ?? "no controller"}
            </p>
            <Show when={object.target}>{(target) => <p class="muted">Target: {target()}</p>}</Show>
          </article>
        )}
      </For>
    </Show>
  </div>
);

const Timeline: Component<{ entries: ArenaTimelineEntry[] }> = (props) => (
  <div class="timeline">
    <Show when={props.entries.length > 0} fallback={<p class="muted">No events yet.</p>}>
      <For each={props.entries}>
        {(entry) => (
          <article class="timeline-entry">
            <span class="timeline-seq">#{entry.sequence}</span>
            <span>{entry.label}</span>
          </article>
        )}
      </For>
    </Show>
  </div>
);

function revealPattern(state: ArenaState): Record<string, boolean> {
  if (state.game.is_over) {
    return Object.fromEntries(state.viewers.map((viewer) => [viewer.player_id, true]));
  }

  return Object.fromEntries(
    state.viewers.map((viewer) => [
      viewer.player_id,
      viewer.player_id === state.game.active_player_id,
    ]),
  );
}

function findAction(viewer: ArenaViewerState, kind: string): ArenaLegalAction | undefined {
  return viewer.legal_actions.find((action) => action.kind === kind);
}

function findPlayer(game: ArenaGameView, playerId: string): ArenaPlayerView | undefined {
  return game.players.find((player) => player.player_id === playerId);
}

function findBattlefieldCard(
  game: ArenaGameView,
  cardId: string,
): ArenaBattlefieldCard | undefined {
  return game.players
    .flatMap((player) => player.battlefield)
    .find((card) => card.card_id === cardId);
}

function battlefieldCardLabel(game: ArenaGameView, cardId: string): string {
  const card = findBattlefieldCard(game, cardId);
  if (!card) {
    return cardId;
  }

  return formatCardLabel(card.definition_id, card.card_type, card.power, card.toughness);
}

function handCardLabel(hand: ArenaHandCard[], cardId: string): string {
  const card = hand.find((entry) => entry.card_id === cardId);
  if (!card) {
    return cardId;
  }

  return formatCardLabel(card.definition_id, card.card_type, card.power, card.toughness);
}

function formatCardLabel(
  definitionId: string,
  cardType: string,
  power: number | null,
  toughness: number | null,
): string {
  if (power !== null && toughness !== null) {
    return `${definitionId} · ${cardType} · ${String(power)}/${String(toughness)}`;
  }

  return `${definitionId} · ${cardType}`;
}

function blockerAssignmentsToArray(assignments: Record<string, string>): BlockerAssignmentInput[] {
  return Object.entries(assignments).map(([blocker_id, attacker_id]) => ({
    blocker_id,
    attacker_id,
  }));
}

export default App;
