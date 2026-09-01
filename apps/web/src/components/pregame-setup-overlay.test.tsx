import { cleanup, fireEvent, render, screen } from "@solidjs/testing-library";
import { afterEach, describe, expect, it, vi } from "vitest";
import { PregameSetupOverlay } from "./pregame-setup-overlay";
import type { ArenaCommandTarget } from "../lib/runtime";
import type { ArenaSessionInfo } from "../lib/session";
import type { ArenaState } from "../lib/types";

const sessionInfo: ArenaSessionInfo = {
  inviteUrl: "https://example.test/?duel=room",
  localSeatId: "player-1",
  role: "host",
  roomId: "room",
  transport: "embedded",
};

function stateWithBottomCount(currentBottomCount: number): ArenaState {
  return {
    game: {
      active_player_id: "player-1",
      end_reason: null,
      game_id: "test-game",
      is_over: false,
      loser_id: null,
      phase: "Setup",
      playable_subset_version: "v1",
      players: [],
      priority_has_pending_pass: null,
      priority_holder: null,
      stack: [],
      turn_number: 1,
      winner_id: null,
    },
    pregame: {
      current_bottom_count: currentBottomCount,
      current_decision_player_id: "player-1",
      kept_player_ids: [],
      starting_player_id: "player-1",
    },
    event_log: [],
    last_command: null,
    viewers: [viewer("player-1", 1), viewer("player-2", 0)],
  };
}

function viewer(playerId: string, mulliganCount: number) {
  return {
    choice_requests: [],
    hand: Array.from({ length: 7 }, (_, index) => ({
      can_cast_in_open_priority: false,
      can_cast_in_open_priority_during_own_turn: false,
      card_id: `${playerId}-card-${String(index)}`,
      card_type: "Land",
      definition_id: "forest",
      has_activated_ability: false,
      keywords: [],
      loyalty: null,
      mana_cost: 0,
      mana_cost_profile: { black: 0, blue: 0, generic: 0, green: 0, red: 0, white: 0 },
      power: null,
      requires_choice: false,
      requires_target: false,
      toughness: null,
    })),
    is_active: playerId === "player-1",
    is_priority_holder: false,
    legal_actions: [],
    mulligan_count: mulliganCount,
    mulligan_used: mulliganCount > 0,
    player_id: playerId,
  };
}

describe("PregameSetupOverlay", () => {
  afterEach(cleanup);

  it("blocks Keep until the required bottom selection is complete", () => {
    render(() => (
      <PregameSetupOverlay
        onRun={vi.fn()}
        selectedBottomCardIds={[]}
        sessionInfo={sessionInfo}
        state={stateWithBottomCount(1)}
      />
    ));

    expect(screen.getByText("Choose 1 card to put back")).not.toBeNull();
    expect(screen.getByRole("button", { name: /Keep 6/i }).getAttribute("disabled")).toBe("");
  });

  it("relays the selected bottom cards through Keep", async () => {
    const onRun = vi.fn();
    const state = stateWithBottomCount(1);
    render(() => (
      <PregameSetupOverlay
        onRun={onRun}
        selectedBottomCardIds={["player-1-card-0"]}
        sessionInfo={sessionInfo}
        state={state}
      />
    ));

    fireEvent.click(screen.getByRole("button", { name: /Keep 6/i }));
    const operation = onRun.mock.calls[0]?.[0] as (
      target: ArenaCommandTarget,
    ) => Promise<ArenaState>;
    const keepOpeningHand = vi.fn(() => state);

    await operation({ keep_opening_hand: keepOpeningHand } as unknown as ArenaCommandTarget);

    expect(keepOpeningHand).toHaveBeenCalledWith("player-1", ["player-1-card-0"]);
  });
});
