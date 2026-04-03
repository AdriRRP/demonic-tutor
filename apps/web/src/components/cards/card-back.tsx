import type { Component } from "solid-js";

interface CardBackProps {
  variant?: "library" | "graveyard" | "exile" | "hidden-hand";
}

export const CardBack: Component<CardBackProps> = (props) => {
  const variant = () => props.variant ?? "library";

  return (
    <div classList={{ "card-back": true, [`variant-${variant()}`]: true }}>
      <div class="card-back-frame">
        <div class="card-back-surface">
          <div class="card-back-corner card-back-corner-tl" />
          <div class="card-back-corner card-back-corner-tr" />
          <div class="card-back-corner card-back-corner-bl" />
          <div class="card-back-corner card-back-corner-br" />

          <div class="card-back-oval">
            <div class="card-back-gems">
              <span />
              <span />
              <span />
              <span />
              <span />
            </div>

            <div class="card-back-plaque" />
          </div>
        </div>
      </div>
    </div>
  );
};
