import { For, Match, Show, Switch } from "solid-js";
import type { Component } from "solid-js";
import { GameCard } from "./cards/game-card";
import {
  activateAbility,
  advanceTurn,
  castSpell,
  concede,
  declareAttackers,
  declareBlockers,
  discardForCleanup,
  passPriority,
  playLand,
  resetArena,
  resolveCombatDamage,
  resolveOptionalEffect,
  resolvePendingHandChoice,
  resolvePendingScry,
  resolvePendingSurveil,
  tapManaSource,
  type WebArenaClient,
} from "../lib/runtime";
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
} from "../lib/types";

interface TableArenaProps {
  state: ArenaState;
  revealedHands: Record<string, boolean>;
  selectedAttackers: string[];
  blockerAssignments: Record<string, string>;
  onToggleHandReveal: (playerId: string) => void;
  onToggleAttackerSelection: (cardId: string) => void;
  onSetBlockerAssignment: (blockerId: string, attackerId: string) => void;
  onRun: (operation: (current: WebArenaClient) => ArenaState) => void;
}

export const TableArena: Component<TableArenaProps> = (props) => {
  const bottomViewer = () => props.state.viewers[0];
  const topViewer = () => props.state.viewers[1] ?? props.state.viewers[0];

  return (
    <div class="table-shell">
      <header class="table-header">
        <div class="table-brand">
          <p class="eyebrow">DemonicTutor Duel Arena</p>
          <h1>Hot-seat tabletop on a real Rust engine.</h1>
          <p class="table-copy">
            One shared deterministic game, two viewer-scoped seats, and a board-first layout made to
            pressure-test real play instead of mock flows.
          </p>
        </div>
        <div class="table-header-actions">
          <button
            class="hero-button"
            onClick={() => {
              props.onRun(resetArena);
            }}
          >
            Reset duel
          </button>
        </div>
      </header>

      <div class="table-layout">
        <section class="duel-table panel">
          <Show when={topViewer()}>
            {(viewer) => (
              <SeatPanel
                blockerAssignments={props.blockerAssignments}
                game={props.state.game}
                onRun={props.onRun}
                onSetBlockerAssignment={props.onSetBlockerAssignment}
                onToggleAttackerSelection={props.onToggleAttackerSelection}
                onToggleHandReveal={props.onToggleHandReveal}
                orientation="top"
                revealed={Boolean(props.revealedHands[viewer().player_id])}
                selectedAttackers={props.selectedAttackers}
                viewer={viewer()}
              />
            )}
          </Show>

          <section class="table-centerline">
            <div class="table-phase-pill">
              <span class="table-phase-turn">Turn {props.state.game.turn_number}</span>
              <strong>{formatPhase(props.state.game.phase)}</strong>
            </div>

            <div class="table-meta-strip">
              <StatusBadge
                label="Active"
                value={props.state.game.active_player_id ?? "n/a"}
                tone="ember"
              />
              <StatusBadge
                label="Priority"
                value={props.state.game.priority_holder ?? "closed"}
                tone="night"
              />
              <StatusBadge
                label="Pending pass"
                value={
                  props.state.game.priority_has_pending_pass === null
                    ? "n/a"
                    : props.state.game.priority_has_pending_pass
                      ? "yes"
                      : "no"
                }
                tone="forest"
              />
              <Show when={props.state.game.is_over}>
                <StatusBadge
                  label="Winner"
                  value={props.state.game.winner_id ?? "draw"}
                  tone="ember"
                />
              </Show>
            </div>

            <div class="table-stack-well">
              <div class="table-well-head">
                <div>
                  <p class="label">Stack</p>
                  <h2>Center lane</h2>
                </div>
                <span class="chip chip-night">
                  {props.state.game.stack.length > 0
                    ? `${String(props.state.game.stack.length)} objects`
                    : "empty"}
                </span>
              </div>
              <StackView stack={props.state.game.stack} />
            </div>
          </section>

          <Show when={bottomViewer()}>
            {(viewer) => (
              <SeatPanel
                blockerAssignments={props.blockerAssignments}
                game={props.state.game}
                onRun={props.onRun}
                onSetBlockerAssignment={props.onSetBlockerAssignment}
                onToggleAttackerSelection={props.onToggleAttackerSelection}
                onToggleHandReveal={props.onToggleHandReveal}
                orientation="bottom"
                revealed={Boolean(props.revealedHands[viewer().player_id])}
                selectedAttackers={props.selectedAttackers}
                viewer={viewer()}
              />
            )}
          </Show>
        </section>

        <aside class="table-sidebar">
          <section class="panel sidebar-panel">
            <div class="panel-head">
              <div>
                <p class="eyebrow sidebar-eyebrow">Session</p>
                <h2>Duel state</h2>
              </div>
              <span class="chip">Game {props.state.game.game_id}</span>
            </div>
            <div class="sidebar-stat-grid">
              <SidebarMetric label="Subset" value={props.state.game.playable_subset_version} />
              <SidebarMetric label="Phase" value={formatPhase(props.state.game.phase)} />
              <SidebarMetric
                label="Priority"
                value={props.state.game.priority_holder ?? "closed"}
              />
              <SidebarMetric label="Game over" value={props.state.game.is_over ? "yes" : "no"} />
            </div>

            <Show when={props.state.last_command}>
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

          <section class="panel sidebar-panel">
            <div class="panel-head">
              <div>
                <p class="eyebrow sidebar-eyebrow">Replay</p>
                <h2>Public timeline</h2>
              </div>
            </div>
            <Timeline entries={props.state.event_log} />
          </section>
        </aside>
      </div>
    </div>
  );
};

const SeatPanel: Component<{
  viewer: ArenaViewerState;
  game: ArenaGameView;
  orientation: "top" | "bottom";
  revealed: boolean;
  selectedAttackers: string[];
  blockerAssignments: Record<string, string>;
  onToggleHandReveal: (playerId: string) => void;
  onToggleAttackerSelection: (cardId: string) => void;
  onSetBlockerAssignment: (blockerId: string, attackerId: string) => void;
  onRun: (operation: (current: WebArenaClient) => ArenaState) => void;
}> = (props) => {
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
        "seat-panel": true,
        [`seat-${props.orientation}`]: true,
        active: props.viewer.is_active,
        priority: props.viewer.is_priority_holder,
      }}
    >
      <header class="seat-banner">
        <div class="seat-banner-copy">
          <p class="seat-title">{props.viewer.player_id}</p>
          <Show when={viewerPlayer()}>
            {(player) => (
              <p class="seat-summary">
                Life {player().life} · hand {player().hand_count} · mana {player().mana_total} ·
                library {player().library_count}
              </p>
            )}
          </Show>
        </div>
        <div class="chip-row">
          <Show when={props.viewer.is_active}>
            <span class="chip chip-ember">Active</span>
          </Show>
          <Show when={props.viewer.is_priority_holder}>
            <span class="chip chip-night">Priority</span>
          </Show>
        </div>
      </header>

      <Show when={viewerPlayer()}>
        {(player) => (
          <div class="seat-zones">
            <div class="zone-pocket">
              <p class="label">Graveyard</p>
              <CompactZone
                cards={player().graveyard}
                emptyLabel="No graveyard cards"
                title={`${String(player().graveyard.length)} cards`}
              />
            </div>

            <section class="battlefield-lane">
              <div class="zone-head">
                <div>
                  <p class="label">Battlefield</p>
                  <h3>Board presence</h3>
                </div>
                <span class="chip">{String(player().battlefield.length)} permanents</span>
              </div>
              <Show
                when={player().battlefield.length > 0}
                fallback={<p class="muted">Nothing on the battlefield yet.</p>}
              >
                <div class="battlefield-strip">
                  <For each={player().battlefield}>
                    {(card) => (
                      <GameCard
                        attacking={card.attacking}
                        blocking={card.blocking}
                        cardType={card.card_type}
                        definitionId={card.definition_id}
                        keywords={card.keywords}
                        loyalty={card.loyalty}
                        mode="battlefield"
                        power={card.power}
                        summoningSickness={card.summoning_sickness}
                        tapped={card.tapped}
                        token={card.token}
                        toughness={card.toughness}
                      />
                    )}
                  </For>
                </div>
              </Show>
            </section>

            <div class="zone-pocket">
              <p class="label">Exile</p>
              <CompactZone
                cards={player().exile}
                emptyLabel="Nothing in exile"
                title={`${String(player().exile.length)} cards`}
              />
            </div>
          </div>
        )}
      </Show>

      <section class="seat-ops">
        <div class="seat-command-bar">
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
              Resolve combat
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

        <div class="seat-ops-grid">
          <section class="seat-module hand-module">
            <div class="panel-head seat-module-head">
              <div>
                <p class="label">Private hand</p>
                <h3>Seat tray</h3>
              </div>
              <button
                class="hero-button hero-button-ghost mini-button"
                onClick={() => {
                  props.onToggleHandReveal(props.viewer.player_id);
                }}
              >
                {props.revealed ? "Hide hand" : "Reveal hand"}
              </button>
            </div>

            <Show
              when={props.revealed}
              fallback={<p class="muted">Hand hidden while the seat is being passed.</p>}
            >
              <div class="hand-fan">
                <For each={props.viewer.hand}>
                  {(card, index) => (
                    <GameCard
                      actions={
                        <>
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
                                props.onRun((current) =>
                                  discardForCleanup(current, playerId, cardId),
                                );
                              }}
                            >
                              Discard
                            </button>
                          </Show>
                        </>
                      }
                      activatedAbility={card.has_activated_ability}
                      cardType={card.card_type}
                      definitionId={card.definition_id}
                      fanCount={props.viewer.hand.length}
                      index={index()}
                      keywords={card.keywords}
                      loyalty={card.loyalty}
                      manaCost={card.mana_cost}
                      mode="hand"
                      openPriority={card.can_cast_in_open_priority}
                      ownTurnPriority={card.can_cast_in_open_priority_during_own_turn}
                      power={card.power}
                      toughness={card.toughness}
                    />
                  )}
                </For>
              </div>
            </Show>
          </section>

          <section class="seat-module control-module">
            <div class="panel-head seat-module-head">
              <div>
                <p class="label">Controls</p>
                <h3>Action rail</h3>
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
                <div class="panel-head seat-module-head">
                  <div>
                    <p class="label">Combat</p>
                    <h3>Declare attackers</h3>
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
                <div class="panel-head seat-module-head">
                  <div>
                    <p class="label">Combat</p>
                    <h3>Declare blockers</h3>
                  </div>
                </div>
                <div class="blocker-grid">
                  <For each={declareBlockersAction()?.blocker_options ?? []}>
                    {(option) => (
                      <label class="blocker-select">
                        <span>{battlefieldCardLabel(props.game, option.blocker_id)}</span>
                        <select
                          onInput={(event) => {
                            props.onSetBlockerAssignment(
                              option.blocker_id,
                              event.currentTarget.value,
                            );
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
              <p class="muted">No battlefield controls for this seat right now.</p>
            </Show>
          </section>

          <section class="seat-module prompt-module">
            <div class="panel-head seat-module-head">
              <div>
                <p class="label">Prompts</p>
                <h3>Viewer surface</h3>
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
        </div>
      </section>
    </section>
  );
};

const SidebarMetric: Component<{ label: string; value: string }> = (props) => (
  <div class="sidebar-metric">
    <p class="label">{props.label}</p>
    <p class="value">{props.value}</p>
  </div>
);

const StatusBadge: Component<{
  label: string;
  value: string;
  tone: "ember" | "forest" | "night";
}> = (props) => (
  <article classList={{ "status-badge": true, [`status-${props.tone}`]: true }}>
    <p>{props.label}</p>
    <strong>{props.value}</strong>
  </article>
);

const SupportedPrompt: Component<{
  prompt: ArenaChoicePrompt;
  playerId: string;
  hand: ArenaHandCard[];
  onRun: (operation: (current: WebArenaClient) => ArenaState) => void;
}> = (props) => {
  return (
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
                    const chosenCardId = itemId;
                    props.onRun((current) =>
                      resolvePendingHandChoice(current, playerId, chosenCardId),
                    );
                  }}
                >
                  {handCardLabel(props.hand, itemId)}
                </button>
              )}
            </For>
          </div>
        </Match>
      </Switch>
    </article>
  );
};

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

function formatPhase(phase: string): string {
  return phase.replace(/([A-Z])/g, " $1").trim();
}
