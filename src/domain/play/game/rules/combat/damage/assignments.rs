use crate::domain::play::{game::model::Player, ids::CardInstanceId};
use std::collections::HashMap;

type BlockAssignment = (CardInstanceId, CardInstanceId);
type CombatAssignments = HashMap<CardInstanceId, Vec<CardInstanceId>>;

pub(super) fn group_assignments_by_attacker(assignments: &[BlockAssignment]) -> CombatAssignments {
    let mut grouped = HashMap::new();

    for (blocker_id, attacker_id) in assignments {
        grouped
            .entry(attacker_id.clone())
            .or_insert_with(Vec::new)
            .push(blocker_id.clone());
    }

    grouped
}

pub(super) fn group_assignments_by_blocker(assignments: &[BlockAssignment]) -> CombatAssignments {
    let mut grouped = HashMap::new();

    for (blocker_id, attacker_id) in assignments {
        grouped
            .entry(blocker_id.clone())
            .or_insert_with(Vec::new)
            .push(attacker_id.clone());
    }

    grouped
}

pub(super) fn blocking_assignments(player: &Player) -> Vec<BlockAssignment> {
    player
        .battlefield()
        .cards()
        .iter()
        .filter_map(|card| {
            card.blocking_target()
                .map(|attacker_id| (card.id().clone(), attacker_id.clone()))
        })
        .collect()
}
