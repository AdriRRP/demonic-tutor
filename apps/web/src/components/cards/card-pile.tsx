import type { Component } from "solid-js";
import type { ArenaCardView } from "../../lib/types";
import { GameCard } from "./game-card";
import { CardBack } from "./card-back";

interface CardPileProps {
  kind: "library" | "graveyard" | "exile";
  count: number;
  topCard?: ArenaCardView | undefined;
  highlight?: boolean | undefined;
  onClick?: () => void;
}

export const CardPile: Component<CardPileProps> = (props) => {
  const title = () => `${formatPileLabel(props.kind)} · ${String(props.count)}`;
  const showsEmptyState = () =>
    props.kind !== "library" && props.count === 0 && props.topCard === undefined;
  const kindGlyph = () => {
    switch (props.kind) {
      case "library":
        return "◫";
      case "graveyard":
        return "✢";
      case "exile":
        return "✧";
    }
  };

  return (
    <button
      classList={{
        "card-pile": true,
        [`kind-${props.kind}`]: true,
        highlight: Boolean(props.highlight),
      }}
      title={title()}
      type="button"
      onClick={() => {
        props.onClick?.();
      }}
    >
      <div class="card-pile-stack">
        <div class="card-pile-visual">
          <div class="card-pile-layer layer-back" />
          <div class="card-pile-layer layer-mid" />
          <div class="card-pile-layer layer-front" />

          <div class="card-pile-card">
            {props.kind === "library" ? (
              <CardBack variant="library" />
            ) : showsEmptyState() ? (
              <div classList={{ "card-pile-empty": true, [`kind-${props.kind}`]: true }}>
                <div class="card-pile-empty-frame">
                  <div class="card-pile-empty-sigil" />
                </div>
              </div>
            ) : props.topCard ? (
              <GameCard
                cardType={props.topCard.card_type}
                definitionId={props.topCard.definition_id}
                mode="zone"
              />
            ) : (
              <CardBack variant={props.kind} />
            )}
          </div>
        </div>

        <div class="card-pile-counter">
          <span class="card-pile-counter-glyph" aria-hidden="true">
            {kindGlyph()}
          </span>
          <strong>{String(props.count)}</strong>
        </div>
      </div>
    </button>
  );
};

function formatPileLabel(kind: CardPileProps["kind"]): string {
  switch (kind) {
    case "library":
      return "Library";
    case "graveyard":
      return "Graveyard";
    case "exile":
      return "Exile";
  }
}
