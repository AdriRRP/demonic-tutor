import { For, Show } from "solid-js";
import type { Component, JSX } from "solid-js";

interface GameCardProps {
  definitionId: string;
  cardType: string;
  manaCost?: number | null | undefined;
  power?: number | null | undefined;
  toughness?: number | null | undefined;
  loyalty?: number | null | undefined;
  keywords?: string[] | undefined;
  mode: "hand" | "battlefield";
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
  const showsManaCost = () =>
    props.manaCost !== undefined &&
    props.manaCost !== null &&
    !props.cardType.toLowerCase().includes("land");
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
            inspectable: Boolean(props.onInspect),
          }}
          onClick={handleInspect}
        >
          <div class="game-card-title-block">
            <h4>{props.definitionId}</h4>
          </div>
          <Show when={showsManaCost()}>
            <span class="game-card-cost">{props.manaCost}</span>
          </Show>
        </div>

        <div
          classList={{
            "game-card-artbox": true,
            inspectable: Boolean(props.onInspect),
          }}
          onClick={handleInspect}
        >
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
