import { For, Match, Show, Switch } from "solid-js";
import type { Component, JSX } from "solid-js";

interface ManaCostProfileLike {
  generic: number;
  white: number;
  blue: number;
  black: number;
  red: number;
  green: number;
}

interface GameCardProps {
  definitionId: string;
  cardType: string;
  manaCost?: number | null | undefined;
  manaCostProfile?: ManaCostProfileLike | undefined;
  power?: number | null | undefined;
  toughness?: number | null | undefined;
  loyalty?: number | null | undefined;
  keywords?: string[] | undefined;
  mode: "hand" | "battlefield" | "zone" | "detail";
  index?: number | undefined;
  fanCount?: number | undefined;
  tapped?: boolean | undefined;
  summoningSickness?: boolean | undefined;
  attacking?: boolean | undefined;
  blocking?: boolean | undefined;
  token?: boolean | undefined;
  openPriority?: boolean | undefined;
  ownTurnPriority?: boolean | undefined;
  activatedAbility?: boolean | undefined;
  actions?: JSX.Element | undefined;
  interactive?: boolean | undefined;
  highlighted?: boolean | undefined;
  selected?: boolean | undefined;
  onClick?:
    | ((event: MouseEvent & { currentTarget: HTMLElement; target: Element }) => void)
    | undefined;
  onInspect?:
    | ((event: MouseEvent & { currentTarget: HTMLElement; target: Element }) => void)
    | undefined;
}

export const GameCard: Component<GameCardProps> = (props) => {
  const keywords = () => props.keywords ?? [];
  const cardTone = () => toneForCardType(props.cardType);
  const inspectable = () => Boolean(props.onInspect);
  const manaCostSymbols = () => expandManaCostSymbols(props.manaCostProfile, props.manaCost);
  const showsManaCost = () =>
    props.mode !== "battlefield" &&
    props.mode !== "zone" &&
    !props.cardType.toLowerCase().includes("land") &&
    manaCostSymbols().length > 0;
  const showsBattlefieldStatBadge = () =>
    (props.mode === "battlefield" || props.mode === "zone") &&
    ((props.power !== undefined &&
      props.power !== null &&
      props.toughness !== undefined &&
      props.toughness !== null) ||
      (props.loyalty !== undefined && props.loyalty !== null));
  const handleInspect = (event: MouseEvent & { currentTarget: HTMLElement; target: Element }) => {
    event.stopPropagation();
    props.onInspect?.(event);
  };
  const frameStyle = () => {
    if (props.mode !== "hand") {
      return undefined;
    }

    const fanCount = props.fanCount ?? 1;
    const index = props.index ?? 0;
    const midpoint = (fanCount - 1) / 2;
    const offset = index - midpoint;

    return {
      transform: `translateY(${String(Math.abs(offset) * 6)}px) rotate(${String(offset * 3.5)}deg)`,
      "transform-origin": "center bottom",
    };
  };

  return (
    <article
      data-card-tone={cardTone()}
      classList={{
        "game-card": true,
        [`tone-${cardTone()}`]: true,
        "mode-hand": props.mode === "hand",
        "mode-battlefield": props.mode === "battlefield",
        "mode-zone": props.mode === "zone",
        "mode-detail": props.mode === "detail",
        interactive: Boolean(props.interactive),
        inspectable: Boolean(props.onInspect),
        highlighted: Boolean(props.highlighted),
        selected: Boolean(props.selected),
        tapped: Boolean(props.tapped),
      }}
      onClick={(event) => {
        props.onClick?.(event);
      }}
      style={frameStyle()}
    >
      <div class="game-card-skin">
        <div
          classList={{
            "game-card-head": true,
          }}
        >
          <Show
            when={inspectable()}
            fallback={
              <div class="game-card-title-block">
                <h4>{props.definitionId}</h4>
              </div>
            }
          >
            <button
              class="game-card-title-button inspectable"
              type="button"
              onClick={handleInspect}
            >
              <div class="game-card-title-block">
                <h4>{props.definitionId}</h4>
              </div>
            </button>
          </Show>
          <Show when={showsManaCost()}>
            <div class="game-card-cost">
              <For each={manaCostSymbols()}>{(symbol) => <ManaCostPip symbol={symbol} />}</For>
            </div>
          </Show>
        </div>

        <div class="game-card-artbox">
          <div class="game-card-art-placeholder" />
          <div class="game-card-statuses">
            <Show when={props.token}>
              <span class="chip">Token</span>
            </Show>
            <Show when={props.tapped && props.mode === "battlefield"}>
              <span class="chip chip-night">Tapped</span>
            </Show>
            <Show when={props.summoningSickness && props.mode === "battlefield"}>
              <span class="chip">Summoning sick</span>
            </Show>
            <Show when={props.attacking && props.mode === "battlefield"}>
              <span class="chip chip-ember">Attacking</span>
            </Show>
            <Show when={props.blocking && props.mode === "battlefield"}>
              <span class="chip chip-forest">Blocking</span>
            </Show>
            <Show when={props.openPriority && props.mode === "hand"}>
              <span class="chip chip-night">Open priority</span>
            </Show>
            <Show when={props.ownTurnPriority && props.mode === "hand"}>
              <span class="chip chip-night">Own-turn priority</span>
            </Show>
            <Show when={props.activatedAbility}>
              <span class="chip chip-forest">Ability</span>
            </Show>
          </div>
          <Show when={showsBattlefieldStatBadge()}>
            <div class="game-card-battlefield-badge">
              <Show
                when={
                  props.power !== undefined &&
                  props.power !== null &&
                  props.toughness !== undefined &&
                  props.toughness !== null
                }
                fallback={<strong>{props.loyalty}</strong>}
              >
                <strong>
                  {props.power}/{props.toughness}
                </strong>
              </Show>
            </div>
          </Show>
        </div>

        <div class="game-card-type-line">
          <p class="game-card-type-bar">{formatCardTypeLine(props.cardType)}</p>
        </div>

        <div class="game-card-textbox">
          <div class="game-card-body">
            <Show when={keywords().length > 0}>
              <div class="game-card-keywords">
                <For each={keywords()}>
                  {(keyword) => <span class="game-keyword">{keyword}</span>}
                </For>
              </div>
            </Show>
            <Show when={props.mode === "battlefield"}>
              <p class="game-card-line">{props.cardType}</p>
            </Show>
          </div>

          <div class="game-card-footer">
            <Show when={props.loyalty !== undefined && props.loyalty !== null}>
              <div class="card-stat-block">
                <span>Loyalty</span>
                <strong>{props.loyalty}</strong>
              </div>
            </Show>
            <Show
              when={
                props.power !== undefined &&
                props.power !== null &&
                props.toughness !== undefined &&
                props.toughness !== null
              }
            >
              <div class="card-stat-block combat">
                <span>Power / Toughness</span>
                <strong>
                  {props.power}/{props.toughness}
                </strong>
              </div>
            </Show>
          </div>
        </div>

        <Show when={props.actions}>
          <div
            class="game-card-actions"
            onClick={(event) => {
              event.stopPropagation();
            }}
          >
            {props.actions}
          </div>
        </Show>
      </div>
    </article>
  );
};

type ManaCostSymbol =
  | { kind: "generic"; value: number }
  | { kind: "white" | "blue" | "black" | "red" | "green" };

const ManaCostPip: Component<{ symbol: ManaCostSymbol }> = (props) => {
  if (props.symbol.kind === "generic") {
    return (
      <span class="game-card-cost-pip kind-generic numeric">
        <span>{props.symbol.value}</span>
      </span>
    );
  }

  return (
    <span classList={{ "game-card-cost-pip": true, [`kind-${props.symbol.kind}`]: true }}>
      <ManaSymbolGlyph kind={props.symbol.kind} />
    </span>
  );
};

const ManaSymbolGlyph: Component<{ kind: Exclude<ManaCostSymbol["kind"], "generic"> }> = (
  props,
) => (
  <svg aria-hidden="true" class="game-card-cost-glyph" viewBox="0 0 24 24">
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
    </Switch>
  </svg>
);

function expandManaCostSymbols(
  manaCostProfile: ManaCostProfileLike | undefined,
  manaCost: number | null | undefined,
): ManaCostSymbol[] {
  if (manaCostProfile) {
    return [
      ...repeatManaSymbol("generic", manaCostProfile.generic),
      ...repeatManaSymbol("white", manaCostProfile.white),
      ...repeatManaSymbol("blue", manaCostProfile.blue),
      ...repeatManaSymbol("black", manaCostProfile.black),
      ...repeatManaSymbol("red", manaCostProfile.red),
      ...repeatManaSymbol("green", manaCostProfile.green),
    ];
  }

  if (!manaCost) {
    return [];
  }

  return [{ kind: "generic", value: manaCost }];
}

function repeatManaSymbol(kind: ManaCostSymbol["kind"], amount: number): ManaCostSymbol[] {
  if (amount <= 0) {
    return [];
  }

  if (kind === "generic") {
    return [{ kind: "generic", value: amount }];
  }

  return Array.from({ length: amount }, () => ({ kind }));
}

function toneForCardType(cardType: string): string {
  const normalized = cardType.toLowerCase();

  if (normalized.includes("land")) {
    return "land";
  }
  if (normalized.includes("artifact")) {
    return "artifact";
  }
  if (normalized.includes("enchantment")) {
    return "enchantment";
  }
  if (normalized.includes("planeswalker")) {
    return "planeswalker";
  }
  if (normalized.includes("instant") || normalized.includes("sorcery")) {
    return "spell";
  }

  return "creature";
}

function formatCardTypeLine(cardType: string): string {
  return cardType.replace(/([a-z])([A-Z])/g, "$1 $2");
}
