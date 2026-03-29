import { For, Show } from "solid-js";
import type { Component, JSX } from "solid-js";

interface GameCardProps {
  definitionId: string;
  cardType: string;
  manaCost?: number | null;
  power?: number | null;
  toughness?: number | null;
  loyalty?: number | null;
  keywords?: string[];
  mode: "hand" | "battlefield";
  index?: number;
  fanCount?: number;
  tapped?: boolean;
  summoningSickness?: boolean;
  attacking?: boolean;
  blocking?: boolean;
  token?: boolean;
  openPriority?: boolean;
  ownTurnPriority?: boolean;
  activatedAbility?: boolean;
  actions?: JSX.Element;
}

export const GameCard: Component<GameCardProps> = (props) => {
  const keywords = () => props.keywords ?? [];
  const cardTone = () => toneForCardType(props.cardType);
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
      classList={{
        "game-card": true,
        [`tone-${cardTone()}`]: true,
        "mode-hand": props.mode === "hand",
        "mode-battlefield": props.mode === "battlefield",
        tapped: Boolean(props.tapped),
      }}
      style={frameStyle()}
    >
      <div class="game-card-skin">
        <div class="game-card-crest" />
        <div class="game-card-head">
          <div>
            <p class="game-card-type">{labelForCardType(props.cardType)}</p>
            <h4>{props.definitionId}</h4>
          </div>
          <Show when={props.manaCost !== undefined && props.manaCost !== null}>
            <span class="game-card-cost">{props.manaCost}</span>
          </Show>
        </div>

        <div class="game-card-statuses">
          <Show when={props.token}>
            <span class="chip">Token</span>
          </Show>
          <Show when={props.tapped}>
            <span class="chip chip-night">Tapped</span>
          </Show>
          <Show when={props.summoningSickness}>
            <span class="chip">Summoning sick</span>
          </Show>
          <Show when={props.attacking}>
            <span class="chip chip-ember">Attacking</span>
          </Show>
          <Show when={props.blocking}>
            <span class="chip chip-forest">Blocking</span>
          </Show>
          <Show when={props.openPriority}>
            <span class="chip chip-night">Open priority</span>
          </Show>
          <Show when={props.ownTurnPriority}>
            <span class="chip chip-night">Own-turn priority</span>
          </Show>
          <Show when={props.activatedAbility}>
            <span class="chip chip-forest">Ability</span>
          </Show>
        </div>

        <div class="game-card-body">
          <Show when={keywords().length > 0}>
            <div class="game-card-keywords">
              <For each={keywords()}>
                {(keyword) => <span class="game-keyword">{keyword}</span>}
              </For>
            </div>
          </Show>
          <p class="game-card-line">{props.cardType}</p>
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

        <Show when={props.actions}>
          <div class="game-card-actions">{props.actions}</div>
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

function labelForCardType(cardType: string): string {
  const normalized = cardType.toLowerCase();

  if (normalized.includes("planeswalker")) {
    return "Planeswalker";
  }
  if (normalized.includes("artifact")) {
    return "Artifact";
  }
  if (normalized.includes("enchantment")) {
    return "Enchantment";
  }
  if (normalized.includes("instant") || normalized.includes("sorcery")) {
    return "Spell";
  }
  if (normalized.includes("land")) {
    return "Land";
  }

  return "Creature";
}
