import type { Component } from "solid-js";

interface CardBackProps {
  variant?: "library" | "graveyard" | "exile" | "hidden-hand";
}

export const CardBack: Component<CardBackProps> = (props) => {
  const variant = () => props.variant ?? "library";

  return (
    <div classList={{ "card-back": true, [`variant-${variant()}`]: true }}>
      <div class="card-back-frame">
        <div class="card-back-oval">
          <div class="card-back-sigil">
            <span class="card-back-sigil-ring" />
            <span class="card-back-sigil-core" />
          </div>
          <div class="card-back-gems">
            <span />
            <span />
            <span />
            <span />
            <span />
          </div>
        </div>
      </div>
    </div>
  );
};
