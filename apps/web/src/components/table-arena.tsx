import { For, Match, Show, Switch, createEffect, createSignal, onCleanup, untrack } from "solid-js";
import type { Component, JSX } from "solid-js";
import { GameCard } from "./cards/game-card";
import { CardBack } from "./cards/card-back";
import { CardPile } from "./cards/card-pile";
import type { ArenaSessionInfo } from "../lib/session";
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
  type ArenaCommandTarget,
} from "../lib/runtime";
import type {
  ArenaBattlefieldCard,
  ArenaCardView,
  ArenaChoicePrompt,
  ArenaGameView,
  ArenaHandCard,
  ArenaLegalAction,
  ArenaManaCost,
  ArenaManaPool,
  ArenaPlayerView,
  ArenaStackObject,
  ArenaState,
  ArenaTimelineEntry,
  ArenaViewerState,
  BlockerAssignmentInput,
} from "../lib/types";

interface TableArenaProps {
  onCopyInviteLink?: (() => void) | undefined;
  state: ArenaState;
  sessionInfo: ArenaSessionInfo | null;
  selectedAttackers: string[];
  blockerAssignments: Record<string, string>;
  onToggleAttackerSelection: (cardId: string) => void;
  onSetBlockerAssignment: (blockerId: string, attackerId: string) => void;
  onRun: (operation: (current: ArenaCommandTarget) => Promise<ArenaState>) => void;
}

type ZoneKind = "library" | "graveyard" | "exile";

type ZoneBrowserState = {
  playerId: string;
  zone: ZoneKind;
} | null;

interface BattlefieldCardActionOption {
  label: string;
  tone?: "primary" | "default";
  run: () => void;
}

interface InspectCardState {
  sourceCardId?: string | undefined;
  definitionId: string;
  cardType: string;
  zoneLabel: string;
  manaCost?: number | null | undefined;
  manaCostProfile?: ArenaManaCost | undefined;
  power?: number | null | undefined;
  toughness?: number | null | undefined;
  loyalty?: number | null | undefined;
  keywords: string[];
  token?: boolean | undefined;
  tapped?: boolean | undefined;
  summoningSickness?: boolean | undefined;
  attacking?: boolean | undefined;
  blocking?: boolean | undefined;
  note?: string | undefined;
}

interface BattlefieldLayoutPoint {
  x: number;
  y: number;
}

interface FloatingDockPosition {
  x: number;
  y: number;
}

interface HandPointerDragState {
  active: boolean;
  cardId: string;
  clientX: number;
  clientY: number;
}

export const TableArena: Component<TableArenaProps> = (props) => {
  const bottomViewer = () =>
    props.state.viewers.find((viewer) => viewer.player_id === props.sessionInfo?.localSeatId) ??
    props.state.viewers[0];
  const topViewer = () =>
    props.state.viewers.find((viewer) => viewer.player_id !== bottomViewer()?.player_id) ??
    bottomViewer();
  const bottomPlayer = () => findPlayer(props.state.game, bottomViewer()?.player_id ?? "");
  const topPlayer = () => findPlayer(props.state.game, topViewer()?.player_id ?? "");
  const bottomPassAction = () => {
    const viewer = bottomViewer();
    return viewer ? findAction(viewer, "PassPriority") : undefined;
  };
  const bottomConcedeAction = () => {
    const viewer = bottomViewer();
    return viewer ? findAction(viewer, "Concede") : undefined;
  };
  const [sidebarOpen, setSidebarOpen] = createSignal(false);
  const [zoneBrowser, setZoneBrowser] = createSignal<ZoneBrowserState>(null);
  const [inspectedCard, setInspectedCard] = createSignal<InspectCardState | null>(null);
  const [draggedCardId, setDraggedCardId] = createSignal<string | null>(null);
  const [stackModalOpen, setStackModalOpen] = createSignal(false);
  const zoneBrowserPlayer = () => {
    const browser = zoneBrowser();
    if (!browser) {
      return undefined;
    }

    return findPlayer(props.state.game, browser.playerId);
  };
  const zoneBrowserCards = () => {
    const browser = zoneBrowser();
    const player = zoneBrowserPlayer();
    if (!browser || !player) {
      return [];
    }

    switch (browser.zone) {
      case "graveyard":
        return player.graveyard;
      case "exile":
        return player.exile;
      case "library":
        return [];
    }
  };

  const playDraggedCard = (viewer: ArenaViewerState, cardId: string) => {
    const playLandIds = new Set(findAction(viewer, "PlayLand")?.card_ids ?? []);
    const castAction = findAction(viewer, "CastSpell");
    const castableIds = new Set(castAction?.card_ids ?? []);
    const handCard = viewer.hand.find((card) => card.card_id === cardId);

    if (playLandIds.has(cardId)) {
      props.onRun((current) => playLand(current, viewer.player_id, cardId));
      return;
    }

    if (
      castableIds.has(cardId) &&
      handCard &&
      !handCard.requires_target &&
      !handCard.requires_choice
    ) {
      props.onRun((current) => castSpell(current, viewer.player_id, cardId));
    }
  };

  createEffect(() => {
    if (props.state.game.stack.length === 0) {
      setStackModalOpen(false);
    }
  });

  return (
    <div class="table-shell">
      <header class="arena-cockpit panel">
        <div class="arena-brand-cluster">
          <div class="arena-brand">
            <p class="eyebrow">DemonicTutor</p>
            <h1>Duel Arena</h1>
          </div>
          <div class="arena-brand-meta">
            <Show when={props.sessionInfo}>
              {(sessionInfo) => (
                <>
                  <MetaRune
                    icon="room"
                    title={`Room ${shortRoomCode(sessionInfo().roomId)}`}
                    value={shortRoomCode(sessionInfo().roomId)}
                  />
                  <MetaRune
                    icon={sessionInfo().role === "peer" ? "peer" : "host"}
                    title={formatSessionRole(sessionInfo().role)}
                    value={formatSessionRole(sessionInfo().role)}
                  />
                </>
              )}
            </Show>
          </div>
        </div>

        <div class="arena-cockpit-hud">
          <TurnRune turnNumber={props.state.game.turn_number} />
          <PhaseTrack currentPhase={props.state.game.phase} />
        </div>

        <div class="arena-cockpit-actions">
          <Show when={bottomViewer()}>
            {(viewer) => (
              <MetaRune
                icon="seat"
                title={`Local seat ${shortPlayerTag(viewer().player_id)}`}
                value={shortPlayerTag(viewer().player_id)}
              />
            )}
          </Show>
          <Show
            when={props.onCopyInviteLink && props.sessionInfo?.transport === "broadcast-channel"}
          >
            <button
              aria-label="Copy duel room link"
              class="hero-button hero-button-ghost mini-button rune-button"
              title="Copy duel room link"
              onClick={() => {
                props.onCopyInviteLink?.();
              }}
            >
              <HudIcon icon="room" />
            </button>
          </Show>
          <button
            aria-label="Open replay log"
            class="hero-button hero-button-ghost mini-button rune-button"
            title="Open replay log"
            onClick={() => {
              setSidebarOpen((open) => !open);
            }}
          >
            <HudIcon icon="log" />
          </button>
          <button
            aria-label="Reset duel"
            class="hero-button rune-button"
            title="Reset duel"
            onClick={() => {
              props.onRun(resetArena);
            }}
          >
            <HudIcon icon="reset" />
          </button>
        </div>
      </header>

      <div class="table-layout">
        <section
          classList={{
            "duel-table": true,
            panel: true,
          }}
        >
          <aside class="duel-side-rail">
            <div class="duel-side-rail-slot duel-side-rail-slot-top">
              <Show when={topViewer() && topPlayer()}>
                {(player) => (
                  <RailSeatCard
                    handCount={player().hand_count}
                    life={player().life}
                    manaTotal={player().mana_total}
                    manaPool={player().mana_pool}
                    name={formatPlayerDisplayName(player().player_id)}
                    orientation="top"
                    playerId={player().player_id}
                    priority={topViewer()?.is_priority_holder ?? false}
                  />
                )}
              </Show>
            </div>

            <div class="duel-side-rail-slot duel-side-rail-slot-stack">
              <Show when={props.state.game.stack.length > 0}>
                <button
                  aria-label={`Open stack with ${String(props.state.game.stack.length)} objects`}
                  class="rail-stack-dock"
                  title="Open stack"
                  type="button"
                  onClick={() => {
                    setStackModalOpen(true);
                  }}
                >
                  <StackDock
                    count={props.state.game.stack.length}
                    topObject={props.state.game.stack.at(-1)}
                  />
                </button>
              </Show>
            </div>

            <div class="duel-side-rail-slot duel-side-rail-slot-bottom">
              <Show when={bottomViewer() && bottomPlayer()}>
                {(player) => (
                  <RailSeatCard
                    actions={
                      <>
                        <Show when={bottomConcedeAction()}>
                          <button
                            class="rail-seat-action rail-seat-action-danger"
                            type="button"
                            onClick={() => {
                              const playerId = player().player_id;
                              props.onRun((current) => concede(current, playerId));
                            }}
                          >
                            <span class="rail-seat-action-icon" aria-hidden="true">
                              ⦸
                            </span>
                            <span>Concede</span>
                          </button>
                        </Show>
                        <Show when={bottomPassAction()}>
                          <button
                            class="rail-seat-action rail-seat-action-primary"
                            type="button"
                            onClick={() => {
                              const playerId = player().player_id;
                              props.onRun((current) => passPriority(current, playerId));
                            }}
                          >
                            <span class="rail-seat-action-icon" aria-hidden="true">
                              ⇢
                            </span>
                            <span>Pass</span>
                          </button>
                        </Show>
                      </>
                    }
                    handCount={player().hand_count}
                    life={player().life}
                    manaTotal={player().mana_total}
                    manaPool={player().mana_pool}
                    name={formatPlayerDisplayName(player().player_id)}
                    orientation="bottom"
                    playerId={player().player_id}
                    priority={bottomViewer()?.is_priority_holder ?? false}
                  />
                )}
              </Show>
            </div>
          </aside>

          <Show when={topViewer()}>
            {(viewer) => (
              <SeatPanel
                blockerAssignments={props.blockerAssignments}
                game={props.state.game}
                onRun={props.onRun}
                onSetBlockerAssignment={props.onSetBlockerAssignment}
                onToggleAttackerSelection={props.onToggleAttackerSelection}
                onInspectCard={setInspectedCard}
                onOpenZoneBrowser={setZoneBrowser}
                onDragHandCard={setDraggedCardId}
                inspectedCardId={inspectedCard()?.sourceCardId ?? null}
                orientation="top"
                selectedAttackers={props.selectedAttackers}
                viewer={viewer()}
                draggedHandCardId={draggedCardId()}
              />
            )}
          </Show>

          <section class="table-centerline">
            <div class="battlefield-divider">
              <span class="battlefield-divider-mark">
                <Show when={props.state.game.is_over} fallback={"VS"}>
                  {shortPlayerTag(props.state.game.winner_id)}
                </Show>
              </span>
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
                onInspectCard={setInspectedCard}
                onOpenZoneBrowser={setZoneBrowser}
                onDragHandCard={setDraggedCardId}
                onBattlefieldDropCard={(cardId) => {
                  playDraggedCard(viewer(), cardId);
                }}
                inspectedCardId={inspectedCard()?.sourceCardId ?? null}
                orientation="bottom"
                selectedAttackers={props.selectedAttackers}
                viewer={viewer()}
                draggedHandCardId={draggedCardId()}
              />
            )}
          </Show>
        </section>
      </div>

      <Show when={sidebarOpen()}>
        <div
          class="table-modal-backdrop"
          onClick={() => {
            setSidebarOpen(false);
          }}
        >
          <aside
            class="table-sidebar panel open"
            onClick={(event) => {
              event.stopPropagation();
            }}
          >
            <div class="table-sidebar-head">
              <div>
                <p class="eyebrow sidebar-eyebrow">Replay</p>
                <h2>Match log</h2>
              </div>
              <div class="chip-row">
                <span class="chip">Game {props.state.game.game_id}</span>
                <button
                  class="hero-button hero-button-ghost mini-button"
                  onClick={() => {
                    setSidebarOpen(false);
                  }}
                >
                  Close
                </button>
              </div>
            </div>

            <div class="sidebar-stat-grid">
              <SidebarMetric label="Subset" value={props.state.game.playable_subset_version} />
              <SidebarMetric label="Phase" value={formatPhase(props.state.game.phase)} />
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

            <Timeline entries={props.state.event_log} />
          </aside>
        </div>
      </Show>

      <Show when={zoneBrowser()}>
        {(browser) => (
          <div
            class="table-modal-backdrop"
            onClick={() => {
              setZoneBrowser(null);
            }}
          >
            <aside
              class="table-sidebar panel open zone-browser-modal"
              onClick={(event) => {
                event.stopPropagation();
              }}
            >
              <div class="table-sidebar-head">
                <div>
                  <p class="eyebrow sidebar-eyebrow">Zone browser</p>
                  <h2>
                    {shortPlayerTag(browser().playerId)} · {formatZoneLabel(browser().zone)}
                  </h2>
                </div>
                <button
                  class="hero-button hero-button-ghost mini-button"
                  onClick={() => {
                    setZoneBrowser(null);
                  }}
                >
                  Close
                </button>
              </div>

              <Show when={browser().zone === "library"}>
                <div class="zone-browser-empty">
                  <p class="label">Library</p>
                  <strong>{String(zoneBrowserPlayer()?.library_count ?? 0)} cards remaining</strong>
                  <p class="muted">
                    Library contents stay hidden. Use this browser to confirm counts without
                    covering the battlefield.
                  </p>
                </div>
              </Show>

              <Show when={browser().zone === "graveyard" || browser().zone === "exile"}>
                <div class="zone-browser-grid">
                  <For each={zoneBrowserCards()}>
                    {(card) => (
                      <button
                        class="zone-browser-card"
                        onClick={() => {
                          setInspectedCard(
                            inspectFromZoneCard(card, formatZoneLabel(browser().zone)),
                          );
                        }}
                      >
                        <span>{card.definition_id}</span>
                        <small>{card.card_type}</small>
                      </button>
                    )}
                  </For>
                </div>
                <Show when={zoneBrowserCards().length === 0}>
                  <p class="muted">{formatZoneLabel(browser().zone)} empty.</p>
                </Show>
              </Show>
            </aside>
          </div>
        )}
      </Show>

      <Show when={stackModalOpen()}>
        <div
          class="table-modal-backdrop"
          onClick={() => {
            setStackModalOpen(false);
          }}
        >
          <aside
            class="table-sidebar panel open stack-modal"
            onClick={(event) => {
              event.stopPropagation();
            }}
          >
            <div class="table-sidebar-head">
              <div>
                <p class="eyebrow sidebar-eyebrow">Stack</p>
                <h2>Resolve lane</h2>
              </div>
              <div class="chip-row">
                <span class="chip chip-night">
                  {`${String(props.state.game.stack.length)} objects`}
                </span>
                <button
                  class="hero-button hero-button-ghost mini-button"
                  onClick={() => {
                    setStackModalOpen(false);
                  }}
                >
                  Close
                </button>
              </div>
            </div>

            <div class="stack-modal-hero">
              <div class="stack-modal-rune" aria-hidden="true">
                ✦
              </div>
              <div>
                <p class="label">Current stack</p>
                <strong>Last in, first out</strong>
                <p class="muted">
                  Open priority objects appear here in resolution order without covering the
                  battlefield.
                </p>
              </div>
            </div>

            <StackView stack={props.state.game.stack} />
          </aside>
        </div>
      </Show>

      <Show when={inspectedCard()}>
        {(card) => (
          <div
            class="table-modal-backdrop"
            onClick={() => {
              setInspectedCard(null);
            }}
          >
            <aside
              class="table-sidebar panel open card-inspect-modal"
              onClick={(event) => {
                event.stopPropagation();
              }}
            >
              <div class="table-sidebar-head">
                <div>
                  <p class="eyebrow sidebar-eyebrow">Card inspect</p>
                  <h2>{card().definitionId}</h2>
                </div>
                <button
                  class="hero-button hero-button-ghost mini-button"
                  onClick={() => {
                    setInspectedCard(null);
                  }}
                >
                  Close
                </button>
              </div>

              <div class="card-inspect-layout">
                <div class="card-inspect-surface">
                  <GameCard
                    attacking={card().attacking}
                    blocking={card().blocking}
                    cardType={card().cardType}
                    definitionId={card().definitionId}
                    keywords={card().keywords}
                    loyalty={card().loyalty}
                    manaCost={card().manaCost}
                    manaCostProfile={card().manaCostProfile}
                    mode="detail"
                    power={card().power}
                    summoningSickness={card().summoningSickness}
                    tapped={false}
                    token={card().token}
                    toughness={card().toughness}
                  />
                </div>

                <div class="card-inspect-details">
                  <SidebarMetric label="Zone" value={card().zoneLabel} />
                  <SidebarMetric label="Type" value={card().cardType} />
                  <Show when={card().power !== undefined && card().power !== null}>
                    <SidebarMetric
                      label="Power / Toughness"
                      value={`${String(card().power)}/${String(card().toughness ?? 0)}`}
                    />
                  </Show>
                  <Show when={card().loyalty !== undefined && card().loyalty !== null}>
                    <SidebarMetric label="Loyalty" value={String(card().loyalty)} />
                  </Show>
                  <Show
                    when={
                      (card().manaCostProfile !== undefined ||
                        (card().manaCost !== undefined && card().manaCost !== null)) &&
                      !card().cardType.toLowerCase().includes("land")
                    }
                  >
                    <SidebarMetric
                      label="Mana"
                      value={formatManaCostSummary(card().manaCostProfile, card().manaCost)}
                    />
                  </Show>
                  <Show when={card().keywords.length > 0}>
                    <div class="chip-row">
                      <For each={card().keywords}>
                        {(keyword) => <span class="chip">{keyword}</span>}
                      </For>
                    </div>
                  </Show>
                  <Show when={card().note}>
                    <p class="support-note">{card().note}</p>
                  </Show>
                </div>
              </div>
            </aside>
          </div>
        )}
      </Show>
    </div>
  );
};

const SeatPanel: Component<{
  viewer: ArenaViewerState;
  game: ArenaGameView;
  orientation: "top" | "bottom";
  selectedAttackers: string[];
  blockerAssignments: Record<string, string>;
  onToggleAttackerSelection: (cardId: string) => void;
  onSetBlockerAssignment: (blockerId: string, attackerId: string) => void;
  onOpenZoneBrowser: (state: ZoneBrowserState) => void;
  onInspectCard: (card: InspectCardState | null) => void;
  onDragHandCard: (cardId: string | null) => void;
  onBattlefieldDropCard?: (cardId: string) => void;
  onRun: (operation: (current: ArenaCommandTarget) => Promise<ArenaState>) => void;
  draggedHandCardId: string | null;
  inspectedCardId: string | null;
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
  const playLandIds = () => new Set(playLandAction()?.card_ids ?? []);
  const castSpellIds = () => new Set(castSpellAction()?.card_ids ?? []);
  const tapManaSourceIds = () => new Set(tapManaSourceAction()?.card_ids ?? []);
  const activateAbilityIds = () => new Set(activateAbilityAction()?.card_ids ?? []);
  const discardForCleanupIds = () => new Set(discardForCleanupAction()?.card_ids ?? []);
  const handPrompt = () =>
    props.viewer.choice_requests.find((prompt) => prompt.kind === "PendingHandChoice");
  const spotlightPrompts = () =>
    props.viewer.choice_requests.filter((prompt) =>
      ["PendingScry", "PendingSurveil", "OptionalEffectDecision"].includes(prompt.kind),
    );
  const unsupportedPrompts = () =>
    props.viewer.choice_requests.filter(
      (prompt) =>
        !["PendingScry", "PendingSurveil", "PendingHandChoice", "OptionalEffectDecision"].includes(
          prompt.kind,
        ),
    );
  const blockerTargets = () => uniqueAttackerIds(declareBlockersAction());
  const attackerCandidateIds = () => new Set(declareAttackersAction()?.card_ids ?? []);
  const battlefieldConsoleVisible = () =>
    Boolean(declareAttackersAction()) || Boolean(declareBlockersAction());
  const hiddenOpponentHandCount = () =>
    props.orientation === "top" ? (viewerPlayer()?.hand_count ?? 0) : 0;
  const blockerSummary = () =>
    Object.entries(props.blockerAssignments).map(([blockerId, attackerId]) => ({
      blockerId,
      attackerId,
    }));
  const showPrivateTray = () => props.orientation === "bottom";
  const [handOrder, setHandOrder] = createSignal<string[]>([]);
  const [focusedHandCardId, setFocusedHandCardId] = createSignal<string | null>(null);
  const [battlefieldOrder, setBattlefieldOrder] = createSignal<string[]>([]);
  const [battlefieldPositions, setBattlefieldPositions] = createSignal<
    Record<string, BattlefieldLayoutPoint>
  >({});
  const [draggedBattlefieldCardId, setDraggedBattlefieldCardId] = createSignal<string | null>(null);
  const [handPointerDrag, setHandPointerDrag] = createSignal<HandPointerDragState | null>(null);
  const [battlefieldActionMenuCardId, setBattlefieldActionMenuCardId] = createSignal<string | null>(
    null,
  );
  let battlefieldSurfaceRef: HTMLDivElement | undefined;
  let battlefieldLaneRef: HTMLElement | undefined;
  let handFanRef: HTMLDivElement | undefined;
  let clearHandPointerTracking: (() => void) | undefined;

  createEffect(() => {
    const nextIds = props.viewer.hand.map((card) => card.card_id);
    setHandOrder((previous) => {
      const retained = previous.filter((cardId) => nextIds.includes(cardId));
      const appended = nextIds.filter((cardId) => !retained.includes(cardId));
      return [...retained, ...appended];
    });
  });

  const orderedHand = () => {
    const cardsById = new Map(props.viewer.hand.map((card) => [card.card_id, card]));
    return handOrder()
      .map((cardId) => cardsById.get(cardId))
      .filter((card): card is ArenaHandCard => card !== undefined);
  };

  createEffect(() => {
    const nextIds = viewerPlayer()?.battlefield.map((card) => card.card_id) ?? [];
    setBattlefieldOrder((previous) => {
      const retained = previous.filter((cardId) => nextIds.includes(cardId));
      const appended = nextIds.filter((cardId) => !retained.includes(cardId));
      return [...retained, ...appended];
    });
  });

  const orderedBattlefield = () => {
    const cardsById = new Map(
      (viewerPlayer()?.battlefield ?? []).map((card) => [card.card_id, card]),
    );
    return battlefieldOrder()
      .map((cardId) => cardsById.get(cardId))
      .filter((card): card is ArenaBattlefieldCard => card !== undefined);
  };

  createEffect(() => {
    const cards = orderedBattlefield();
    setBattlefieldPositions((previous) => {
      const next: Record<string, BattlefieldLayoutPoint> = {};

      cards.forEach((card, index) => {
        next[card.card_id] =
          previous[card.card_id] ??
          defaultBattlefieldPosition(index, cards.length, props.orientation);
      });

      return next;
    });
  });

  createEffect(() => {
    const hand = orderedHand();
    if (hand.length === 0) {
      setFocusedHandCardId(null);
      return;
    }

    const focusedCardId = focusedHandCardId();
    if (focusedCardId && hand.some((card) => card.card_id === focusedCardId)) {
      return;
    }

    setFocusedHandCardId(null);
  });

  createEffect(() => {
    const openCardId = battlefieldActionMenuCardId();
    if (!openCardId) {
      return;
    }

    const battlefield = orderedBattlefield();
    if (!battlefield.some((card) => card.card_id === openCardId)) {
      setBattlefieldActionMenuCardId(null);
    }
  });

  const moveHandCard = (draggedCardId: string, targetCardId: string) => {
    setHandOrder((previous) => reorderCardIds(previous, draggedCardId, targetCardId));
  };

  const resolveFocusedHandCardId = (clientX: number): string | null => {
    const hand = orderedHand();
    const handFan = handFanRef;

    if (hand.length === 0 || !handFan) {
      return null;
    }

    const rect = handFan.getBoundingClientRect();
    if (rect.width <= 0) {
      return hand[0]?.card_id ?? null;
    }

    const midpoint = (hand.length - 1) / 2;
    const spread = hand.length >= 8 ? 58 : hand.length >= 6 ? 66 : 74;
    const localX = clientX - (rect.left + rect.width / 2);

    let nearestCardId = hand[0]?.card_id ?? null;
    let nearestDistance = Number.POSITIVE_INFINITY;

    hand.forEach((card, index) => {
      const offset = index - midpoint;
      const cardX = offset * spread;
      const distance = Math.abs(localX - cardX);

      if (distance < nearestDistance) {
        nearestCardId = card.card_id;
        nearestDistance = distance;
      }
    });

    return nearestCardId;
  };

  const openInspectCard = (card: InspectCardState) => {
    setFocusedHandCardId(null);
    setBattlefieldActionMenuCardId(null);
    props.onInspectCard(card);
  };

  const battlefieldCardActions = (card: ArenaBattlefieldCard): BattlefieldCardActionOption[] => {
    const actions: BattlefieldCardActionOption[] = [];

    if (tapManaSourceIds().has(card.card_id)) {
      actions.push({
        label: "Tap for mana",
        tone: "primary",
        run: () => {
          const playerId = props.viewer.player_id;
          props.onRun((current) => tapManaSource(current, playerId, card.card_id));
        },
      });
    }

    if (activateAbilityIds().has(card.card_id)) {
      actions.push({
        label: "Activate ability",
        tone: "primary",
        run: () => {
          const playerId = props.viewer.player_id;
          props.onRun((current) => activateAbility(current, playerId, card.card_id));
        },
      });
    }

    if (attackerCandidateIds().has(card.card_id)) {
      actions.push({
        label: props.selectedAttackers.includes(card.card_id)
          ? "Remove attacker"
          : "Declare attacker",
        run: () => {
          props.onToggleAttackerSelection(card.card_id);
        },
      });
    }

    return actions;
  };

  const handleBattlefieldCardAction = (card: ArenaBattlefieldCard) => {
    const actions = battlefieldCardActions(card);

    if (actions.length === 0) {
      return;
    }

    if (actions.length === 1) {
      setBattlefieldActionMenuCardId(null);
      actions[0]?.run();
      return;
    }

    setBattlefieldActionMenuCardId((current) => (current === card.card_id ? null : card.card_id));
  };

  const canDropDraggedCard = () => canDropHandCard(props.draggedHandCardId);

  const canDropHandCard = (cardId: string | null) =>
    props.orientation === "bottom" &&
    Boolean(cardId) &&
    Boolean(
      props.viewer.hand.find((card) => card.card_id === cardId) &&
      (playLandIds().has(cardId ?? "") ||
        (castSpellIds().has(cardId ?? "") &&
          !props.viewer.hand.find((card) => card.card_id === cardId)?.requires_choice &&
          !props.viewer.hand.find((card) => card.card_id === cardId)?.requires_target)),
    );

  const dropDraggedHandCard = () => {
    if (!canDropDraggedCard()) {
      return;
    }

    const cardId = props.draggedHandCardId;
    if (cardId) {
      props.onBattlefieldDropCard?.(cardId);
    }
    props.onDragHandCard(null);
  };

  const stopHandPointerDrag = () => {
    clearHandPointerTracking?.();
    clearHandPointerTracking = undefined;
    setHandPointerDrag(null);
    props.onDragHandCard(null);
  };

  const startHandPointerTracking = (cardId: string, startX: number, startY: number) => {
    if (props.orientation !== "bottom") {
      return;
    }

    stopHandPointerDrag();

    const activationThreshold = 10;
    let active = false;

    const removeHandPointerListeners = () => {
      window.removeEventListener("pointermove", handlePointerMove);
      window.removeEventListener("pointerup", finishPointerDrag);
      window.removeEventListener("pointercancel", finishPointerDrag);
    };

    const handlePointerMove = (event: PointerEvent) => {
      const distance = Math.hypot(event.clientX - startX, event.clientY - startY);
      if (!active && distance < activationThreshold) {
        return;
      }

      if (!active) {
        active = true;
        setFocusedHandCardId(null);
        props.onDragHandCard(cardId);
      }

      setHandPointerDrag({
        active: true,
        cardId,
        clientX: event.clientX,
        clientY: event.clientY,
      });

      event.preventDefault();
    };

    const finishPointerDrag = (event: PointerEvent) => {
      removeHandPointerListeners();
      clearHandPointerTracking = undefined;

      if (active) {
        const target = document.elementFromPoint(event.clientX, event.clientY);
        const inBattlefield =
          Boolean(target && battlefieldSurfaceRef?.contains(target)) ||
          Boolean(target && battlefieldLaneRef?.contains(target));

        if (inBattlefield && untrack(() => canDropHandCard(cardId))) {
          props.onBattlefieldDropCard?.(cardId);
        } else {
          const handTarget = target?.closest("[data-hand-card-id]");
          const targetCardId = handTarget?.getAttribute("data-hand-card-id");
          if (targetCardId) {
            moveHandCard(cardId, targetCardId);
          }
        }
      }

      setHandPointerDrag(null);
      props.onDragHandCard(null);
    };

    clearHandPointerTracking = removeHandPointerListeners;

    window.addEventListener("pointermove", handlePointerMove, { passive: false });
    window.addEventListener("pointerup", finishPointerDrag, { passive: false });
    window.addEventListener("pointercancel", finishPointerDrag, { passive: false });
  };

  onCleanup(() => {
    stopHandPointerDrag();
  });

  const repositionBattlefieldCard = (cardId: string, clientX: number, clientY: number) => {
    const surface = battlefieldSurfaceRef;
    if (!surface) {
      return;
    }

    const rect = surface.getBoundingClientRect();
    if (rect.width <= 0 || rect.height <= 0) {
      return;
    }

    const nextX = clamp((clientX - rect.left) / rect.width, 0.08, 0.92);
    const nextY = clamp((clientY - rect.top) / rect.height, 0.14, 0.88);

    setBattlefieldPositions((previous) => ({
      ...previous,
      [cardId]: { x: nextX, y: nextY },
    }));
  };

  return (
    <section
      classList={{
        "seat-panel": true,
        [`seat-${props.orientation}`]: true,
        "is-active-turn": props.viewer.is_active,
        "has-spotlight": spotlightPrompts().length > 0 || unsupportedPrompts().length > 0,
      }}
    >
      <Show when={spotlightPrompts().length > 0 || unsupportedPrompts().length > 0}>
        <section class="seat-spotlight-strip">
          <For each={spotlightPrompts()}>
            {(prompt) => (
              <SupportedPrompt
                hand={props.viewer.hand}
                onRun={props.onRun}
                playerId={props.viewer.player_id}
                prompt={prompt}
                variant="spotlight"
              />
            )}
          </For>

          <For each={unsupportedPrompts()}>
            {(prompt) => (
              <article class="unsupported-item unsupported-spotlight">
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
        </section>
      </Show>

      <Show when={hiddenOpponentHandCount() > 0}>
        <div
          aria-label={`${String(hiddenOpponentHandCount())} cards in opponent hand`}
          class="opponent-hand-fan"
        >
          <For each={hiddenHandSlots(hiddenOpponentHandCount())}>
            {(_, index) => (
              <div
                class="opponent-hand-card-shell"
                style={hiddenHandCardShellStyle(index(), hiddenOpponentHandCount())}
              >
                <CardBack variant="hidden-hand" />
              </div>
            )}
          </For>
        </div>
      </Show>

      <Show when={viewerPlayer()}>
        {(player) => (
          <div
            class="seat-zones"
            onDragOver={(event) => {
              if (!canDropDraggedCard()) {
                return;
              }

              event.preventDefault();
              if (event.dataTransfer) {
                event.dataTransfer.dropEffect = "copy";
              }
            }}
            onDrop={(event) => {
              if (!canDropDraggedCard()) {
                return;
              }

              event.preventDefault();
              dropDraggedHandCard();
            }}
          >
            <section
              ref={(element) => {
                battlefieldLaneRef = element;
              }}
              classList={{
                "battlefield-lane": true,
                "drop-ready": canDropDraggedCard(),
              }}
              onClick={() => {
                setBattlefieldActionMenuCardId(null);
              }}
              onDragOver={(event) => {
                if (!canDropDraggedCard()) {
                  return;
                }

                event.preventDefault();
              }}
              onDrop={(event) => {
                if (!canDropDraggedCard()) {
                  return;
                }

                event.preventDefault();
                dropDraggedHandCard();
              }}
            >
              <Show when={props.orientation === "bottom"}>
                <div class="lane-quick-actions">
                  <Show when={advanceTurnAction()}>
                    <button
                      class="action-button action-button-primary"
                      onClick={() => {
                        props.onRun(advanceTurn);
                      }}
                    >
                      Next
                    </button>
                  </Show>
                  <Show when={resolveCombatDamageAction()}>
                    <button
                      class="action-button action-button-primary"
                      onClick={() => {
                        const playerId = props.viewer.player_id;
                        props.onRun((current) => resolveCombatDamage(current, playerId));
                      }}
                    >
                      Damage
                    </button>
                  </Show>
                </div>
              </Show>

              <div
                class="battlefield-surface"
                ref={(element) => {
                  battlefieldSurfaceRef = element;
                }}
                onDragOver={(event) => {
                  if (!props.draggedHandCardId && !draggedBattlefieldCardId()) {
                    return;
                  }

                  event.preventDefault();
                  if (event.dataTransfer) {
                    event.dataTransfer.dropEffect = props.draggedHandCardId ? "copy" : "move";
                  }
                }}
                onDrop={(event) => {
                  event.preventDefault();

                  if (props.draggedHandCardId) {
                    dropDraggedHandCard();
                  }

                  const draggedBattlefieldCard = draggedBattlefieldCardId();
                  if (draggedBattlefieldCard) {
                    repositionBattlefieldCard(draggedBattlefieldCard, event.clientX, event.clientY);
                    setDraggedBattlefieldCardId(null);
                  }
                }}
              >
                <Show
                  when={player().battlefield.length > 0}
                  fallback={
                    <div class="battlefield-empty-state" aria-label="Battlefield empty">
                      <span class="battlefield-empty-rune">◌</span>
                    </div>
                  }
                >
                  <For each={orderedBattlefield()}>
                    {(card, index) => (
                      <div
                        classList={{
                          "battlefield-card-shell": true,
                          draggable: props.orientation === "bottom",
                          dragging: draggedBattlefieldCardId() === card.card_id,
                          selected:
                            battlefieldActionMenuCardId() === card.card_id ||
                            props.inspectedCardId === card.card_id,
                        }}
                        draggable={props.orientation === "bottom"}
                        onClick={(event) => {
                          event.stopPropagation();
                          if (props.orientation !== "bottom") {
                            return;
                          }

                          handleBattlefieldCardAction(card);
                        }}
                        onDragEnd={() => {
                          setDraggedBattlefieldCardId(null);
                        }}
                        onDragStart={(event) => {
                          if (props.orientation !== "bottom") {
                            return;
                          }

                          setDraggedBattlefieldCardId(card.card_id);
                          if (event.dataTransfer) {
                            event.dataTransfer.setData("text/plain", card.card_id);
                            event.dataTransfer.effectAllowed = "move";
                          }
                        }}
                        style={battlefieldCardShellStyle(
                          battlefieldPositions()[card.card_id] ??
                            defaultBattlefieldPosition(
                              index(),
                              orderedBattlefield().length,
                              props.orientation,
                            ),
                        )}
                      >
                        <GameCard
                          attacking={card.attacking}
                          blocking={card.blocking}
                          cardType={card.card_type}
                          definitionId={card.definition_id}
                          highlighted={
                            tapManaSourceIds().has(card.card_id) ||
                            activateAbilityIds().has(card.card_id) ||
                            attackerCandidateIds().has(card.card_id)
                          }
                          interactive
                          keywords={card.keywords}
                          loyalty={card.loyalty}
                          manaCostProfile={card.mana_cost}
                          mode="battlefield"
                          onClick={() => {
                            if (props.orientation !== "bottom") {
                              return;
                            }

                            handleBattlefieldCardAction(card);
                          }}
                          onInspect={() => {
                            openInspectCard(inspectFromBattlefieldCard(card));
                          }}
                          power={card.power}
                          selected={
                            battlefieldActionMenuCardId() === card.card_id ||
                            props.inspectedCardId === card.card_id
                          }
                          summoningSickness={card.summoning_sickness}
                          tapped={card.tapped}
                          token={card.token}
                          toughness={card.toughness}
                        />
                        <Show
                          when={
                            props.orientation === "bottom" &&
                            battlefieldActionMenuCardId() === card.card_id &&
                            battlefieldCardActions(card).length > 1
                          }
                        >
                          <div
                            class="battlefield-card-popover"
                            onClick={(event) => {
                              event.stopPropagation();
                            }}
                          >
                            <For each={battlefieldCardActions(card)}>
                              {(action) => (
                                <button
                                  classList={{
                                    "action-button": true,
                                    "action-button-primary": action.tone === "primary",
                                  }}
                                  onClick={() => {
                                    action.run();
                                    setBattlefieldActionMenuCardId(null);
                                  }}
                                >
                                  {action.label}
                                </button>
                              )}
                            </For>
                          </div>
                        </Show>
                      </div>
                    )}
                  </For>
                </Show>
              </div>

              <Show when={battlefieldConsoleVisible()}>
                <div class="battlefield-console">
                  <Show when={declareAttackersAction()}>
                    <article class="combat-stage">
                      <div class="panel-head seat-module-head">
                        <div>
                          <p class="label">Combat lane</p>
                          <h3>Choose attackers</h3>
                        </div>
                      </div>
                      <div class="combat-pieces">
                        <For each={declareAttackersAction()?.card_ids ?? []}>
                          {(cardId) => (
                            <button
                              classList={{
                                chip: true,
                                "chip-toggle": true,
                                "combat-toggle": true,
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

                      <Show when={props.selectedAttackers.length > 0}>
                        <div class="combat-preview">
                          <p class="label">Committed attackers</p>
                          <div class="chip-row">
                            <For each={props.selectedAttackers}>
                              {(cardId) => (
                                <span class="chip chip-ember">
                                  {battlefieldCardLabel(props.game, cardId)}
                                </span>
                              )}
                            </For>
                          </div>
                        </div>
                      </Show>

                      <button
                        class="hero-button seat-button"
                        onClick={() => {
                          const playerId = props.viewer.player_id;
                          const attackerIds = [...props.selectedAttackers];
                          props.onRun((current) =>
                            declareAttackers(current, playerId, attackerIds),
                          );
                        }}
                      >
                        Commit attackers
                      </button>
                    </article>
                  </Show>

                  <Show when={declareBlockersAction()}>
                    <article class="combat-stage">
                      <div class="panel-head seat-module-head">
                        <div>
                          <p class="label">Combat lane</p>
                          <h3>Assign blockers</h3>
                        </div>
                      </div>

                      <Show when={blockerTargets().length > 0}>
                        <div class="combat-preview">
                          <p class="label">Incoming attackers</p>
                          <div class="chip-row">
                            <For each={blockerTargets()}>
                              {(attackerId) => (
                                <span class="chip chip-ember">
                                  {battlefieldCardLabel(props.game, attackerId)}
                                </span>
                              )}
                            </For>
                          </div>
                        </div>
                      </Show>

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

                      <Show when={blockerSummary().length > 0}>
                        <div class="combat-preview">
                          <p class="label">Current blocks</p>
                          <div class="chip-row">
                            <For each={blockerSummary()}>
                              {(assignment) => (
                                <span class="chip chip-forest">
                                  {battlefieldCardLabel(props.game, assignment.blockerId)} on{" "}
                                  {battlefieldCardLabel(props.game, assignment.attackerId)}
                                </span>
                              )}
                            </For>
                          </div>
                        </div>
                      </Show>

                      <button
                        class="hero-button seat-button"
                        onClick={() => {
                          const playerId = props.viewer.player_id;
                          const assignments = blockerAssignmentsToArray(props.blockerAssignments);
                          props.onRun((current) => declareBlockers(current, playerId, assignments));
                        }}
                      >
                        Commit blockers
                      </button>
                    </article>
                  </Show>
                </div>
              </Show>
            </section>

            <aside class="zone-rail">
              <div class="zone-rail-content">
                <Show
                  when={props.orientation === "top"}
                  fallback={
                    <>
                      <CardPile
                        count={player().library_count}
                        kind="library"
                        onClick={() => {
                          props.onOpenZoneBrowser({
                            playerId: props.viewer.player_id,
                            zone: "library",
                          });
                        }}
                      />
                      <CardPile
                        count={player().graveyard.length}
                        kind="graveyard"
                        onClick={() => {
                          props.onOpenZoneBrowser({
                            playerId: props.viewer.player_id,
                            zone: "graveyard",
                          });
                        }}
                        topCard={topZoneCard(player().graveyard)}
                      />
                      <CardPile
                        count={player().exile.length}
                        kind="exile"
                        onClick={() => {
                          props.onOpenZoneBrowser({
                            playerId: props.viewer.player_id,
                            zone: "exile",
                          });
                        }}
                        topCard={topZoneCard(player().exile)}
                      />
                    </>
                  }
                >
                  <>
                    <CardPile
                      count={player().exile.length}
                      kind="exile"
                      onClick={() => {
                        props.onOpenZoneBrowser({
                          playerId: props.viewer.player_id,
                          zone: "exile",
                        });
                      }}
                      topCard={topZoneCard(player().exile)}
                    />
                    <CardPile
                      count={player().graveyard.length}
                      kind="graveyard"
                      onClick={() => {
                        props.onOpenZoneBrowser({
                          playerId: props.viewer.player_id,
                          zone: "graveyard",
                        });
                      }}
                      topCard={topZoneCard(player().graveyard)}
                    />
                    <CardPile
                      count={player().library_count}
                      kind="library"
                      onClick={() => {
                        props.onOpenZoneBrowser({
                          playerId: props.viewer.player_id,
                          zone: "library",
                        });
                      }}
                    />
                  </>
                </Show>
              </div>
            </aside>
          </div>
        )}
      </Show>

      <Show when={showPrivateTray()}>
        <section class="seat-ops">
          <section class="hand-drawer">
            <div class="hand-drawer-body">
              <Show when={handPrompt()}>
                {(prompt) => (
                  <SupportedPrompt
                    hand={props.viewer.hand}
                    onRun={props.onRun}
                    playerId={props.viewer.player_id}
                    prompt={prompt()}
                    variant="private"
                  />
                )}
              </Show>
              <div
                class="hand-fan hand-fan-floating"
                ref={(element) => {
                  handFanRef = element;
                }}
                onMouseMove={(event) => {
                  if (handPointerDrag()?.active) {
                    return;
                  }

                  setFocusedHandCardId(resolveFocusedHandCardId(event.clientX));
                }}
                onMouseLeave={() => {
                  setFocusedHandCardId(null);
                }}
              >
                <For each={orderedHand()}>
                  {(card, index) => (
                    <div
                      data-hand-card-id={card.card_id}
                      classList={{
                        "card-drag-shell": true,
                        "hand-card-shell": true,
                        dragging:
                          handPointerDrag()?.active && handPointerDrag()?.cardId === card.card_id,
                        focused: focusedHandCardId() === card.card_id,
                        selected: props.inspectedCardId === card.card_id,
                      }}
                      onPointerDown={(event) => {
                        if (event.button !== 0) {
                          return;
                        }

                        startHandPointerTracking(card.card_id, event.clientX, event.clientY);
                      }}
                      onFocusIn={() => {
                        setFocusedHandCardId(card.card_id);
                      }}
                      onMouseEnter={() => {
                        setFocusedHandCardId(card.card_id);
                      }}
                      style={handCardShellStyle(
                        index(),
                        orderedHand().length,
                        false,
                        focusedHandCardId() === card.card_id,
                        handPointerDrag()?.active && handPointerDrag()?.cardId === card.card_id
                          ? handPointerDrag()
                          : null,
                      )}
                    >
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
                                  ? "Target UI soon"
                                  : "Cast"}
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
                        highlighted={
                          playLandIds().has(card.card_id) ||
                          castSpellIds().has(card.card_id) ||
                          discardForCleanupIds().has(card.card_id)
                        }
                        index={index()}
                        interactive
                        keywords={card.keywords}
                        loyalty={card.loyalty}
                        manaCost={card.mana_cost}
                        manaCostProfile={card.mana_cost_profile}
                        mode="hand"
                        onInspect={() => {
                          openInspectCard(inspectFromHandCard(card));
                        }}
                        openPriority={card.can_cast_in_open_priority}
                        ownTurnPriority={card.can_cast_in_open_priority_during_own_turn}
                        power={card.power}
                        selected={
                          focusedHandCardId() === card.card_id ||
                          props.inspectedCardId === card.card_id
                        }
                        toughness={card.toughness}
                      />
                    </div>
                  )}
                </For>
              </div>

              <Show when={unsupportedPrompts().length > 0}>
                <details class="unsupported-drawer">
                  <summary class="debug-drawer-summary seat-drawer-summary">
                    <div>
                      <p class="label">Seat read</p>
                      <h3>Tactical notes</h3>
                    </div>
                    <span class="chip chip-night">debug</span>
                  </summary>

                  <p class="support-note">{seatSupportCopy()}</p>

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
                </details>
              </Show>
            </div>
          </section>
        </section>
      </Show>
    </section>
  );
};

const SidebarMetric: Component<{ label: string; value: string }> = (props) => (
  <div class="sidebar-metric">
    <p class="label">{props.label}</p>
    <p class="value">{props.value}</p>
  </div>
);

const RailSeatCard: Component<{
  life: number;
  handCount: number;
  manaTotal: number;
  manaPool: ArenaManaPool;
  name: string;
  orientation: "top" | "bottom";
  playerId: string;
  priority: boolean;
  actions?: JSX.Element;
}> = (props) => (
  <section
    classList={{
      "rail-seat-card": true,
      [`orientation-${props.orientation}`]: true,
      "has-priority": props.priority,
    }}
  >
    <div class="rail-seat-stats" role="list">
      <RailSeatCounter kind="life" value={props.life} />
      <RailSeatCounter kind="hand" value={props.handCount} />
    </div>

    <div class="rail-seat-avatar-wrap">
      <article
        class="rail-seat-avatar-card"
        title={props.priority ? `${props.playerId} has priority` : props.playerId}
      >
        <div class="rail-seat-avatar-surface">
          <SeatAvatarGlyph orientation={props.orientation} />
        </div>
        <div class="rail-seat-nameplate">{props.name}</div>
      </article>

      <Show when={props.manaTotal > 0}>
        <ManaPoolDock manaPool={props.manaPool} orientation={props.orientation} />
      </Show>
    </div>

    <Show when={props.actions}>
      <div class="rail-seat-actions">{props.actions}</div>
    </Show>
  </section>
);

const MANA_POOL_ORDER: {
  key: keyof ArenaManaPool;
  label: string;
}[] = [
  { key: "white", label: "White mana" },
  { key: "blue", label: "Blue mana" },
  { key: "black", label: "Black mana" },
  { key: "red", label: "Red mana" },
  { key: "green", label: "Green mana" },
  { key: "colorless", label: "Colorless mana" },
];

const ManaPoolDock: Component<{
  manaPool: ArenaManaPool;
  orientation: "top" | "bottom";
}> = (props) => {
  const [position, setPosition] = createSignal({ x: 0, y: 0 } satisfies FloatingDockPosition);
  const [dragging, setDragging] = createSignal(false);
  let clearPointerTracking: (() => void) | undefined;

  const stopPointerTracking = () => {
    clearPointerTracking?.();
    clearPointerTracking = undefined;
    setDragging(false);
  };

  const startPointerTracking = (event: PointerEvent & { currentTarget: HTMLDivElement }) => {
    if (event.button !== 0) {
      return;
    }

    event.preventDefault();
    const startX = event.clientX;
    const startY = event.clientY;
    const startPosition = position();

    const handlePointerMove = (moveEvent: PointerEvent) => {
      setDragging(true);
      setPosition({
        x: startPosition.x + moveEvent.clientX - startX,
        y: startPosition.y + moveEvent.clientY - startY,
      });
      moveEvent.preventDefault();
    };

    const finishPointerTracking = () => {
      window.removeEventListener("pointermove", handlePointerMove);
      window.removeEventListener("pointerup", finishPointerTracking);
      window.removeEventListener("pointercancel", finishPointerTracking);
      clearPointerTracking = undefined;
      setDragging(false);
    };

    clearPointerTracking = () => {
      window.removeEventListener("pointermove", handlePointerMove);
      window.removeEventListener("pointerup", finishPointerTracking);
      window.removeEventListener("pointercancel", finishPointerTracking);
    };

    window.addEventListener("pointermove", handlePointerMove, { passive: false });
    window.addEventListener("pointerup", finishPointerTracking, { passive: false });
    window.addEventListener("pointercancel", finishPointerTracking, { passive: false });
  };

  onCleanup(() => {
    stopPointerTracking();
  });

  const manaEntries = () =>
    MANA_POOL_ORDER.map((entry) => ({
      ...entry,
      value: props.manaPool[entry.key],
    }));

  return (
    <div
      classList={{
        "mana-pool-dock": true,
        [`orientation-${props.orientation}`]: true,
        dragging: dragging(),
      }}
      style={{
        "--mana-dock-x": `${String(position().x)}px`,
        "--mana-dock-y": `${String(position().y)}px`,
      }}
      onPointerDown={startPointerTracking}
    >
      <div class="mana-pool-head">
        <HudIcon icon="mana" />
        <span>Mana pool</span>
      </div>
      <div class="mana-pool-body">
        <div class="mana-pool-grid" role="list" aria-label="Mana pool by color">
          <For each={manaEntries()}>
            {(entry) => (
              <div
                classList={{
                  "mana-pool-entry": true,
                  empty: entry.value === 0,
                }}
                role="listitem"
                title={entry.label}
              >
                <span
                  aria-hidden="true"
                  classList={{
                    "mana-pool-gem": true,
                    [`kind-${entry.key}`]: true,
                  }}
                >
                  <ManaPoolGlyph kind={entry.key} />
                </span>
                <strong>{String(entry.value)}</strong>
              </div>
            )}
          </For>
        </div>
      </div>
    </div>
  );
};

const ManaPoolGlyph: Component<{ kind: keyof ArenaManaPool }> = (props) => (
  <svg aria-hidden="true" class="mana-pool-glyph" viewBox="0 0 24 24">
    <Switch>
      <Match when={props.kind === "white"}>
        <path d="M12 3.8 13.9 8l4.5-.4-3.4 2.9 1.1 4.3L12 12.4 7.9 14.8 9 10.5 5.6 7.6l4.5.4Z" />
      </Match>
      <Match when={props.kind === "blue"}>
        <path d="M12 3.5c2.8 3.5 4.2 5.8 4.2 7.9a4.2 4.2 0 1 1-8.4 0c0-2.1 1.4-4.4 4.2-7.9Z" />
      </Match>
      <Match when={props.kind === "black"}>
        <path d="M12 4.3c3.6 0 6.5 2.9 6.5 6.4 0 2.6-1.5 4.9-3.8 5.9l.9 3.1-2.7-1.7-.9 2.5-.9-2.5-2.7 1.7.9-3.1a6.4 6.4 0 0 1-3.8-5.9C5.5 7.2 8.4 4.3 12 4.3Z" />
      </Match>
      <Match when={props.kind === "red"}>
        <path d="M12.8 3.7c2.2 2.7 3.4 4.7 3.4 6.8 0 3.2-2.3 5.8-5.6 6.7 1-1 1.7-2.4 1.7-4 0-1.5-.6-2.9-1.5-3.9 1.1-.3 2-1.3 2-2.5 0-1-.3-2-.9-3.1Z" />
      </Match>
      <Match when={props.kind === "green"}>
        <path d="M17.4 5.1c-5.1.3-9.1 3.4-10.2 8.8-.4 2 1 4 3.1 4.3 4.9.6 8.5-3.7 8.1-9.3-.1-1.4-.4-2.7-1-3.8ZM9.8 15.9c1.2-2.8 3.3-5 6.5-6.4-2 1.7-3.5 3.9-4.6 6.5Z" />
      </Match>
      <Match when={props.kind === "colorless"}>
        <path d="m12 4.2 6.4 3.7v7.3L12 18.8l-6.4-3.6V7.9Z" />
      </Match>
    </Switch>
  </svg>
);

const MetaRune: Component<{
  icon: HudIconName;
  title: string;
  value: string;
}> = (props) => (
  <article class="meta-rune" title={props.title}>
    <HudIcon icon={props.icon} />
    <strong>{props.value}</strong>
  </article>
);

const RailSeatCounter: Component<{ kind: "life" | "hand"; value: number }> = (props) => (
  <article
    classList={{
      "rail-seat-counter": true,
      [`kind-${props.kind}`]: true,
    }}
    title={props.kind === "life" ? "Life total" : "Cards in hand"}
  >
    <RailCounterGlyph kind={props.kind} />
    <strong>{String(props.value)}</strong>
  </article>
);

const RailCounterGlyph: Component<{ kind: "life" | "hand" }> = (props) => (
  <svg aria-hidden="true" class="rail-counter-glyph" viewBox="0 0 24 24">
    <Switch>
      <Match when={props.kind === "life"}>
        <path d="M12 21.2 4.9 14.4A4.7 4.7 0 0 1 4 8.6C5 6.4 7.8 5.5 10 6.5c.8.4 1.5 1 2 1.8.5-.8 1.2-1.4 2-1.8 2.2-1 5-.1 6 2.1.9 1.9.5 4.2-1 5.8Z" />
      </Match>
      <Match when={props.kind === "hand"}>
        <path d="M7.2 18.5c-1.4 0-2.5-1-2.7-2.4l-.6-4.8c-.1-.7.4-1.3 1.1-1.4.7-.1 1.3.4 1.4 1.1l.2 1.3-.6-4.8c-.1-.7.4-1.3 1.1-1.4.7-.1 1.3.4 1.4 1.1l.6 4.4-.5-4.1c-.1-.7.4-1.3 1.1-1.4.7-.1 1.3.4 1.4 1.1l.5 4-.3-2.5c-.1-.7.4-1.3 1.1-1.4.7-.1 1.3.4 1.4 1.1l.5 4.2c.3 2.6-1.5 5-4.1 5.3l-5 .6Z" />
      </Match>
    </Switch>
  </svg>
);

const SeatAvatarGlyph: Component<{ orientation: "top" | "bottom" }> = (props) => (
  <svg aria-hidden="true" class="seat-avatar-glyph" viewBox="0 0 120 120">
    <defs>
      <radialGradient id={`avatar-core-${props.orientation}`} cx="50%" cy="34%" r="62%">
        <stop offset="0%" stop-color="rgba(255,255,255,0.92)" />
        <stop offset="32%" stop-color={props.orientation === "top" ? "#8dd8ff" : "#ffdd9a"} />
        <stop offset="100%" stop-color={props.orientation === "top" ? "#355d90" : "#7f4b2d"} />
      </radialGradient>
      <linearGradient id={`avatar-ring-${props.orientation}`} x1="0%" x2="100%">
        <stop offset="0%" stop-color={props.orientation === "top" ? "#87beff" : "#ffcf82"} />
        <stop offset="100%" stop-color={props.orientation === "top" ? "#4b79d6" : "#d77f3b"} />
      </linearGradient>
    </defs>
    <circle class="seat-avatar-ring" cx="60" cy="60" r="44" />
    <circle cx="60" cy="58" r="34" fill={`url(#avatar-core-${props.orientation})`} opacity="0.9" />
    <path
      class="seat-avatar-crest"
      d="M60 28c10 0 18 8.6 18 19.2 0 4.7-1.6 8.7-4.8 12.8L67 67.5H53L46.8 60c-3.2-4.1-4.8-8.1-4.8-12.8C42 36.6 50 28 60 28Z"
    />
    <path
      class="seat-avatar-horns"
      d="M47 34 35 28l7 16m31-10 12-6-7 16"
      fill="none"
      stroke={`url(#avatar-ring-${props.orientation})`}
      stroke-linecap="round"
      stroke-width="4"
    />
    <circle class="seat-avatar-eye" cx="52" cy="48" r="3" />
    <circle class="seat-avatar-eye" cx="68" cy="48" r="3" />
    <path
      class="seat-avatar-smile"
      d="M49 58c3.6 3.8 7.3 5.6 11 5.6 3.8 0 7.5-1.8 11-5.6"
      fill="none"
      stroke="rgba(19, 12, 9, 0.72)"
      stroke-linecap="round"
      stroke-width="3.6"
    />
    <circle class="seat-avatar-rune" cx="60" cy="82" r="8" />
  </svg>
);

const StackDock: Component<{ count: number; topObject: ArenaStackObject | undefined }> = (
  props,
) => {
  const previewCard = () => {
    const object = props.topObject;
    if (!object?.definition_id || !object.card_type) {
      return null;
    }

    return {
      cardType: object.card_type,
      definitionId: object.definition_id,
    };
  };

  return (
    <div class="stack-dock-frame">
      <div class="stack-dock-head">
        <div class="stack-dock-rune" aria-hidden="true">
          ✦
        </div>
        <span class="stack-dock-count">{String(props.count)}</span>
      </div>
      <div class="stack-dock-body" aria-hidden="true">
        <span class="stack-dock-layer layer-back" />
        <span class="stack-dock-layer layer-mid" />
        <span class="stack-dock-layer layer-front" />
        <Show when={previewCard()} fallback={<span class="stack-dock-sigil">⟡</span>}>
          {(card) => (
            <div class="stack-dock-card">
              <GameCard cardType={card().cardType} definitionId={card().definitionId} mode="zone" />
            </div>
          )}
        </Show>
      </div>
      <div class="stack-dock-caption">
        <p>Stack</p>
        <strong>{formatStackObjectKind(props.topObject?.kind)}</strong>
      </div>
    </div>
  );
};

const TurnRune: Component<{ turnNumber: number }> = (props) => (
  <article class="turn-rune" title={`Turn ${String(props.turnNumber)}`}>
    <span class="turn-rune-mark">↻</span>
    <strong>{String(props.turnNumber)}</strong>
  </article>
);

const PhaseTrack: Component<{ currentPhase: string }> = (props) => {
  const currentIndex = () => PHASE_NODES.findIndex((node) => node.phase === props.currentPhase);

  return (
    <div class="phase-track" role="list" aria-label="Current phase track">
      <For each={PHASE_NODES}>
        {(node, index) => (
          <div
            classList={{
              "phase-node": true,
              current: node.phase === props.currentPhase,
              complete: currentIndex() >= index(),
            }}
            role="listitem"
            title={node.label}
          >
            <span class="phase-node-glyph" aria-hidden="true">
              {node.glyph}
            </span>
          </div>
        )}
      </For>
    </div>
  );
};

type HudIconName =
  | "battlefield"
  | "hand"
  | "host"
  | "life"
  | "log"
  | "mana"
  | "peer"
  | "reset"
  | "room"
  | "seat"
  | "zones";

const HudIcon: Component<{ icon: HudIconName }> = (props) => (
  <svg aria-hidden="true" class="hud-icon" viewBox="0 0 24 24">
    <Switch fallback={<circle cx="12" cy="12" r="5" />}>
      <Match when={props.icon === "life"}>
        <path d="M12 20.5C7 16.7 4 13.8 4 9.8A3.8 3.8 0 0 1 7.9 6c1.6 0 3.1.8 4.1 2.1A5 5 0 0 1 16.1 6 3.8 3.8 0 0 1 20 9.8c0 4-3 6.9-8 10.7Z" />
      </Match>
      <Match when={props.icon === "hand"}>
        <path d="M7 20c-1.3 0-2.3-1-2.3-2.2v-5.1c0-.7.5-1.2 1.1-1.2.7 0 1.2.5 1.2 1.2V9.4c0-.7.5-1.2 1.1-1.2.7 0 1.2.5 1.2 1.2V7.9c0-.7.5-1.2 1.1-1.2.7 0 1.2.5 1.2 1.2v1c0-.6.5-1.1 1.1-1.1s1.1.5 1.1 1.1v1.2c0-.5.4-.9.9-.9s.9.4.9.9v4.8c0 3-2 5.1-5.1 5.1H7Z" />
      </Match>
      <Match when={props.icon === "mana"}>
        <path d="M12 3.2c3.2 3.7 5 6.6 5 9.1A5 5 0 1 1 7 12.3c0-2.5 1.8-5.4 5-9.1Z" />
      </Match>
      <Match when={props.icon === "room"}>
        <path
          d="M8 7.5a2.5 2.5 0 1 1 0 5 2.5 2.5 0 0 1 0-5Zm8 4a2.5 2.5 0 1 1 0 5 2.5 2.5 0 0 1 0-5ZM10.4 9.8h3.2m-3 6 2.8-1.6"
          fill="none"
          stroke="currentColor"
          stroke-linecap="round"
          stroke-width="2"
        />
      </Match>
      <Match when={props.icon === "host"}>
        <path d="M12 3.5 6.5 7v5.5c0 3.2 2.2 6 5.5 8 3.3-2 5.5-4.8 5.5-8V7L12 3.5Zm0 5.2 1.4 2.9 3.1.5-2.2 2.2.5 3.2-2.8-1.5-2.8 1.5.5-3.2-2.2-2.2 3.1-.5L12 8.7Z" />
      </Match>
      <Match when={props.icon === "peer"}>
        <path d="M8 7.5a2.5 2.5 0 1 1 0 5 2.5 2.5 0 0 1 0-5Zm8 4a2.5 2.5 0 1 1 0 5 2.5 2.5 0 0 1 0-5Z" />
      </Match>
      <Match when={props.icon === "zones"}>
        <path
          d="M7 6.5h9.5a1.5 1.5 0 0 1 1.5 1.5v8.5a1.5 1.5 0 0 1-1.5 1.5H7A1.5 1.5 0 0 1 5.5 16.5V8A1.5 1.5 0 0 1 7 6.5Zm-2-2h9.5"
          fill="none"
          stroke="currentColor"
          stroke-linecap="round"
          stroke-width="2"
        />
      </Match>
      <Match when={props.icon === "log"}>
        <path
          d="M7 7h10M7 12h10M7 17h7"
          fill="none"
          stroke="currentColor"
          stroke-linecap="round"
          stroke-width="2"
        />
      </Match>
      <Match when={props.icon === "reset"}>
        <path
          d="M18 8V4m0 0h-4m4 0-3.2 3.2A7 7 0 1 0 19 12"
          fill="none"
          stroke="currentColor"
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
        />
      </Match>
      <Match when={props.icon === "seat"}>
        <path
          d="M12 4.5a3.4 3.4 0 1 1 0 6.8 3.4 3.4 0 0 1 0-6.8ZM6 18.5c1.1-2.6 3.2-3.9 6-3.9s4.9 1.3 6 3.9"
          fill="none"
          stroke="currentColor"
          stroke-linecap="round"
          stroke-width="2"
        />
      </Match>
      <Match when={props.icon === "battlefield"}>
        <path
          d="m6 17 6-10 6 10M8.4 13h7.2"
          fill="none"
          stroke="currentColor"
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
        />
      </Match>
    </Switch>
  </svg>
);

const PHASE_NODES = [
  { phase: "Upkeep", glyph: "✶", label: "Upkeep" },
  { phase: "Draw", glyph: "↓", label: "Draw" },
  { phase: "FirstMain", glyph: "◈", label: "First Main" },
  { phase: "BeginningOfCombat", glyph: "⚔", label: "Beginning of Combat" },
  { phase: "DeclareAttackers", glyph: "➶", label: "Declare Attackers" },
  { phase: "DeclareBlockers", glyph: "⛨", label: "Declare Blockers" },
  { phase: "CombatDamage", glyph: "✹", label: "Combat Damage" },
  { phase: "EndOfCombat", glyph: "◌", label: "End of Combat" },
  { phase: "SecondMain", glyph: "◈", label: "Second Main" },
  { phase: "EndStep", glyph: "☾", label: "End Step" },
] as const;

const SupportedPrompt: Component<{
  prompt: ArenaChoicePrompt;
  playerId: string;
  hand: ArenaHandCard[];
  variant?: "default" | "private" | "spotlight";
  onRun: (operation: (current: ArenaCommandTarget) => Promise<ArenaState>) => void;
}> = (props) => {
  const variant = () => props.variant ?? "default";

  return (
    <article classList={{ "prompt-item": true, [`prompt-${variant()}`]: true }}>
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

const StackView: Component<{ stack: ArenaStackObject[] }> = (props) => (
  <div class="stack-list">
    <Show when={props.stack.length > 0} fallback={<p class="muted">Stack empty.</p>}>
      <For each={props.stack}>
        {(object) => (
          <article class="stack-item">
            <strong>
              #{object.number} · {formatStackObjectKind(object.kind)}
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

function formatStackObjectKind(kind: string | undefined): string {
  switch (kind) {
    case undefined:
      return "Object";
    case "Spell":
      return "Spell";
    case "ActivatedAbility":
      return "Activated ability";
    case "TriggeredAbility":
      return "Triggered ability";
    case "Unavailable":
      return "Unavailable";
    default:
      return kind;
  }
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

function inspectFromHandCard(card: ArenaHandCard): InspectCardState {
  return {
    sourceCardId: card.card_id,
    definitionId: card.definition_id,
    cardType: card.card_type,
    zoneLabel: "Hand",
    manaCost: card.mana_cost,
    manaCostProfile: card.mana_cost_profile,
    power: card.power,
    toughness: card.toughness,
    loyalty: card.loyalty,
    keywords: card.keywords,
    note:
      card.requires_target || card.requires_choice
        ? "This card still needs a dedicated target or choice flow before battlefield drop-casting."
        : "You can cast or play this card directly from the hand tray, or drag it onto your battlefield when legal.",
  };
}

function inspectFromBattlefieldCard(card: ArenaBattlefieldCard): InspectCardState {
  return {
    sourceCardId: card.card_id,
    definitionId: card.definition_id,
    cardType: card.card_type,
    zoneLabel: "Battlefield",
    manaCostProfile: card.mana_cost,
    power: card.power,
    toughness: card.toughness,
    loyalty: card.loyalty,
    keywords: card.keywords,
    token: card.token,
    tapped: card.tapped,
    summoningSickness: card.summoning_sickness,
    attacking: card.attacking,
    blocking: card.blocking,
    note: card.attached_to ? `Attached to ${card.attached_to}` : undefined,
  };
}

function inspectFromZoneCard(card: ArenaCardView, zoneLabel: string): InspectCardState {
  return {
    definitionId: card.definition_id,
    cardType: card.card_type,
    zoneLabel,
    manaCostProfile: card.mana_cost,
    keywords: [],
  };
}

function topZoneCard(cards: ArenaCardView[]): ArenaCardView | undefined {
  return cards.at(-1);
}

function formatManaCostSummary(
  manaCostProfile: ArenaManaCost | undefined,
  manaCost: number | null | undefined,
): string {
  if (manaCostProfile) {
    const symbols = [
      ...(manaCostProfile.generic > 0 ? [String(manaCostProfile.generic)] : []),
      ...repeatManaLabel("W", manaCostProfile.white),
      ...repeatManaLabel("U", manaCostProfile.blue),
      ...repeatManaLabel("B", manaCostProfile.black),
      ...repeatManaLabel("R", manaCostProfile.red),
      ...repeatManaLabel("G", manaCostProfile.green),
    ];

    if (symbols.length > 0) {
      return symbols.join(" ");
    }
  }

  return String(manaCost ?? 0);
}

function repeatManaLabel(symbol: string, amount: number): string[] {
  return Array.from({ length: amount }, () => symbol);
}

function blockerAssignmentsToArray(assignments: Record<string, string>): BlockerAssignmentInput[] {
  return Object.entries(assignments).map(([blocker_id, attacker_id]) => ({
    blocker_id,
    attacker_id,
  }));
}

function uniqueAttackerIds(action: ArenaLegalAction | undefined): string[] {
  if (!action) {
    return [];
  }

  return Array.from(new Set(action.blocker_options.flatMap((option) => option.attacker_ids)));
}

function seatSupportCopy(): string {
  return "This seat is your local duel surface. Public zones stay live while private information remains scoped to your own instance.";
}

function formatPhase(phase: string): string {
  return phase.replace(/([A-Z])/g, " $1").trim();
}

function formatZoneLabel(zone: ZoneKind): string {
  return zone.charAt(0).toUpperCase() + zone.slice(1);
}

function shortPlayerTag(playerId: string | null | undefined): string {
  if (!playerId) {
    return "n/a";
  }

  const digits = playerId.match(/\d+/g)?.join("");
  if (digits) {
    return `P${digits}`;
  }

  return playerId.length > 6 ? playerId.slice(0, 6) : playerId;
}

function formatPlayerDisplayName(playerId: string): string {
  const digits = playerId.match(/\d+/g)?.join("");
  if (digits) {
    return `Player ${digits}`;
  }

  return playerId.replace(/[-_]+/g, " ").replace(/\b\w/g, (value) => value.toUpperCase());
}

function hiddenHandSlots(count: number): number[] {
  return Array.from({ length: count }, (_, index) => index);
}

function hiddenHandCardShellStyle(index: number, count: number): Record<string, string> {
  const spread = count >= 8 ? 58 : count >= 6 ? 66 : 74;
  const midpoint = (count - 1) / 2;
  const offset = index - midpoint;
  const verticalOffset = -186 + Math.min(Math.abs(offset) * 4, 12);
  const hoverOffset = offset === 0 ? 0 : Math.sign(offset) * 8;

  return {
    "--hidden-hand-x": `${String(offset * spread)}px`,
    "--hidden-hand-hover-shift": `${String(hoverOffset)}px`,
    "--hidden-hand-angle": `${String(offset * -4.25)}deg`,
    "--hidden-hand-y": `${String(verticalOffset)}px`,
    "--hidden-hand-depth": String(index + 1),
  };
}

function defaultBattlefieldPosition(
  index: number,
  count: number,
  orientation: "top" | "bottom",
): BattlefieldLayoutPoint {
  const rowCapacity = 6;
  const row = Math.floor(index / rowCapacity);
  const cardsInRow = Math.min(rowCapacity, count - row * rowCapacity);
  const column = index % rowCapacity;
  const midpoint = (cardsInRow - 1) / 2;
  const horizontalSpread = 0.12;
  const x = clamp(0.5 + (column - midpoint) * horizontalSpread, 0.16, 0.84);
  const verticalBase = orientation === "top" ? 0.34 : 0.66;
  const verticalStep = 0.17;
  const y =
    orientation === "top"
      ? clamp(verticalBase + row * verticalStep, 0.22, 0.78)
      : clamp(verticalBase - row * verticalStep, 0.22, 0.82);

  return { x, y };
}

function battlefieldCardShellStyle(position: BattlefieldLayoutPoint): Record<string, string> {
  return {
    left: `${String(position.x * 100)}%`,
    top: `${String(position.y * 100)}%`,
  };
}

function shortRoomCode(roomId: string): string {
  return roomId
    .replace(/^duel-/, "")
    .slice(0, 6)
    .toUpperCase();
}

function formatSessionRole(role: ArenaSessionInfo["role"]): string {
  return role === "peer" ? "Peer" : "Host";
}

function reorderCardIds(cardIds: string[], draggedCardId: string, targetCardId: string): string[] {
  if (draggedCardId === targetCardId) {
    return cardIds;
  }

  const next = [...cardIds];
  const draggedIndex = next.indexOf(draggedCardId);
  const targetIndex = next.indexOf(targetCardId);

  if (draggedIndex === -1 || targetIndex === -1) {
    return next;
  }

  next.splice(draggedIndex, 1);
  next.splice(targetIndex, 0, draggedCardId);
  return next;
}

function clamp(value: number, min: number, max: number): number {
  return Math.min(Math.max(value, min), max);
}

function handCardShellStyle(
  index: number,
  total: number,
  collapsed: boolean,
  focused: boolean,
  pointerDrag: HandPointerDragState | null,
): Record<string, string> {
  const midpoint = (total - 1) / 2;
  const offset = index - midpoint;
  const spread = total >= 8 ? 58 : total >= 6 ? 66 : 74;
  const x = offset * spread;
  const y = (collapsed ? 154 : 142) + Math.min(Math.abs(offset) * 4, 12);
  const angle = offset * 4.25;
  const depth = focused ? total + 12 : index + 1;
  const focusLift = 0;
  const hoverLift = collapsed ? -18 : -24;

  const style: Record<string, string> = {
    "--hand-x": `${String(x)}px`,
    "--hand-y": `${String(y)}px`,
    "--hand-angle": `${String(angle)}deg`,
    "--hand-depth": String(depth),
    "--hand-focus-y": `${String(focusLift)}px`,
    "--hand-hover-y": `${String(hoverLift)}px`,
  };

  if (pointerDrag?.active) {
    style["--pointer-drag-x"] = `${String(pointerDrag.clientX)}px`;
    style["--pointer-drag-y"] = `${String(pointerDrag.clientY)}px`;
  }

  return style;
}
