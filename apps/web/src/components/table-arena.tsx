import { For, Match, Show, Switch, createEffect, createSignal } from "solid-js";
import type { Component } from "solid-js";
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
  revealedSeatId: string | null;
  pendingHandoffPlayerId: string | null;
  selectedAttackers: string[];
  blockerAssignments: Record<string, string>;
  onToggleSeatPrivacy: (playerId: string) => void;
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

export const TableArena: Component<TableArenaProps> = (props) => {
  const liveViewer = () =>
    props.state.viewers.find((viewer) => viewer.player_id === props.revealedSeatId) ??
    props.state.viewers.find((viewer) => viewer.is_priority_holder) ??
    props.state.viewers.find((viewer) => viewer.is_active) ??
    props.state.viewers[0];
  const bottomViewer = () => liveViewer();
  const topViewer = () =>
    props.state.viewers.find((viewer) => viewer.player_id !== bottomViewer()?.player_id) ??
    bottomViewer();
  const [handTrayOpen, setHandTrayOpen] = createSignal(true);
  const [zonesOpen, setZonesOpen] = createSignal(true);
  const [sidebarOpen, setSidebarOpen] = createSignal(false);
  const [zoneBrowser, setZoneBrowser] = createSignal<ZoneBrowserState>(null);
  const [inspectedCard, setInspectedCard] = createSignal<InspectCardState | null>(null);
  const [draggedCardId, setDraggedCardId] = createSignal<string | null>(null);
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
          <div class="arena-seat-signals">
            <SeatStateGlyph
              icon="active"
              playerId={props.state.game.active_player_id}
              title="Active player"
              tone="ember"
            />
            <SeatStateGlyph
              icon="priority"
              playerId={props.state.game.priority_holder}
              title="Priority holder"
              tone="night"
            />
          </div>
        </div>

        <div class="arena-cockpit-actions">
          <Show when={liveViewer()}>
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
            aria-label="Toggle hand tray"
            class="hero-button hero-button-ghost mini-button rune-button"
            title="Toggle hand tray"
            onClick={() => {
              setHandTrayOpen((open) => !open);
            }}
          >
            <HudIcon icon="hand" />
          </button>
          <button
            aria-label="Toggle zone rail"
            class="hero-button hero-button-ghost mini-button rune-button"
            title="Toggle zone rail"
            onClick={() => {
              setZonesOpen((open) => !open);
            }}
          >
            <HudIcon icon="zones" />
          </button>
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
            "handoff-pending": Boolean(props.pendingHandoffPlayerId),
          }}
        >
          <Show when={props.state.game.stack.length > 0}>
            <aside class="table-stack-overlay">
              <div class="table-stack-well">
                <div class="table-well-head">
                  <div>
                    <p class="label">Stack</p>
                    <h2>Resolve lane</h2>
                  </div>
                  <span class="chip chip-night">
                    {`${String(props.state.game.stack.length)} objects`}
                  </span>
                </div>
                <StackView stack={props.state.game.stack} />
              </div>
            </aside>
          </Show>

          <Show when={topViewer()}>
            {(viewer) => (
              <SeatPanel
                blockerAssignments={props.blockerAssignments}
                game={props.state.game}
                needsHandoff={props.pendingHandoffPlayerId === viewer().player_id}
                onRun={props.onRun}
                onSetBlockerAssignment={props.onSetBlockerAssignment}
                onToggleAttackerSelection={props.onToggleAttackerSelection}
                onToggleSeatPrivacy={props.onToggleSeatPrivacy}
                onToggleZones={() => {
                  setZonesOpen((open) => !open);
                }}
                onInspectCard={setInspectedCard}
                onOpenZoneBrowser={setZoneBrowser}
                onDragHandCard={setDraggedCardId}
                inspectedCardId={inspectedCard()?.sourceCardId ?? null}
                orientation="top"
                revealed={props.state.game.is_over || props.revealedSeatId === viewer().player_id}
                selectedAttackers={props.selectedAttackers}
                viewer={viewer()}
                draggedHandCardId={draggedCardId()}
                zonesOpen={zonesOpen()}
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
                needsHandoff={props.pendingHandoffPlayerId === viewer().player_id}
                onRun={props.onRun}
                onSetBlockerAssignment={props.onSetBlockerAssignment}
                onToggleAttackerSelection={props.onToggleAttackerSelection}
                onToggleSeatPrivacy={props.onToggleSeatPrivacy}
                onToggleHandTray={() => {
                  setHandTrayOpen((open) => !open);
                }}
                onToggleZones={() => {
                  setZonesOpen((open) => !open);
                }}
                onInspectCard={setInspectedCard}
                onOpenZoneBrowser={setZoneBrowser}
                onDragHandCard={setDraggedCardId}
                onBattlefieldDropCard={(cardId) => {
                  playDraggedCard(viewer(), cardId);
                }}
                inspectedCardId={inspectedCard()?.sourceCardId ?? null}
                orientation="bottom"
                revealed={props.state.game.is_over || props.revealedSeatId === viewer().player_id}
                selectedAttackers={props.selectedAttackers}
                viewer={viewer()}
                handTrayOpen={handTrayOpen()}
                draggedHandCardId={draggedCardId()}
                zonesOpen={zonesOpen()}
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
              <SidebarMetric
                label="Priority"
                value={shortPlayerTag(props.state.game.priority_holder)}
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
                    mode="battlefield"
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
                      card().manaCost !== undefined &&
                      card().manaCost !== null &&
                      !card().cardType.toLowerCase().includes("land")
                    }
                  >
                    <SidebarMetric label="Mana" value={String(card().manaCost)} />
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
  revealed: boolean;
  needsHandoff: boolean;
  handTrayOpen?: boolean;
  selectedAttackers: string[];
  blockerAssignments: Record<string, string>;
  onToggleSeatPrivacy: (playerId: string) => void;
  onToggleAttackerSelection: (cardId: string) => void;
  onSetBlockerAssignment: (blockerId: string, attackerId: string) => void;
  onToggleHandTray?: () => void;
  onToggleZones: () => void;
  onOpenZoneBrowser: (state: ZoneBrowserState) => void;
  onInspectCard: (card: InspectCardState | null) => void;
  onDragHandCard: (cardId: string | null) => void;
  onBattlefieldDropCard?: (cardId: string) => void;
  onRun: (operation: (current: ArenaCommandTarget) => Promise<ArenaState>) => void;
  zonesOpen: boolean;
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
  const passPriorityAction = () => findAction(props.viewer, "PassPriority");
  const concedeAction = () => findAction(props.viewer, "Concede");
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
  const [battlefieldActionMenuCardId, setBattlefieldActionMenuCardId] = createSignal<string | null>(
    null,
  );
  let battlefieldSurfaceRef: HTMLDivElement | undefined;

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

  createEffect(() => {
    if (props.orientation !== "bottom") {
      return;
    }

    if (!props.handTrayOpen || !props.revealed) {
      setFocusedHandCardId(null);
    }
  });

  const moveHandCard = (draggedCardId: string, targetCardId: string) => {
    setHandOrder((previous) => reorderCardIds(previous, draggedCardId, targetCardId));
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

  const canDropDraggedCard = () =>
    props.orientation === "bottom" &&
    Boolean(props.draggedHandCardId) &&
    props.revealed &&
    Boolean(
      props.viewer.hand.find((card) => card.card_id === props.draggedHandCardId) &&
      (playLandIds().has(props.draggedHandCardId ?? "") ||
        (castSpellIds().has(props.draggedHandCardId ?? "") &&
          !props.viewer.hand.find((card) => card.card_id === props.draggedHandCardId)
            ?.requires_choice &&
          !props.viewer.hand.find((card) => card.card_id === props.draggedHandCardId)
            ?.requires_target)),
    );

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
        active: props.viewer.is_active,
        priority: props.viewer.is_priority_holder,
        "has-spotlight": spotlightPrompts().length > 0 || unsupportedPrompts().length > 0,
        "tray-open": props.orientation === "bottom" && Boolean(props.handTrayOpen),
        "tray-closed": props.orientation === "bottom" && !props.handTrayOpen,
      }}
    >
      <header class="seat-banner">
        <div class="seat-identity">
          <div class="seat-banner-copy">
            <p class="seat-role">{props.orientation === "top" ? "Opponent" : "You"}</p>
            <p class="seat-title">{props.viewer.player_id}</p>
          </div>
          <Show when={viewerPlayer()}>
            {(player) => (
              <div class="seat-summary">
                <SeatStatPill icon="life" title="Life total" value={String(player().life)} />
                <SeatStatPill
                  icon="hand"
                  title="Cards in hand"
                  value={String(player().hand_count)}
                />
                <SeatStatPill icon="mana" title="Mana pool" value={String(player().mana_total)} />
              </div>
            )}
          </Show>
        </div>
        <div class="seat-sigil-row">
          <Show when={props.viewer.is_active}>
            <SeatStateGlyph icon="active" title="Active player" tone="ember" />
          </Show>
          <Show when={props.viewer.is_priority_holder}>
            <SeatStateGlyph icon="priority" title="Priority holder" tone="night" />
          </Show>
          <Show when={props.needsHandoff}>
            <SeatStateGlyph icon="seat" title="Ready to take seat" tone="forest" />
          </Show>
        </div>
      </header>

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
          <div class="seat-zones">
            <section
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
                const cardId = props.draggedHandCardId;
                if (cardId) {
                  props.onBattlefieldDropCard?.(cardId);
                }
                props.onDragHandCard(null);
              }}
            >
              <div class="zone-head">
                <div class="battlefield-headline">
                  <HudIcon icon="battlefield" />
                  <strong>{String(player().battlefield.length)}</strong>
                </div>
                <div class="chip-row">
                  <button
                    aria-label={props.zonesOpen ? "Hide zones" : "Show zones"}
                    class="chip chip-toggle zone-toggle-chip"
                    title={props.zonesOpen ? "Hide zones" : "Show zones"}
                    onClick={() => {
                      props.onToggleZones();
                    }}
                  >
                    <HudIcon icon="zones" />
                  </button>
                </div>
              </div>

              <Show when={props.orientation === "bottom"}>
                <div class="lane-quick-actions">
                  <Show when={passPriorityAction()}>
                    <button
                      class="action-button action-button-primary"
                      onClick={() => {
                        const playerId = props.viewer.player_id;
                        props.onRun((current) => passPriority(current, playerId));
                      }}
                    >
                      Pass
                    </button>
                  </Show>
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
                  <button
                    class="action-button"
                    onClick={() => {
                      props.onToggleHandTray?.();
                    }}
                  >
                    {props.handTrayOpen ? "Hide hand" : "Show hand"}
                  </button>
                  <button
                    class="action-button"
                    onClick={() => {
                      props.onToggleSeatPrivacy(props.viewer.player_id);
                    }}
                  >
                    {props.revealed ? "Mask" : props.needsHandoff ? "Take seat" : "Reveal"}
                  </button>
                  <Show when={concedeAction()}>
                    <button
                      class="action-button"
                      onClick={() => {
                        const playerId = props.viewer.player_id;
                        props.onRun((current) => concede(current, playerId));
                      }}
                    >
                      Concede
                    </button>
                  </Show>
                </div>
              </Show>

              <div
                class="battlefield-surface"
                ref={battlefieldSurfaceRef}
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

                  const draggedHandCardId = props.draggedHandCardId;
                  if (draggedHandCardId) {
                    props.onBattlefieldDropCard?.(draggedHandCardId);
                    props.onDragHandCard(null);
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

            <aside classList={{ "zone-rail": true, collapsed: !props.zonesOpen }}>
              <div class="zone-rail-content">
                <CardPile
                  count={player().library_count}
                  highlight={props.viewer.is_active}
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
              </div>
            </aside>
          </div>
        )}
      </Show>

      <Show when={showPrivateTray()}>
        <section class="seat-ops">
          <section
            classList={{
              "hand-drawer": true,
              collapsed: !props.handTrayOpen,
            }}
          >
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

              <Show
                when={props.revealed}
                fallback={
                  <div class="seat-privacy-guard">
                    <p class="muted">
                      {props.needsHandoff
                        ? "Take the seat to reveal this hand."
                        : "Hand hidden. Reveal it when you want to play from hand."}
                    </p>
                    <button
                      class="hero-button"
                      onClick={() => {
                        props.onToggleSeatPrivacy(props.viewer.player_id);
                      }}
                    >
                      {props.needsHandoff ? "Take seat" : "Reveal hand"}
                    </button>
                  </div>
                }
              >
                <div
                  class="hand-fan hand-fan-floating"
                  onMouseLeave={() => {
                    setFocusedHandCardId(null);
                  }}
                >
                  <For each={orderedHand()}>
                    {(card, index) => (
                      <div
                        classList={{
                          "card-drag-shell": true,
                          "hand-card-shell": true,
                          focused: focusedHandCardId() === card.card_id,
                          selected: props.inspectedCardId === card.card_id,
                        }}
                        draggable={props.revealed}
                        onDragOver={(event) => {
                          if (!props.revealed || !props.draggedHandCardId) {
                            return;
                          }

                          event.preventDefault();
                          if (event.dataTransfer) {
                            event.dataTransfer.dropEffect = "move";
                          }
                        }}
                        onDragEnd={() => {
                          props.onDragHandCard(null);
                        }}
                        onDrop={(event) => {
                          if (!props.draggedHandCardId) {
                            return;
                          }

                          event.preventDefault();
                          moveHandCard(props.draggedHandCardId, card.card_id);
                          props.onDragHandCard(null);
                        }}
                        onDragStart={(event) => {
                          props.onDragHandCard(card.card_id);
                          if (event.dataTransfer) {
                            event.dataTransfer.setData("text/plain", card.card_id);
                            event.dataTransfer.effectAllowed = "move";
                          }
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
                          !props.handTrayOpen,
                          focusedHandCardId() === card.card_id,
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
              </Show>

              <Show when={unsupportedPrompts().length > 0}>
                <details class="unsupported-drawer">
                  <summary class="debug-drawer-summary seat-drawer-summary">
                    <div>
                      <p class="label">Seat read</p>
                      <h3>Tactical notes</h3>
                    </div>
                    <span class="chip chip-night">debug</span>
                  </summary>

                  <p class="support-note">{seatSupportCopy(props.viewer, props.needsHandoff)}</p>

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

const SeatStatPill: Component<{
  icon: HudIconName;
  title: string;
  value: string;
}> = (props) => (
  <article class="seat-stat-pill" title={props.title}>
    <HudIcon icon={props.icon} />
    <strong>{props.value}</strong>
  </article>
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

const TurnRune: Component<{ turnNumber: number }> = (props) => (
  <article class="turn-rune" title={`Turn ${String(props.turnNumber)}`}>
    <span class="turn-rune-mark">↻</span>
    <strong>{String(props.turnNumber)}</strong>
  </article>
);

const SeatStateGlyph: Component<{
  icon: "active" | "priority" | "seat";
  playerId?: string | null | undefined;
  title: string;
  tone: "ember" | "forest" | "night";
}> = (props) => (
  <article
    classList={{ "seat-state-glyph": true, [`tone-${props.tone}`]: true }}
    title={`${props.title}${props.playerId ? ` · ${shortPlayerTag(props.playerId)}` : ""}`}
  >
    <HudIcon icon={props.icon} />
    <Show when={props.playerId}>
      <strong>{shortPlayerTag(props.playerId)}</strong>
    </Show>
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
  | "active"
  | "battlefield"
  | "hand"
  | "host"
  | "life"
  | "log"
  | "mana"
  | "peer"
  | "priority"
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
      <Match when={props.icon === "active"}>
        <path d="m12 4 1.9 4 4.4.5-3.3 3 1 4.4-4-2.2-4 2.2 1-4.4-3.3-3 4.4-.5L12 4Z" />
      </Match>
      <Match when={props.icon === "priority"}>
        <path d="M12 3.5 14 9l5.5 1-4 3.4 1 5.1-4.5-2.8-4.5 2.8 1-5.1-4-3.4L10 9l2-5.5Z" />
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

function inspectFromHandCard(card: ArenaHandCard): InspectCardState {
  return {
    sourceCardId: card.card_id,
    definitionId: card.definition_id,
    cardType: card.card_type,
    zoneLabel: "Hand",
    manaCost: card.mana_cost,
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
    keywords: [],
  };
}

function topZoneCard(cards: ArenaCardView[]): ArenaCardView | undefined {
  return cards.at(-1);
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

function seatSupportCopy(viewer: ArenaViewerState, needsHandoff: boolean): string {
  if (needsHandoff) {
    return "This seat is up next. Pass the device, reveal the hand, and continue from the live viewer-scoped surface.";
  }

  if (viewer.is_priority_holder) {
    return "Priority lives here right now. The hand tray and battlefield console are the fastest paths to keep the duel flowing.";
  }

  if (viewer.is_active) {
    return "This seat owns the turn, even if interaction is paused somewhere else in the priority loop.";
  }

  return "This seat is currently observing. Public zones stay live, but private information remains shielded until the hand is reopened.";
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

function hiddenHandSlots(count: number): number[] {
  return Array.from({ length: count }, (_, index) => index);
}

function hiddenHandCardShellStyle(index: number, count: number): Record<string, string> {
  const spread = Math.min(26, Math.max(14, 140 / Math.max(count, 2)));
  const midpoint = (count - 1) / 2;
  const offset = index - midpoint;
  const verticalOffset = Math.abs(offset) * 4;

  return {
    "--hidden-hand-x": `${String(offset * spread)}px`,
    "--hidden-hand-angle": `${String(offset * 4.2)}deg`,
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

  return {
    "--hand-x": `${String(x)}px`,
    "--hand-y": `${String(y)}px`,
    "--hand-angle": `${String(angle)}deg`,
    "--hand-depth": String(depth),
    "--hand-focus-y": `${String(focusLift)}px`,
    "--hand-hover-y": `${String(hoverLift)}px`,
  };
}
