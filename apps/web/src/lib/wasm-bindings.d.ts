import type { ArenaState, BlockerAssignmentInput } from "./types";

declare module "../wasm/pkg/demonictutor_ui.js" {
  export default function init(): Promise<void>;

  export class WebArenaClient {
    constructor();

    state(): ArenaState;
    reset(): ArenaState;
    pass_priority(playerId: string): ArenaState;
    advance_turn(): ArenaState;
    concede(playerId: string): ArenaState;
    play_land(playerId: string, cardId: string): ArenaState;
    tap_mana_source(playerId: string, cardId: string): ArenaState;
    cast_spell(playerId: string, cardId: string): ArenaState;
    activate_ability(playerId: string, cardId: string): ArenaState;
    declare_attackers(playerId: string, attackerIds: string[]): ArenaState;
    declare_blockers(playerId: string, assignments: BlockerAssignmentInput[]): ArenaState;
    resolve_combat_damage(playerId: string): ArenaState;
    discard_for_cleanup(playerId: string, cardId: string): ArenaState;
    resolve_optional_effect(playerId: string, accept: boolean): ArenaState;
    resolve_pending_hand_choice(playerId: string, cardId: string): ArenaState;
    resolve_pending_scry(playerId: string, moveToBottom: boolean): ArenaState;
    resolve_pending_surveil(playerId: string, moveToGraveyard: boolean): ArenaState;
  }
}
